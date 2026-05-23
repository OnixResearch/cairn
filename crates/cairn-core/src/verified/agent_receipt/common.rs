use super::types::AgentRunGateIssue;

pub(super) fn is_blake3_hex(candidate: &str) -> bool {
    candidate.len() == 64 && candidate.as_bytes().iter().all(u8::is_ascii_hexdigit)
}

pub(super) fn issue(code: impl Into<String>, message: impl Into<String>) -> AgentRunGateIssue {
    AgentRunGateIssue {
        code: code.into(),
        message: message.into(),
    }
}
