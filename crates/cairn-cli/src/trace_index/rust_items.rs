// Saved-byte Rust adapter. syn/proc-macro2 are used only at the shell boundary.
// No Buoy source is copied: the adapter identity is Cairn's own versioned namespace.
use cairn_core::{ContentBoundTraceLink, RustItemConfiguration, RustItemFact, TraceabilityFile};
use quote::ToTokens;
use syn::{spanned::Spanned, visit::Visit, Attribute, Expr, Item, Lit, Meta};

const MAX_RUST_SOURCE_BYTES: usize = 1024 * 1024;

#[derive(Default)]
struct MacroDetector(bool);

impl<'ast> Visit<'ast> for MacroDetector {
    fn visit_macro(&mut self, _macro: &'ast syn::Macro) {
        self.0 = true;
    }
}
const MAX_ITEMS: usize = 16_384;
const MAX_BRACE_DEPTH: usize = 128;

struct Candidate {
    start: usize,
    end: usize,
    kind: &'static str,
    locator: String,
    normalized: String,
    unsupported: Option<String>,
}

pub(super) fn facts(
    files: &[TraceabilityFile],
    links: &[ContentBoundTraceLink],
    configuration: &RustItemConfiguration,
) -> Result<Vec<RustItemFact>, String> {
    let mut output = Vec::with_capacity(links.len());
    for file in files {
        if !links.iter().any(|link| link.source_path == file.path) {
            continue;
        }
        if !file.path.ends_with(".rs") || file.text.len() > MAX_RUST_SOURCE_BYTES {
            return Err(format!(
                "{}: rust_item_v1 requires a bounded (.rs, at most 1 MiB) UTF-8 saved source",
                file.path
            ));
        }
        let mut brace_depth = 0usize;
        for byte in file.text.bytes() {
            match byte {
                b'{' => {
                    brace_depth = brace_depth.saturating_add(1);
                    if brace_depth > MAX_BRACE_DEPTH {
                        return Err(format!("{}: Rust nesting budget exceeded", file.path));
                    }
                }
                b'}' => brace_depth = brace_depth.saturating_sub(1),
                _ => {}
            }
        }
        let parsed = syn::parse_file(&file.text).map_err(|error| {
            format!("{}: unsupported/malformed Rust source: {error}", file.path)
        })?;
        let mut items = Vec::new();
        collect_items(&parsed.items, "crate", configuration, &mut items, 0, None)?;
        if items.len() > MAX_ITEMS {
            return Err(format!("{}: Rust item budget exceeded", file.path));
        }
        for link in links.iter().filter(|link| link.source_path == file.path) {
            let marker_end = usize::try_from(link.marker_span.end_byte)
                .map_err(|_| "marker offset exceeds usize")?;
            let selected = items
                .iter()
                .filter(|item| {
                    item.start >= marker_end
                        && file
                            .text
                            .get(marker_end..item.start)
                            .is_some_and(declaration_gap)
                })
                .min_by_key(|item| item.start);
            let Some(item) = selected else {
                return Err(format!(
                    "{}:{}: marker is not immediately followed by a parsed Rust item",
                    file.path, link.marker_span.start_byte
                ));
            };
            if let Some(reason) = &item.unsupported {
                return Err(format!(
                    "{}:{}: unsupported/incomplete Rust item: {reason}",
                    file.path, item.start
                ));
            }
            if item.end > file.text.len()
                || !file.text.is_char_boundary(item.start)
                || !file.text.is_char_boundary(item.end)
            {
                return Err(format!(
                    "{}: parser returned invalid UTF-8 byte boundaries",
                    file.path
                ));
            }
            output.push(RustItemFact {
                source_path: file.path.clone(),
                marker_start_byte: link.marker_span.start_byte,
                start_byte: item.start as u64,
                end_byte: item.end as u64,
                item_kind: item.kind.into(),
                scoped_locator: item.locator.clone(),
                normalized_item_blake3: normalized_hash(&item.normalized),
            });
        }
    }
    Ok(output)
}

fn normalized_hash(tokens: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"cairn.rust-item.normalized-tokens.v1\0");
    hasher.update(tokens.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn collect_items(
    items: &[Item],
    scope: &str,
    cfg: &RustItemConfiguration,
    out: &mut Vec<Candidate>,
    depth: usize,
    inherited: Option<&str>,
) -> Result<(), String> {
    if depth > 64 {
        return Err("Rust module nesting budget exceeded".into());
    }
    for (ordinal, item) in items.iter().enumerate() {
        if out.len() >= MAX_ITEMS {
            return Err("Rust item budget exceeded".into());
        }
        let (kind, name, attrs): (&str, String, &[Attribute]) = match item {
            Item::Fn(v) => ("fn", v.sig.ident.to_string(), &v.attrs),
            Item::Struct(v) => ("struct", v.ident.to_string(), &v.attrs),
            Item::Enum(v) => ("enum", v.ident.to_string(), &v.attrs),
            Item::Union(v) => ("union", v.ident.to_string(), &v.attrs),
            Item::Mod(v) => ("mod", v.ident.to_string(), &v.attrs),
            Item::Trait(v) => ("trait", v.ident.to_string(), &v.attrs),
            Item::Impl(v) => ("impl", v.self_ty.to_token_stream().to_string(), &v.attrs),
            Item::Type(v) => ("type", v.ident.to_string(), &v.attrs),
            Item::Const(v) => ("const", v.ident.to_string(), &v.attrs),
            Item::Static(v) => ("static", v.ident.to_string(), &v.attrs),
            Item::Macro(v) => ("macro", v.mac.path.to_token_stream().to_string(), &v.attrs),
            _ => ("unsupported", String::new(), &[]),
        };
        let span = item.span().byte_range();
        let start = attrs.first().map_or(span.start, |attr| {
            attr.span().byte_range().start.min(span.start)
        });
        let locator = format!("{scope}/{kind}:{name}#{ordinal}");
        let mut unsupported = if matches!(item, Item::Macro(_)) {
            Some("macro expansion is not resolved".to_string())
        } else if matches!(item, Item::Mod(module) if module.content.is_none()) {
            Some("external module content is not resolved by this saved-file adapter".into())
        } else if kind == "unsupported" {
            Some("declaration kind is outside the rust_item_v1 subset".to_string())
        } else {
            None
        };
        for attr in attrs {
            match attribute_enabled(attr, cfg) {
                Ok(true) => {}
                Ok(false) => {
                    unsupported =
                        Some("item disabled by explicit cfg; no active item to bind".into())
                }
                Err(reason) => unsupported = Some(reason),
            }
        }
        let normalized = item.to_token_stream().to_string();
        let mut macros = MacroDetector::default();
        macros.visit_item(item);
        if macros.0 {
            unsupported = Some("macro expansion inside item is not resolved".into());
        }
        if unsupported.is_none() {
            unsupported = inherited.map(str::to_string);
        }
        let parent_unsupported = unsupported.clone();
        out.push(Candidate {
            start,
            end: span.end,
            kind: match kind {
                "fn" => "fn",
                "struct" => "struct",
                "enum" => "enum",
                "union" => "union",
                "mod" => "mod",
                "trait" => "trait",
                "impl" => "impl",
                "type" => "type",
                "const" => "const",
                "static" => "static",
                _ => "unsupported",
            },
            locator: locator.clone(),
            normalized,
            unsupported,
        });
        if let Item::Mod(module) = item {
            if let Some((_, children)) = &module.content {
                collect_items(
                    children,
                    &locator,
                    cfg,
                    out,
                    depth + 1,
                    parent_unsupported.as_deref(),
                )?;
            }
        }
        if let Item::Trait(item_trait) = item {
            for (member_index, member) in item_trait.items.iter().enumerate() {
                let span = member.span().byte_range();
                let (kind, name, attrs): (&str, String, &[Attribute]) = match member {
                    syn::TraitItem::Fn(v) => ("trait_fn", v.sig.ident.to_string(), &v.attrs),
                    syn::TraitItem::Const(v) => ("trait_const", v.ident.to_string(), &v.attrs),
                    syn::TraitItem::Type(v) => ("trait_type", v.ident.to_string(), &v.attrs),
                    _ => ("unsupported", String::new(), &[]),
                };
                let mut macros = MacroDetector::default();
                macros.visit_trait_item(member);
                push_member(
                    out,
                    span,
                    attrs,
                    kind,
                    name,
                    &locator,
                    member_index,
                    member.to_token_stream().to_string(),
                    macros.0,
                    parent_unsupported.as_deref(),
                    cfg,
                )?;
            }
        }
        if let Item::Impl(item_impl) = item {
            for (member_index, member) in item_impl.items.iter().enumerate() {
                let span = member.span().byte_range();
                let (kind, name, attrs): (&str, String, &[Attribute]) = match member {
                    syn::ImplItem::Fn(v) => ("impl_fn", v.sig.ident.to_string(), &v.attrs),
                    syn::ImplItem::Const(v) => ("impl_const", v.ident.to_string(), &v.attrs),
                    syn::ImplItem::Type(v) => ("impl_type", v.ident.to_string(), &v.attrs),
                    _ => ("unsupported", String::new(), &[]),
                };
                let mut macros = MacroDetector::default();
                macros.visit_impl_item(member);
                push_member(
                    out,
                    span,
                    attrs,
                    kind,
                    name,
                    &locator,
                    member_index,
                    member.to_token_stream().to_string(),
                    macros.0,
                    parent_unsupported.as_deref(),
                    cfg,
                )?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn push_member(
    out: &mut Vec<Candidate>,
    span: std::ops::Range<usize>,
    attrs: &[Attribute],
    kind: &str,
    name: String,
    scope: &str,
    ordinal: usize,
    normalized: String,
    has_macro: bool,
    inherited: Option<&str>,
    cfg: &RustItemConfiguration,
) -> Result<(), String> {
    if out.len() >= MAX_ITEMS {
        return Err("Rust item budget exceeded".into());
    }
    let mut unsupported = if kind == "unsupported" || has_macro {
        Some("unsupported member or unresolved macro expansion".to_string())
    } else {
        None
    };
    for attr in attrs {
        match attribute_enabled(attr, cfg) {
            Ok(true) => {}
            Ok(false) => unsupported = Some("member disabled by cfg".into()),
            Err(reason) => unsupported = Some(reason),
        }
    }
    if unsupported.is_none() {
        unsupported = inherited.map(str::to_string);
    }
    out.push(Candidate {
        start: attrs.first().map_or(span.start, |attr| {
            attr.span().byte_range().start.min(span.start)
        }),
        end: span.end,
        kind: match kind {
            "trait_fn" => "trait_fn",
            "trait_const" => "trait_const",
            "trait_type" => "trait_type",
            "impl_fn" => "impl_fn",
            "impl_const" => "impl_const",
            "impl_type" => "impl_type",
            _ => "unsupported",
        },
        locator: format!("{scope}/{kind}:{name}#{ordinal}"),
        normalized,
        unsupported,
    });
    Ok(())
}

fn attribute_enabled(attr: &Attribute, cfg: &RustItemConfiguration) -> Result<bool, String> {
    if attr.path().is_ident("cfg") {
        let Meta::List(list) = &attr.meta else {
            return Err("malformed cfg attribute".into());
        };
        let predicate: Meta =
            syn::parse2(list.tokens.clone()).map_err(|_| "unsupported cfg expression")?;
        cfg_value(&predicate, cfg)
    } else if [
        "doc",
        "allow",
        "warn",
        "deny",
        "forbid",
        "inline",
        "cold",
        "must_use",
        "deprecated",
        "repr",
        "non_exhaustive",
        "track_caller",
        "test",
    ]
    .iter()
    .any(|name| attr.path().is_ident(name))
    {
        Ok(true)
    } else {
        Err(format!("attribute {} may expand or depends on an unsupported target; interpretation incomplete", attr.path().to_token_stream()))
    }
}

fn cfg_value(meta: &Meta, cfg: &RustItemConfiguration) -> Result<bool, String> {
    match meta {
        Meta::List(list) => {
            let args = list
                .parse_args_with(
                    syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
                )
                .map_err(|_| "malformed cfg predicate")?;
            if list.path.is_ident("all") {
                args.iter()
                    .try_fold(true, |value, arg| Ok(value & cfg_value(arg, cfg)?))
            } else if list.path.is_ident("any") {
                args.iter()
                    .try_fold(false, |value, arg| Ok(value | cfg_value(arg, cfg)?))
            } else if list.path.is_ident("not") && args.len() == 1 {
                Ok(!cfg_value(&args[0], cfg)?)
            } else {
                Err(format!(
                    "unknown cfg operator: {}",
                    list.path.to_token_stream()
                ))
            }
        }
        Meta::Path(path) => {
            let key = path.to_token_stream().to_string();
            if key == "unix" {
                Ok(cfg.target.contains("linux")
                    || cfg.target.contains("darwin")
                    || cfg.target.contains("bsd"))
            } else if key == "windows" {
                Ok(cfg.target.contains("windows"))
            } else if cfg.cfg.iter().any(|value| value == &key) {
                Ok(true)
            } else {
                Err(format!(
                    "unknown cfg flag {key}; declare it in configuration.cfg"
                ))
            }
        }
        Meta::NameValue(pair) => {
            let Expr::Lit(literal) = &pair.value else {
                return Err("cfg value must be a string literal".into());
            };
            let Lit::Str(value) = &literal.lit else {
                return Err("cfg value must be a string literal".into());
            };
            let key = pair.path.to_token_stream().to_string();
            match key.as_str() {
                "feature" => {
                    if !cfg
                        .declared_features
                        .iter()
                        .any(|feature| feature == &value.value())
                    {
                        Err(format!(
                            "unknown feature {}; declare it in configuration.declared_features",
                            value.value()
                        ))
                    } else {
                        Ok(cfg.features.iter().any(|feature| feature == &value.value()))
                    }
                }
                "target_arch" => Ok(cfg.target.split('-').next() == Some(value.value().as_str())),
                "target_os" => Ok(cfg.target.contains(value.value().as_str())),
                "target_family" => Ok(match value.value().as_str() {
                    "unix" => {
                        cfg.target.contains("linux")
                            || cfg.target.contains("darwin")
                            || cfg.target.contains("bsd")
                    }
                    "windows" => cfg.target.contains("windows"),
                    _ => return Err("unknown target family".into()),
                }),
                _ => {
                    let exact = format!("{key}=\"{}\"", value.value());
                    if cfg.cfg.iter().any(|entry| entry == &exact) {
                        Ok(true)
                    } else {
                        Err(format!(
                            "unknown cfg value {exact}; declare it in configuration.cfg"
                        ))
                    }
                }
            }
        }
    }
}

fn declaration_gap(mut gap: &str) -> bool {
    loop {
        gap = gap.trim_start();
        if gap.is_empty() {
            return true;
        }
        if let Some(rest) = gap.strip_prefix("//") {
            gap = rest.find('\n').map_or("", |position| &rest[position + 1..]);
        } else if let Some(rest) = gap.strip_prefix("/*") {
            let Some(end) = rest.find("*/") else {
                return false;
            };
            gap = &rest[end + 2..];
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Assembled so this fixture source does not register as a live traceability
    /// claim for the intentionally undeclared `demo.item` requirement.
    const MARKER: &str = concat!("r[im", "pl demo.item]");

    fn config() -> RustItemConfiguration {
        RustItemConfiguration {
            source_path: "rust-config.json".into(),
            file_blake3: "a".repeat(64),
            schema_version: "cairn.rust-item-configuration.v1".into(),
            target: "x86_64-unknown-linux-gnu".into(),
            features: vec![],
            declared_features: vec!["missing".into()],
            cfg: vec![],
        }
    }
    fn link(path: &str, start: usize, end: usize) -> ContentBoundTraceLink {
        ContentBoundTraceLink {
            requirement_id: "demo.item".into(),
            requirement_row_blake3: "a".repeat(64),
            role: "impl".into(),
            source_path: path.into(),
            source_file_blake3: "b".repeat(64),
            marker_span: cairn_core::ContentBoundTraceMarkerSpan {
                start_byte: start as u64,
                end_byte: end as u64,
                start_line: 1,
                end_line: 1,
            },
            marker_blake3: "c".repeat(64),
            link_blake3: "d".repeat(64),
        }
    }

    #[test]
    fn saved_bytes_resolve_nested_duplicate_names_without_collapsing() {
        let path = "src/lib.rs";
        let text = concat!(
            "mod α {\r\n// r[im",
            "pl demo.item]\r\n/* é */ fn x() {}\r\n}\r\nmod β {\r\n// r[im",
            "pl demo.item]\r\nfn x() {}\r\n}\r\n"
        );
        let markers = text
            .match_indices(MARKER)
            .map(|(at, raw)| link(path, at, at + raw.len()))
            .collect::<Vec<_>>();
        let resolved = facts(
            &[TraceabilityFile {
                path: path.into(),
                text: text.into(),
            }],
            &markers,
            &config(),
        )
        .unwrap();
        assert_eq!(resolved.len(), 2);
        assert_eq!(
            resolved[0].start_byte as usize,
            text.find("fn x()").unwrap()
        );
        assert_ne!(resolved[0].scoped_locator, resolved[1].scoped_locator);
        assert_eq!(
            resolved[0].normalized_item_blake3,
            resolved[1].normalized_item_blake3
        );
    }

    #[test]
    fn disabled_enclosing_module_cannot_expose_child_item() {
        let source = concat!(
            "#[cfg(any())] mod hidden { // r[im",
            "pl demo.item]\nfn x() {} }"
        );
        let marker = source.find(MARKER).unwrap();
        let selected = facts(
            &[TraceabilityFile {
                path: "src/lib.rs".into(),
                text: source.into(),
            }],
            &[link("src/lib.rs", marker, marker + MARKER.len())],
            &config(),
        );
        assert!(selected.unwrap_err().contains("disabled"));
    }

    #[test]
    fn cfg_unknown_and_macro_expansion_fail_closed() {
        for source in [
            concat!(
                "// r[im",
                "pl demo.item]\n#[cfg(target_feature = \"imaginary\")] fn x() {}"
            ),
            concat!("// r[im", "pl demo.item]\nfoo!();"),
        ] {
            let marker = link("src/lib.rs", 3, 20);
            let result = facts(
                &[TraceabilityFile {
                    path: "src/lib.rs".into(),
                    text: source.into(),
                }],
                &[marker],
                &config(),
            );
            assert!(result.is_err(), "unsupported source should fail: {source}");
        }
    }

    #[test]
    fn ordinary_logical_negation_is_not_a_macro() {
        let source = concat!(
            "// r[im",
            "pl demo.item]\nfn x(value: bool) -> bool { !value }"
        );
        let result = facts(
            &[TraceabilityFile {
                path: "src/lib.rs".into(),
                text: source.into(),
            }],
            &[link("src/lib.rs", 3, 20)],
            &config(),
        );
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn source_budget_fails_before_parsing() {
        let mut source = String::from(concat!("// r[im", "pl demo.item]\nfn x() {}\n"));
        source.push_str(&" ".repeat(MAX_RUST_SOURCE_BYTES));
        let result = facts(
            &[TraceabilityFile {
                path: "src/lib.rs".into(),
                text: source,
            }],
            &[link("src/lib.rs", 3, 20)],
            &config(),
        );
        assert!(result.unwrap_err().contains("bounded"));
    }

    #[test]
    fn cfg_is_structural_not_substring_based() {
        let meta: Meta = syn::parse_str("not(any())").unwrap();
        assert!(cfg_value(&meta, &config()).unwrap());
        let meta: Meta = syn::parse_str("any(feature = \"missing\")").unwrap();
        assert!(!cfg_value(&meta, &config()).unwrap());
        let meta: Meta = syn::parse_str("unknown_condition").unwrap();
        assert!(cfg_value(&meta, &config()).is_err());
        let unknown: Meta = syn::parse_str("feature = \"not-declared\"").unwrap();
        assert!(cfg_value(&unknown, &config())
            .unwrap_err()
            .contains("unknown feature"));
    }

    #[test]
    fn unicode_attribute_keeps_exact_item_byte_boundary() {
        let text = concat!(
            "// r[im",
            "pl demo.item]\r\n/* é */ #[doc = \"λ\"] fn café() {}\r\n"
        );
        let file = TraceabilityFile {
            path: "src/lib.rs".into(),
            text: text.into(),
        };
        let selected = facts(&[file], &[link("src/lib.rs", 3, 20)], &config()).unwrap();
        assert_eq!(selected[0].start_byte as usize, text.find("#[doc").unwrap());
        assert_eq!(
            selected[0].end_byte as usize,
            text.find("}\r\n").unwrap() + 1
        );
    }

    #[test]
    fn byte_spans_survive_unicode_and_crlf() {
        let text = "/* é */ fn x() {}\r\nmod λ { /* é */ fn café() {} }";
        let parsed = syn::parse_file(text).unwrap();
        let mut items = Vec::new();
        collect_items(&parsed.items, "crate", &config(), &mut items, 0, None).unwrap();
        assert_eq!(items[0].start, text.find("fn x").unwrap());
        assert_eq!(items[1].start, text.find("mod λ").unwrap());
        assert_eq!(items[2].start, text.find("fn café").unwrap());
        assert!(text.is_char_boundary(items[2].start));
    }
}
