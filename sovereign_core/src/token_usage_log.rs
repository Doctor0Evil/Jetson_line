// Thin, pure facade between TokenUsageGuard decisions and your WORM logging
// layer. This module does NOT write files directly; it just shapes the
// TokenUsageEvent payload that your existing ledger code serializes into
// .evolve.jsonl / .donutloop.aln. [file:10][file:19]

use crate::policy_engine::token_usage_guard::{TokenKind, TokenSink, TokenUsageDecision, TokenUsageReason};

/// Logical representation of a TokenUsageEvent for logging.
/// This struct should map 1:1 to TOL-TOKEN-USAGE-KEYS/FIELDS. [file:19]
#[derive(Clone, Debug)]
pub struct TokenUsageEvent {
    pub usage_id: String,
    pub subject_id: Option<String>,
    pub service_id: String,
    pub timestamp_utc: String, // ISO-8601
    pub donutloop_ref: Option<String>,

    pub token_kind: TokenKind,
    pub token_sink: TokenSink,
    pub decision: String,
    pub decision_reason: String,
    pub caller_context: Option<String>,
}

/// Pure helper to build a TokenUsageEvent from a guard decision.
/// The caller is responsible for generating usage_id and timestamp,
/// and for passing the event to the WORM logger. [file:10][file:19]
pub fn make_token_usage_event(
    usage_id: String,
    subject_id: Option<String>,
    service_id: String,
    timestamp_utc: String,
    donutloop_ref: Option<String>,
    kind: TokenKind,
    sink: TokenSink,
    decision: TokenUsageDecision,
    caller_context: Option<String>,
) -> TokenUsageEvent {
    let (decision_str, reason_str) = match decision {
        TokenUsageDecision::Allowed => (
            "Allowed".to_string(),
            match sink {
                TokenSink::Hud => "AllowedTokenUsageHud".to_string(),
                TokenSink::AiChat => "AllowedTokenUsageAiChat".to_string(),
                TokenSink::AnalyticsOffline => "AllowedTokenUsageAnalyticsOffline".to_string(),
                _ => "AllowedTokenUsageOther".to_string(),
            },
        ),
        TokenUsageDecision::Denied(reason) => (
            "Denied".to_string(),
            format!("{:?}", reason),
        ),
    };

    TokenUsageEvent {
        usage_id,
        subject_id,
        service_id,
        timestamp_utc,
        donutloop_ref,
        token_kind: kind,
        token_sink: sink,
        decision: decision_str,
        decision_reason: reason_str,
        caller_context,
    }
}
