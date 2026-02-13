# TokenUsageGuard / NonActuatingToken Policy

This document specifies the TokenUsageGuard, a pure policy kernel that constrains how readonly diagnostics (Tree-of-Life, Church-of-FEAR, data-sovereignty views, moral scores) may be consumed in the NewRow-Print! stack.[file:10][file:19]

## Scope

NonActuatingToken domain includes:

- TreeOfLifeView (normalized TREE assets).[file:19]
- TreeOfLifeDiagnostics (NATURE predicates like CALMSTABLE, OVERLOADED, RECOVERY, UNFAIRDRAIN).[file:10]
- NonActuatingDataSovView (data-sovereignty snapshots over .evolve.jsonl and .donutloop.aln).[file:10]
- ChurchAccountView (Church-of-FEAR ChurchAccountState).[file:10]
- MoralPositionScore (AutoChurch-style moralposition diagnostics).[file:10]

These tokens are **observer-only**; they MUST NOT write CapabilityState, ConsentState, PolicyStack, BiophysicalEnvelopeSpec, or stake/neurorights shards.[file:10][file:19]

## Allowed sinks

Per `TOL-TOKEN-POLICY`:

- Hud: rendering in HUD or UI.
- AiChat: explanation and documentation in AI-chat.
- AnalyticsOffline: offline analytics in CapModelOnly or CapLabBench contexts only.[file:10]

Any (TokenKind, TokenSink) pair not explicitly listed as allowed is denied by default.[file:10]

## Forbidden sinks

The following sinks MUST NEVER consume NonActuatingToken values as control inputs:

- CapabilityEngine (CapabilityTransitionRequest, CapabilityGuard).[file:19]
- ReversalKernel (ReversalConditions, neuromorph reversal kernels).[file:19]
- RewardModel (reward/econet/scoring/ranking models).
- EnvelopeKernel (BiophysicalEnvelopeSpec kernels, except for passive correlation-only logging).[file:19]
- StakeGovernance (.stake.aln, neurorights, role allocation).[file:10]

TokenUsageGuard implements these constraints as a pure function:

```rust
fn evaluate_token_usage(kind: TokenKind, sink: TokenSink) -> TokenUsageDecision
