# INTEGRATION.md — topo-sonata-rs

> **Repository:** `SuperInstance/topo-sonata-rs`  
> **Language:** Rust  
> **License:** MIT  
> **Purpose:** Topological analysis of musical sonata form — structure, transitions, motifs, tension, and recapitulation similarity.

---

## What This Crate Provides

`topo-sonata-rs` applies algebraic topology to the analysis of musical structure, specifically sonata form. It models:

- **Form structure** — exposition, development, recapitulation, coda as typed sections with key and timing
- **Tonal transitions** — key changes on the circle of fifths with path computation and classification
- **Motivic complexes** — Vietoris-Rips complexes built from motif feature vectors, with persistence analysis
- **Tension landscapes** — synthetic harmonic tension curves with peak/valley detection and arc integration
- **Recapitulation analysis** — Wasserstein-based similarity measurement between exposition and recapitulation

All operations work with symbolic data — no audio dependencies required.

---

## Quick Start

```toml
[dependencies]
topo-sonata-rs = { git = "https://github.com/SuperInstance/topo-sonata-rs" }
```

```rust
use topo_sonata_rs::form::{SonataForm, SectionType};
use topo_sonata_rs::tension::TensionLandscape;
use topo_sonata_rs::transition::{transition_map, tonal_complexity};
use topo_sonata_rs::motif::{Motif, MotivicComplex};
use topo_sonata_rs::recapitulation::analyze_recapitulation;

// Build a standard sonata form
let sonata = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);

// Generate tension landscape
let landscape = TensionLandscape::from_sonata(&sonata);
println!("Max tension: {:.2}", landscape.max_tension());
println!("Tension peaks at measures: {:?}", landscape.peaks());

// Compute tonal transitions
let transitions = transition_map(&sonata);
println!("Total tonal distance: {:.1}", tonal_complexity(&sonata));

// Build motivic complex
let motifs = vec![
    Motif::new(0, vec![1.0, 0.0, 2.0], 1),
    Motif::new(1, vec![1.1, 0.1, 1.9], 5),
    Motif::new(2, vec![3.0, 1.0, 0.0], 41),
];
let complex = MotivicComplex::build(&motifs, 2.0);
println!("Motivic components: {}", complex.num_components());

// Analyze recapitulation fidelity
let analysis = analyze_recapitulation(&sonata);
println!("Wasserstein distance: {:.2}", analysis.wasserstein_dist);
println!("Key fidelity: {:.1}%", analysis.key_fidelity * 100.0);
```

---

## Cross-Repository Integration

### 1. Conservation-Law Budgeting for Musical Analysis Pipelines

Feed `topo-sonata-rs` analysis costs into `conservation-law-rs` to ensure the computational energy spent on each section of a piece is conserved across the analysis pipeline.

```rust
use topo_sonata_rs::form::SonataForm;
use conservation_law_rs::budget::EnergyBudget;

let sonata = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
let total_analysis_energy = 1000.0;

let mut budget = EnergyBudget::new(total_analysis_energy);
let exp_duration = sonata.exposition().iter().map(|s| s.duration()).sum::<usize>();
let dev_duration = sonata.development().map(|d| d.duration()).unwrap_or(0);
let rec_duration = sonata.recapitulation().iter().map(|s| s.duration()).sum::<usize>();
let total = exp_duration + dev_duration + rec_duration;

budget.allocate("exposition_analysis", total_analysis_energy * exp_duration as f64 / total as f64);
budget.allocate("development_analysis", total_analysis_energy * dev_duration as f64 / total as f64);
budget.allocate("recapitulation_analysis", total_analysis_energy * rec_duration as f64 / total as f64);

assert!(budget.check_conservation(), "Analysis energy budget must be conserved");
```

### 2. Fleet Registry via si-cli

Register the sonata-analysis capability with `si-cli` for ecosystem discovery.

```bash
# Scan and register
cargo run -- scan /path/to/topo-sonata-rs --sync

# Generate CAPABILITY.toml
cargo run -- generate capability --name topo-sonata-rs --output CAPABILITY.toml

# Check fleet-wide conservation
cargo run -- check --from-supabase
```

### 3. Witness Topology for Motivic Feature Clouds

Use `witness-topology-rs` to build persistent homology on motivic feature clouds extracted by `topo-sonata-rs`. This reveals the true topological shape of a composition's thematic material.

```rust
use topo_sonata_rs::motif::{Motif, cluster_motifs};
use witness_topology_rs::landmark::{maxmin_landmarks, distance_matrix};
use witness_topology_rs::witness::build_witness_complex;
use witness_topology_rs::persistence::{compute_persistence, betti_numbers};

// Collect motifs from multiple pieces
let all_motifs: Vec<Motif> = /* ... */ vec![];
let point_cloud: Vec<Vec<f64>> = all_motifs.iter().map(|m| m.features.clone()).collect();

let landmarks = maxmin_landmarks(&point_cloud, 30);
let complex = build_witness_complex(&point_cloud, &landmarks, 7);
let dist = distance_matrix(&point_cloud);
let pairs = compute_persistence(&complex, &dist);
let betti = betti_numbers(&pairs, f64::INFINITY);

println!("Motivic topology: β₀={}, β₁={}", betti[0], betti.get(1).unwrap_or(&0));
// β₀ = number of distinct thematic families
// β₁ = number of cyclic thematic relationships
```

### 4. Optimal Transport for Recapitulation Comparison

Replace the greedy Wasserstein approximation in `topo-sonata-rs` with the full Sinkhorn algorithm from `optimal-transport-agents-rs` for more accurate recapitulation similarity measurement.

```rust
use topo_sonata_rs::form::SonataForm;
use topo_sonata_rs::recapitulation::SectionFeatures;
use topo_sonata_rs::tension::TensionLandscape;
use optimal_transport_agents_rs::distribution::AgentDistribution;
use optimal_transport_agents_rs::sinkhorn::sinkhorn;

let sonata = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
let landscape = TensionLandscape::from_sonata(&sonata);

let exp_feats: Vec<SectionFeatures> = sonata.exposition()
    .iter()
    .map(|s| SectionFeatures::from_section(s, landscape.tension_at(s.start_measure).unwrap_or(0.5)))
    .collect();
let rec_feats: Vec<SectionFeatures> = sonata.recapitulation()
    .iter()
    .map(|s| SectionFeatures::from_section(s, landscape.tension_at(s.start_measure).unwrap_or(0.5)))
    .collect();

// Build distributions from feature vectors
let dist_exp = AgentDistribution::from_support_points(
    exp_feats.iter().map(|f| vec![f.duration, f.key, f.tension, f.section_id]).collect(),
    vec![1.0 / exp_feats.len() as f64; exp_feats.len()],
);
let dist_rec = AgentDistribution::from_support_points(
    rec_feats.iter().map(|f| vec![f.duration, f.key, f.tension, f.section_id]).collect(),
    vec![1.0 / rec_feats.len() as f64; rec_feats.len()],
);

let (plan, cost) = sinkhorn(&dist_exp, &dist_rec, 0.01, 1000).unwrap();
println!("Optimal transport recapitulation cost: {:.4}", cost);
```

### 5. Spectral Prosody for Vocal Sonata Analysis

When analyzing vocal works (e.g., Lieder), combine `topo-sonata-rs` structural analysis with `spectral-prosody-rs` to correlate musical tension landscapes with the singer's prosodic energy.

```rust
use topo_sonata_rs::tension::TensionLandscape;
use topo_sonata_rs::form::SonataForm;
use spectral_prosody_rs::energy::{extract_energy_envelope, EnergyEnvelope};

let sonata = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
let landscape = TensionLandscape::from_sonata(&sonata);
let vocal_envelope = extract_energy_envelope(&vocal_signal, 16000.0, 800, 400);

// Correlate musical tension peaks with vocal energy peaks
for peak in landscape.peaks() {
    let measure = peak;
    let time_sec = measure as f64 * 2.0; // ~2 seconds per measure
    let frame_idx = (time_sec / vocal_envelope.frame_shift) as usize;
    if frame_idx < vocal_envelope.len() {
        println!("Peak at m{}: musical tension {:.2}, vocal energy {:.2}",
            measure, landscape.profile[measure].tension, vocal_envelope.values[frame_idx]);
    }
}
```

### 6. Fleet Budget Auditing via si-fleet-api

When `topo-sonata-rs` runs as a fleet service analyzing large corpora, use `si-fleet-api` to track and audit computational budgets.

```bash
# Query all sonata-analysis agents
curl -s "https://fleet-api.example.com/api/fleet/budgets?service=sonata-analysis" | jq '.'

# Transfer budget from low-priority to high-priority analysis
curl -X POST "https://fleet-api.example.com/api/fleet/transfer" \
  -H "Content-Type: application/json" \
  -d '{"from_agent":"sonata-batch-1","to_agent":"sonata-realtime","amount":50.0}'

# Run conservation audit
curl -s "https://fleet-api.example.com/api/fleet/audit" | jq '.violations'
```

### 7. Ecosystem Dashboard Monitoring

The `ecosystem-dashboard` can display live sonata analysis metrics: tension landscapes, motivic complex statistics, and recapitulation fidelity scores across the fleet.

```javascript
// From ecosystem-dashboard/index.html
async function loadSonataMetrics() {
  const stats = await apiFetch('fleet_events', 'event_type=eq.sonata-analysis');
  renderTensionChart(stats);
  renderMotifCloud(stats);
  renderRecapitulationGauge(stats);
}
```

---

## Design Patterns

### Symbolic-First Architecture

All analysis operates on symbolic representations (measures, keys, feature vectors) rather than raw audio. This makes the crate fast, deterministic, and independent of audio parsing libraries.

### Section-Type Enumeration

The `SectionType` enum encodes the grammatical structure of sonata form. This type system prevents invalid form constructions at compile time.

### Topological Abstraction

Motivic analysis uses Vietoris-Rips complexes and persistent homology — the same tools used in `witness-topology-rs`. This creates a shared mathematical vocabulary across the ecosystem.

---

## Integration Checklist

- [ ] Add `topo-sonata-rs` to `Cargo.toml` of consuming crate
- [ ] Define piece structure with `SonataForm::standard()` or `SonataForm::new()`
- [ ] Connect `TensionLandscape` to visualization pipeline
- [ ] Feed `MotivicComplex` outputs to `witness-topology-rs` for persistent homology
- [ ] Use `optimal-transport-agents-rs` for accurate recapitulation Wasserstein distances
- [ ] Register capabilities in `CAPABILITY.toml` for fleet discovery
- [ ] Audit computational budgets via `si-fleet-api` at scale
- [ ] Monitor via `ecosystem-dashboard` for fleet-wide metrics

---

## Related Repositories

| Repository | Integration Point |
|---|---|
| `conservation-law-rs` | Energy budget conservation for analysis pipelines |
| `witness-topology-rs` | Persistent homology on motivic feature clouds |
| `optimal-transport-agents-rs` | Accurate Wasserstein recapitulation comparison |
| `spectral-prosody-rs` | Prosodic analysis for vocal/instrumental works |
| `renormalization-group-rs` | Multi-scale structural analysis |
| `si-cli` | Capability scanning and fleet registry |
| `si-fleet-api` | Fleet budget auditing and transfer |
| `ecosystem-dashboard` | Live monitoring of analysis metrics |
