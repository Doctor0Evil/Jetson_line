# Jetson-Line Tree-of-Life Colonizer

A nonfictional, 1‑D **Tree-of-Life** microsociety simulator implemented as a Rust crate.  
Each site on a 1‑D Jetson-Line carries CHURCH, FEAR, POWER, TECH, bioload, and trust fields, and must sustain biophysical life under explicit, auditable constraints.

The crate is a **research tool**, not a game. It is designed to produce stable, reproducible **knowledge objects** (episode logs, metrics, and W‑cycle reflections) for studying:

- When fair colonization regimes emerge (low collapse, low inequality, decreasing global bioload).
- How different colonization and repair policies affect stability and perceived fairness.
- Which CHURCH/FEAR/POWER/TECH rules yield “allowed miracles”: large system‑wide improvements that respect all biophysical bounds.

## Features

- 1‑D lattice `World` with per‑site:
  - `church`, `fear`, `power`, `tech`
  - `bioload`, `bioload_max`
  - `trust_left`, `trust_right`
  - `occupied` flag
- Deterministic `step_world` update:
  - FEAR from bioload and trust
  - POWER minting in a safe FEAR band and under CHURCH/POWER caps
  - TECH growth attenuated by bioload
  - Colonization only when trunk CHURCH/FEAR/POWER/TECH and load constraints are satisfied
- Neuromorph‑GOD invariants:
  - Global POWER ≤ k · global CHURCH
  - Per‑site and global bioload ceilings
  - FEAR bands for high‑impact actions
- Deed layer:
  - `Colonize` plus hooks for `Help`, `Conflict`, `Repair`, `DeployCleanTech`, etc.
- Judgment layer:
  - Per‑deed `MoralScores` (harmscore, fairnessscore, responsibilityscore, overallmoralscore)
- Episode log + metrics:
  - Cooperation index
  - Gini for CHURCH/POWER/TECH
  - Collapse count under bioload limits
  - Optional W‑cycle reflection (`What`, `So what`, `Now what`)

All mechanics are strictly numeric and deterministic; there are no fictional powers, random “events,” or hidden states.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
jetsonline-treeoflife = { path = "." }

Run the examples:

bash
cargo run --example minimal_run
cargo run --example fair_colonization
Each example prints a JSON Episode object to stdout. You can redirect this to a file and analyze with your preferred tools.

bash
cargo run --example minimal_run > episode_minimal.json
API sketch
rust
use jetsonline_treeoflife::{
    World, WorldParams, GlobalConstraints, Episode, EpisodeConfig, step_world,
};

fn main() {
    let params = WorldParams::default();
    let constraints = GlobalConstraints::default();
    let mut world = World::new(params, constraints, 10.0);
    world.occupy_origin(params.length / 2, 10.0);

    let mut episode = Episode::new(EpisodeConfig { max_ticks: 100 }, world);
    episode.run(|w| step_world(w).deeds);

    println!("{}", episode.to_json().unwrap());
}
Project invariants
No fictional mechanics: every rule is an explicit numeric function of the state.

Tree-of-Life fairness principle:

“Minimal acceptable inequality with globally decreasing biophysical load and bounded POWER, under explicit, auditable rules.”

All changes that cross moral or safety thresholds must be logged as Deed records.

Neuromorph‑GOD invariants must be enforced every step.

License
Dual-licensed under MIT or Apache‑2.0 at your option. See LICENSE-MIT and LICENSE-APACHE.

text

***

### LICENSE-MIT

```text
MIT License

Copyright (c) 2026 ...

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
