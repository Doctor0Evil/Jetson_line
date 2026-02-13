# Contributing to Jetson-Line Tree-of-Life Colonizer

This repository is a **research crate** for Tree-of-Life stewardship and fairness studies.  
All contributions must preserve **nonfictional**, **numeric**, and **auditable** behavior.

## Core principles

1. **Nonfictional mechanics only**
   - Every new rule must be an explicit numeric function of the existing state.
   - No narrative “events,” hidden variables, or story-driven overrides.
   - All state changes must come from functions in `dynamics`, `deeds`, or `invariants`.

2. **Tree-of-Life fairness**
   - Fairness is defined as:
     - “Minimal acceptable inequality with globally decreasing biophysical load and bounded POWER, under explicit, auditable rules.”
   - New features must not introduce unbounded POWER, untracked load, or silent harm shifting.

3. **Neuromorph‑GOD invariants**
   - Global POWER must remain bounded by CHURCH via `GlobalConstraints`.
   - Bioload must remain bounded per-site and globally.
   - FEAR bands must gate high‑impact actions.

4. **Total transparency**
   - Any action that changes CHURCH/FEAR/POWER/TECH or bioload in a safety‑relevant way must emit a `Deed`.
   - Deeds must be judged into `Judgment` with clear rationale.

## How to contribute

1. Fork and branch
   - Fork the repo and create a feature branch:
     - `feature/add-help-deed`
     - `feature/add-pollution-extension`

2. Extending state
   - Add new fields only in `model.rs`.
   - Provide reasonable defaults and bounds in `WorldParams` or dedicated parameter structs.

3. Adding deeds
   - Add new variants to `DeedKind` in `deeds.rs`.
   - Implement explicit state transitions in `dynamics.rs` or a dedicated helper.
   - Log pre/post state into `Deed`.

4. Updating invariants
   - Add or modify global safety rules in `invariants.rs`.
   - Keep them numeric and documented.
   - Avoid breaking existing invariants; if changes are necessary, explain them in PR text.

5. Metrics and knowledge objects
   - All new metrics should be added in `metrics.rs`.
   - If they affect episode structure, extend `Episode` in `episode.rs` and keep JSON/CBOR stable when possible.

## Coding style

- Rust 2021 edition.
- `rustfmt` with the repository `rustfmt.toml`.
- Prefer pure functions and immutable data where possible inside computation steps.
- Keep module boundaries:
  - `model` – data types only.
  - `dynamics` – local update rules + colonization.
  - `invariants` – global Neuromorph‑GOD constraints.
  - `deeds` – logging of events.
  - `judgement` – moral scoring logic.
  - `metrics` – quantitative summaries.
  - `episode` – orchestration, serialization.

Thank you for helping keep this crate transparent, stable, and usable for Tree‑of‑Life research.
