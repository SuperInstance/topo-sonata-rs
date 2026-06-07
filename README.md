# topo-sonata-rs

**Topological analysis of musical sonata form.**

This crate applies algebraic topology to musical structure: model sonata form as sections with tonal coordinates, compute topological transitions between keys on the circle of fifths, build Vietoris-Rips complexes from motivic feature vectors, trace harmonic tension landscapes across the piece, and measure recapitulation similarity using Wasserstein distances. With 43 tests covering form parsing, motivic analysis, tension modeling, transition computation, and recapitulation matching, it demonstrates that topology isn't abstract — it's the natural language for understanding musical architecture.

## Why This Matters

Music is organized time — and topology is the mathematics of organized structure. Sonata form is one of the most sophisticated organizational schemes in human culture: exposition presents themes, development transforms them, recapitulation returns them transformed. This crate makes that structure computable. Motivic complexes reveal how themes relate. Tension landscapes quantify emotional arcs. Transition topology maps the journey through tonal space. Wasserstein distances measure how faithfully the recapitulation mirrors the exposition. For an AGI system, this is a concrete demonstration that topological reasoning captures *meaningful* structure in temporal data — applicable far beyond music to any system with sections, transitions, and recurrence.

## Quick Start

```toml
# Cargo.toml
[dependencies]
topo-sonata-rs = "0.1.0"
```

```rust
use topo_sonata_rs::form::{SonataForm, Section, SectionType};
use topo_sonata_rs::tension::TensionLandscape;
use topo_sonata_rs::transition::{transition_map, TonalPoint};
use topo_sonata_rs::motif::{Motif, MotivicComplex};
use topo_sonata_rs::recapitulation::{wasserstein_distance, SectionFeatures};

// Define a sonata form
let sonata = SonataForm::standard(
    1, 40,   // Exposition: measures 1-40
    41, 80,  // Development: measures 41-80
    81, 120, // Recapitulation: measures 81-120
    0,       // Key: C major
    2,       // Secondary key: D major
);

// Generate a tension landscape
let landscape = TensionLandscape::from_sonata(&sonata);
println!("Tension at measure 50: {:.2}", landscape.tension_at(50).unwrap());
let peaks = landscape.peaks();
println!("Tension peaks: {:?}", peaks.iter().map(|p| p.measure).collect::<Vec<_>>());

// Compute tonal transitions between sections
let transitions = transition_map(&sonata);
for t in &transitions {
    println!("Key {} → {}: distance {:.1}", t.from.fifths, t.to.fifths, t.path_length);
}

// Build a motivic complex from themes
let theme_a = vec![
    Motif::new(0, vec![1.0, 0.0, 2.0], 1),
    Motif::new(1, vec![1.1, 0.1, 1.9], 5),
    Motif::new(2, vec![3.0, 1.0, 0.0], 41),
];
let complex = MotivicComplex::build(&theme_a, 2.0);
println!("Motivic edges: {} (connected themes)", complex.edges.len());
```

## Architecture

| Module | Purpose |
|---|---|
| `form` | Sonata form structure — sections, timing, keys, standard templates |
| `transition` | Tonal transitions on the circle of fifths, intermediate key computation |
| `motif` | Motivic analysis via Vietoris-Rips complexes, topological motif clustering |
| `tension` | Harmonic tension landscapes — synthetic generation, peak detection |
| `recapitulation` | Exposition-recapitulation comparison via Wasserstein distance |

## API Tour

### Sonata Form (`form`)

- **`SectionType`** — Enum: `Introduction`, `ExpositionTheme1`, `ExpositionTransition`, `ExpositionTheme2`, `ExpositionClosing`, `Development`, `RecapitulationTheme1`, `RecapitulationTransition`, `RecapitulationTheme2`, `Coda`
- **`Section { section_type, start_measure, end_measure, key }`**
  - `.duration()` — Length in measures
  - `.contains_measure(m)` — Membership test
- **`SonataForm { sections }`**
  - `::new(sections)` — Custom form
  - `::standard(exp_start, exp_end, dev_start, dev_end, recap_start, recap_end, key, secondary_key)` — Template
  - `.section_at(measure) → Option<&Section>` — Look up by position
  - `.exposition_sections()`, `.development_sections()`, `.recapitulation_sections()` — Filtered access

### Tonal Transitions (`transition`)

- **`TonalPoint { fifths, mode }`** — Position on circle of fifths + major/minor
  - `.circle_distance(other)` — Wrapped distance on the circle
  - `.distance(other)` — Including mode penalty
- **`TonalTransition { from, to, path_length, intermediates }`** — A key change
- **`compute_transition(from_key, to_key) → TonalTransition`** — Single transition
- **`transition_map(sonata) → Vec<TonalTransition>`** — All section-boundary transitions

### Motivic Analysis (`motif`)

- **`Motif { id, features, measure }`** — A musical idea in feature space
  - `.distance(other)` — Euclidean distance in feature space
- **`MotivicComplex { motifs, edges, triangles }`** — Vietoris-Rips complex
  - `::build(motifs, epsilon)` — Construct with proximity threshold
  - `.num_vertices()`, `.num_edges()`, `.num_triangles()` — Counts
  - `.euler_characteristic()` — Topological invariant

### Tension Landscape (`tension`)

- **`TensionPoint { measure, tension }`** — A single measurement
- **`TensionLandscape { profile }`** — Full tension curve
  - `::from_sonata(sonata)` — Generate synthetic tension based on section types
  - `::new(raw_values)` — From explicit (measure, tension) pairs
  - `.tension_at(measure) → Option<f64>` — Interpolated lookup
  - `.peaks()` — Local maxima (climactic moments)
  - `.integrate()` — Total tension (area under curve)

### Recapitulation (`recapitulation`)

- **`SectionFeatures { duration, key, tension, section_id }`** — Feature vector per section
  - `::from_section(section, tension)` — Extract from a Section
  - `.distance(other)` — Euclidean distance
- **`wasserstein_distance(exposition, recapitulation) → f64`** — Approximate W1 distance

## Performance

- Form construction: O(sections)
- Tension landscape generation: O(total measures)
- Motivic complex: O(n²) for n motifs (pairwise distances)
- Transition map: O(sections)
- Recapitulation comparison: O(exp × recap) for greedy matching
- All pure Rust, no audio dependencies — works with symbolic data

## Ecosystem

Part of the **SuperInstance** family:

- [`spectral-prosody-rs`](https://github.com/SuperInstance/spectral-prosody-rs) — Spectral analysis of prosodic features
- [`witness-topology-rs`](https://github.com/SuperInstance/witness-topology-rs) — Topological approximation from point clouds
- [`optimal-transport-rs`](https://github.com/SuperInstance/optimal-transport-rs) — Wasserstein distances
- [`sheaf-coherence-rs`](https://github.com/SuperInstance/sheaf-coherence-rs) — Coherence of multi-part structures
- [`renormalization-group-rs`](https://github.com/SuperInstance/renormalization-group-rs) — Multi-scale structural analysis

## Ideas for Improvement

- **Audio input** — Connect to real audio via `symphonia` or `rodio`
- **MIDI parsing** — Direct import from MIDI files using `midly`
- **Persistent motivic homology** — Track motif birth/death across epsilon values
- **Harmonic analysis** — Automated chord detection and Roman numeral analysis
- **Corpus analysis** — Batch processing of sonata collections for statistical patterns
- **Visualization** — SVG tension landscapes and motivic complexes

## License

MIT
