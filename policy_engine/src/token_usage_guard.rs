// TokenUsageGuard: runtime firewall for Tree-of-Life, Church-of-FEAR,
// and data-sovereignty diagnostics. Enforces that these "NonActuatingToken"
// values can only flow to HUD/AI-chat/offline analytics, never into
// capability, reversal, reward, envelope, or stake kernels.
//
// This module is pure: no IO, no global state. It is intended to be called
// from sovereignty core whenever a service attempts to consume a diagnostic
// token.

#![allow(dead_code)]

/// Token kinds covered by TOL-TOKEN-POLICY.
/// These are readonly diagnostics derived from governed inputs
/// (TreeOfLifeView, NATURE predicates, data-sovereignty views, ChurchAccountState).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TokenKind {
    TreeOfLifeView,
    TreeOfLifeDiagnostics,
    NonActuatingDataSovView,
    ChurchAccountView,
    MoralPositionScore,
}

/// Sinks that may attempt to consume a token.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum TokenSink {
    Hud,
    AiChat,
    AnalyticsOffline,
    CapabilityEngine,
    ReversalKernel,
    RewardModel,
    EnvelopeKernel,
    StakeGovernance,
}

/// Decision outcome for a token usage request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenUsageDecision {
    Allowed,
    Denied(TokenUsageReason),
}

/// Enumerates why a token usage was denied (or which allow rule fired).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenUsageReason {
    // Positive reasons (for auditing which allow rule applied)
    AllowedTokenUsageHud,
    AllowedTokenUsageAiChat,
    AllowedTokenUsageAnalyticsOffline,

    // Deny reasons
    DeniedTokenUsageByDefault,
    DeniedTokenActuationInCapabilityEngine,
    DeniedTokenActuationInReversalKernel,
    DeniedTokenActuationInRewardModel,
    DeniedTokenActuationInEnvelopeKernel,
    DeniedTokenActuationInStakeGovernance,
}

/// Pure evaluation function implementing the matrix in TOL-TOKEN-POLICY.
///
/// This function MUST remain side-effect free and determined solely by
/// `kind` and `sink`. Higher-level code is responsible for logging the
/// resulting decision into WORM logs (.donutloop.aln or aligned ledgers).
pub fn evaluate_token_usage(kind: TokenKind, sink: TokenSink) -> TokenUsageDecision {
    use TokenKind::*;
    use TokenSink::*;
    use TokenUsageDecision::*;
    use TokenUsageReason::*;

    match (kind, sink) {
        // Allowed sinks: HUD
        (TreeOfLifeView, Hud)
        | (TreeOfLifeDiagnostics, Hud)
        | (NonActuatingDataSovView, Hud)
        | (ChurchAccountView, Hud)
        | (MoralPositionScore, Hud) => AllowedTokenUsageHud.into(),

        // Allowed sinks: AI-chat
        (TreeOfLifeView, AiChat)
        | (TreeOfLifeDiagnostics, AiChat)
        | (NonActuatingDataSovView, AiChat)
        | (ChurchAccountView, AiChat)
        | (MoralPositionScore, AiChat) => AllowedTokenUsageAiChat.into(),

        // Allowed sinks: offline analytics (CapModelOnly / CapLabBench contexts)
        (TreeOfLifeView, AnalyticsOffline)
        | (TreeOfLifeDiagnostics, AnalyticsOffline)
        | (NonActuatingDataSovView, AnalyticsOffline)
        | (ChurchAccountView, AnalyticsOffline)
        | (MoralPositionScore, AnalyticsOffline) => AllowedTokenUsageAnalyticsOffline.into(),

        // Forbidden: capability engine
        (_, CapabilityEngine) => Denied(DeniedTokenActuationInCapabilityEngine),

        // Forbidden: ReversalConditions or related kernels
        (_, ReversalKernel) => Denied(DeniedTokenActuationInReversalKernel),

        // Forbidden: reward / econet models
        (_, RewardModel) => Denied(DeniedTokenActuationInRewardModel),

        // Forbidden: envelope kernels (beyond passive logging)
        (_, EnvelopeKernel) => Denied(DeniedTokenActuationInEnvelopeKernel),

        // Forbidden: stake / neurorights / role governance
        (_, StakeGovernance) => Denied(DeniedTokenActuationInStakeGovernance),

        // Safety default for any future sink not explicitly covered
        _ => Denied(DeniedTokenUsageByDefault),
    }
}

// Small helper to reduce verbosity in the match above.
impl From<TokenUsageReason> for TokenUsageDecision {
    fn from(r: TokenUsageReason) -> Self {
        TokenUsageDecision::AllowedIf(r)
    }
}

// Adjust the enum above to this pattern if you prefer:
//   pub enum TokenUsageDecision {
//       AllowedIf(TokenUsageReason), // carries which allow rule fired
//       Denied(TokenUsageReason),
//   }
// For strict minimalism, you can instead keep Allowed as a bare variant and
// log the mapping from (kind, sink) to policy rule in the caller.

/// Example pure helper to classify a sink as "actuating" vs "non-actuating".
pub fn is_actuating_sink(sink: TokenSink) -> bool {
    use TokenSink::*;
    matches!(
        sink,
        CapabilityEngine | ReversalKernel | RewardModel | EnvelopeKernel | StakeGovernance
    )
}
