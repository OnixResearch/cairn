#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeltaMerge {
    pub markdown: String,
    pub added_blocks: usize,
    pub removed_blocks: usize,
}

pub fn merge_delta_spec(accepted: Option<&str>, delta: &str, spec_id: &str) -> DeltaMerge {
    // r[impl sync_archive.execute.sync] r[impl sync_archive.execute.sync.applies]
    // r[impl sync_archive.execute.sync.merge_preserves_wrapper]
    // r[impl sync_archive.execute.sync.merge_creates_spec]
    let delta_blocks = delta_requirement_blocks(delta);
    let headings = delta_blocks
        .iter()
        .filter_map(|block| requirement_heading(block))
        .collect::<Vec<_>>();
    let removed_headings = removed_requirement_headings(delta);
    let mut headings_to_replace = headings;
    headings_to_replace.extend(removed_headings);
    headings_to_replace.sort();
    headings_to_replace.dedup();

    let base = accepted
        .filter(|text| !text.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| empty_accepted_spec(spec_id));
    let (prefix, requirements) =
        split_requirements(&base).unwrap_or_else(|| (base.trim_end().to_string(), String::new()));
    let kept = remove_requirement_blocks(&requirements, &headings_to_replace);
    let merged_requirements = append_blocks(&kept, &delta_blocks);
    let markdown = format!(
        "{}\n\n## Requirements\n\n{}\n",
        prefix.trim_end(),
        merged_requirements.trim()
    );

    DeltaMerge {
        markdown,
        added_blocks: delta_blocks.len(),
        removed_blocks: headings_to_replace.len(),
    }
}

fn empty_accepted_spec(spec_id: &str) -> String {
    format!(
        "# {} Specification\n\n## Purpose\n\nDefines the `{}` capability.\n\n## Requirements\n",
        title_from_spec_id(spec_id),
        spec_id
    )
}

fn title_from_spec_id(spec_id: &str) -> String {
    spec_id
        .split(['-', '_', '/', '.'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_requirements(markdown: &str) -> Option<(String, String)> {
    let marker = "## Requirements";
    let offset = markdown.find(marker)?;
    let prefix = markdown[..offset].trim_end().to_string();
    let requirements = markdown[offset + marker.len()..].trim().to_string();
    Some((prefix, requirements))
}

fn delta_requirement_blocks(delta: &str) -> Vec<String> {
    let mut blocks = Vec::with_capacity(delta.matches("### Requirement:").count());
    for section in ["## ADDED Requirements", "## MODIFIED Requirements"] {
        let Some(section_body) = section_body(delta, section) else {
            continue;
        };
        blocks.extend(requirement_blocks(section_body));
    }
    blocks
}

fn removed_requirement_headings(delta: &str) -> Vec<String> {
    let Some(section_body) = section_body(delta, "## REMOVED Requirements") else {
        return Vec::new();
    };
    requirement_blocks(section_body)
        .iter()
        .filter_map(|block| requirement_heading(block))
        .collect()
}

fn section_body<'a>(markdown: &'a str, section: &str) -> Option<&'a str> {
    let start = markdown.find(section)? + section.len();
    let rest = &markdown[start..];
    let end = rest.find("\n## ").unwrap_or(rest.len());
    Some(&rest[..end])
}

fn requirement_blocks(section: &str) -> Vec<String> {
    let mut blocks = Vec::with_capacity(section.matches("### Requirement:").count());
    let mut current = Vec::with_capacity(section.lines().count());
    for line in section.lines() {
        if line.starts_with("### Requirement:") && !current.is_empty() {
            blocks.push(current.join("\n").trim().to_string());
            current.clear();
        }
        if line.starts_with("### Requirement:") || !current.is_empty() {
            current.push(line.to_string());
        }
    }
    if !current.is_empty() {
        blocks.push(current.join("\n").trim().to_string());
    }
    blocks.retain(|block| !block.trim().is_empty());
    blocks
}

fn remove_requirement_blocks(requirements: &str, headings: &[String]) -> String {
    let line_count = requirements.lines().count();
    let mut output = Vec::with_capacity(requirements.matches("### Requirement:").count() + 1);
    let mut current = Vec::with_capacity(line_count);
    let mut current_heading = None;

    for line in requirements.lines() {
        if line.starts_with("### Requirement:") {
            if let Some(block) = kept_block(&current, current_heading.as_deref(), headings) {
                output.push(block);
            }
            current.clear();
            current_heading = Some(line.to_string());
        }
        current.push(line.to_string());
    }
    if let Some(block) = kept_block(&current, current_heading.as_deref(), headings) {
        output.push(block);
    }
    output.join("\n\n")
}

fn kept_block(
    current: &[String],
    heading: Option<&str>,
    headings_to_remove: &[String],
) -> Option<String> {
    if current.is_empty() {
        return None;
    }
    let should_remove =
        heading.is_some_and(|text| headings_to_remove.iter().any(|item| item == text));
    if should_remove {
        None
    } else {
        Some(current.join("\n").trim().to_string())
    }
}

fn append_blocks(requirements: &str, blocks: &[String]) -> String {
    let mut parts = Vec::with_capacity(blocks.len() + 1);
    if !requirements.trim().is_empty() {
        parts.push(requirements.trim().to_string());
    }
    parts.extend(blocks.iter().map(|block| block.trim().to_string()));
    parts.join("\n\n")
}

fn requirement_heading(block: &str) -> Option<String> {
    block
        .lines()
        .find(|line| line.starts_with("### Requirement:"))
        .map(ToOwned::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_accepted_wrapper_when_merging_delta() {
        let accepted = "# Demo Specification\n\n## Purpose\n\nKeep this purpose.\n\n## Requirements\n\n### Requirement: Existing\n\nr[demo.existing] The system MUST keep old content.\n\n#### Scenario: Old\n\n- GIVEN old\n- WHEN old\n- THEN old\n";
        let delta = "## ADDED Requirements\n\n### Requirement: New\n\nr[demo.new] The system MUST add new content.\n\n#### Scenario: New\n\n- GIVEN new\n- WHEN new\n- THEN new\n";
        let merged = merge_delta_spec(Some(accepted), delta, "demo");
        assert!(merged.markdown.contains("# Demo Specification"));
        assert!(merged.markdown.contains("## Purpose\n\nKeep this purpose."));
        assert!(merged.markdown.contains("## Requirements"));
        assert!(merged.markdown.contains("### Requirement: Existing"));
        assert!(merged.markdown.contains("### Requirement: New"));
        assert!(!merged.markdown.contains("## ADDED Requirements"));
    }

    #[test]
    fn replaces_exact_modified_requirement_heading() {
        let accepted = "# Demo Specification\n\n## Requirements\n\n### Requirement: Existing\n\nr[demo.existing] The system MUST keep old content.\n";
        let delta = "## MODIFIED Requirements\n\n### Requirement: Existing\n\nr[demo.existing] The system MUST use updated content.\n";
        let merged = merge_delta_spec(Some(accepted), delta, "demo");
        assert!(!merged.markdown.contains("old content"));
        assert!(merged.markdown.contains("updated content"));
    }

    #[test]
    fn creates_stable_accepted_spec_for_new_domain() {
        let delta = "## ADDED Requirements\n\n### Requirement: First\n\nr[new.domain.first] The system MUST create a wrapped spec.\n";
        let merged = merge_delta_spec(None, delta, "new-domain");
        assert!(merged.markdown.starts_with("# New Domain Specification"));
        assert!(merged.markdown.contains("## Purpose"));
        assert!(merged.markdown.contains("## Requirements"));
        assert!(merged.markdown.contains("### Requirement: First"));
        assert!(!merged.markdown.contains("## ADDED Requirements"));
    }
}
