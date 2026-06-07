//! Motivic analysis using persistent homology.
//!
//! Represents musical motifs as points in a feature space and applies
//! topological analysis to discover motivic structure.

/// A musical motif represented as a point in feature space.
#[derive(Debug, Clone)]
pub struct Motif {
    /// Unique identifier.
    pub id: usize,
    /// Feature vector (e.g., intervallic content, rhythm, contour).
    pub features: Vec<f64>,
    /// Starting measure.
    pub measure: usize,
}

impl Motif {
    /// Create a new motif.
    pub fn new(id: usize, features: Vec<f64>, measure: usize) -> Self {
        Self { id, features, measure }
    }

    /// Euclidean distance to another motif in feature space.
    pub fn distance(&self, other: &Motif) -> f64 {
        self.features
            .iter()
            .zip(other.features.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

/// A simple Vietoris-Rips complex built from motifs.
#[derive(Debug, Clone)]
pub struct MotivicComplex {
    /// Motifs used to build the complex.
    pub motifs: Vec<Motif>,
    /// Edges (pairs of motif indices).
    pub edges: Vec<(usize, usize)>,
    /// Triangles (triples of motif indices).
    pub triangles: Vec<(usize, usize, usize)>,
}

impl MotivicComplex {
    /// Build a Vietoris-Rips complex up to dimension 2 with given epsilon.
    pub fn build(motifs: &[Motif], epsilon: f64) -> Self {
        let n = motifs.len();
        let mut edges = Vec::new();
        let mut triangles = Vec::new();

        // Add edges
        for i in 0..n {
            for j in (i + 1)..n {
                if motifs[i].distance(&motifs[j]) <= epsilon {
                    edges.push((i, j));
                }
            }
        }

        // Add triangles (if all three edges exist)
        let edge_set: std::collections::HashSet<(usize, usize)> =
            edges.iter().cloned().collect();
        for i in 0..n {
            for j in (i + 1)..n {
                for k in (j + 1)..n {
                    let e1 = (i.min(j), i.max(j));
                    let e2 = (i.min(k), i.max(k));
                    let e3 = (j.min(k), j.max(k));
                    if edge_set.contains(&e1) && edge_set.contains(&e2) && edge_set.contains(&e3) {
                        triangles.push((i, j, k));
                    }
                }
            }
        }

        Self {
            motifs: motifs.to_vec(),
            edges,
            triangles,
        }
    }

    /// Number of connected components.
    pub fn num_components(&self) -> usize {
        let n = self.motifs.len();
        if n == 0 {
            return 0;
        }

        let mut parent: Vec<usize> = (0..n).collect();

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        for &(a, b) in &self.edges {
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            parent[ra] = rb;
        }

        let mut roots = std::collections::HashSet::new();
        for i in 0..n {
            roots.insert(find(&mut parent, i));
        }
        roots.len()
    }

    /// Euler characteristic.
    pub fn euler_characteristic(&self) -> i64 {
        let v = self.motifs.len() as i64;
        let e = self.edges.len() as i64;
        let t = self.triangles.len() as i64;
        v - e + t
    }
}

/// A persistence diagram entry for motivic analysis.
#[derive(Debug, Clone)]
pub struct MotivicPersistencePair {
    /// Dimension.
    pub dim: usize,
    /// Birth epsilon.
    pub birth: f64,
    /// Death epsilon.
    pub death: Option<f64>,
}

/// Compute motivic persistence (simplified).
/// Returns pairs for H0 (connected components merging).
pub fn motivic_persistence(motifs: &[Motif], max_epsilon: f64, _steps: usize) -> Vec<MotivicPersistencePair> {
    let mut pairs = Vec::new();
    let n = motifs.len();

    if n == 0 {
        return pairs;
    }

    // Compute all pairwise distances
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            distances.push((i, j, motifs[i].distance(&motifs[j])));
        }
    }
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    // Union-Find for H0
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank = vec![0usize; n];
    let mut birth: Vec<Option<f64>> = vec![None; n]; // birth of component

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for b in birth.iter_mut().take(n) {
        *b = Some(0.0);
    }

    let mut _num_components = n;

    for (a, b, d) in &distances {
        if *d > max_epsilon {
            break;
        }
        let ra = find(&mut parent, *a);
        let rb = find(&mut parent, *b);
        if ra != rb {
            // Merge: the component with higher rank absorbs
            let (winner, loser) = if rank[ra] >= rank[rb] { (ra, rb) } else { (rb, ra) };
            parent[loser] = winner;
            if rank[ra] == rank[rb] {
                rank[winner] += 1;
            }
            pairs.push(MotivicPersistencePair {
                dim: 0,
                birth: 0.0,
                death: Some(*d),
            });
            _num_components -= 1;
        }
    }

    // Remaining components are essential
    let mut roots = std::collections::HashSet::new();
    for i in 0..n {
        roots.insert(find(&mut parent, i));
    }

    for _ in roots {
        pairs.push(MotivicPersistencePair {
            dim: 0,
            birth: 0.0,
            death: None,
        });
    }

    pairs
}

/// Cluster motifs by density at a given epsilon.
pub fn cluster_motifs(motifs: &[Motif], epsilon: f64) -> Vec<Vec<usize>> {
    let complex = MotivicComplex::build(motifs, epsilon);
    let n = motifs.len();

    let mut parent: Vec<usize> = (0..n).collect();

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for &(a, b) in &complex.edges {
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        parent[ra] = rb;
    }

    let mut clusters: std::collections::HashMap<usize, Vec<usize>> =
        std::collections::HashMap::new();
    for i in 0..n {
        let root = find(&mut parent, i);
        clusters.entry(root).or_default().push(i);
    }

    let mut result: Vec<Vec<usize>> = clusters.into_values().collect();
    result.sort_by_key(|c| c[0]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motif_distance() {
        let a = Motif::new(0, vec![0.0, 0.0], 1);
        let b = Motif::new(1, vec![3.0, 4.0], 5);
        assert!((a.distance(&b) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_rips_complex() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![1.0], 2),
            Motif::new(2, vec![2.0], 3),
        ];
        let complex = MotivicComplex::build(&motifs, 1.5);
        assert_eq!(complex.edges.len(), 2); // (0,1) and (1,2)
        assert_eq!(complex.triangles.len(), 0);
    }

    #[test]
    fn test_rips_complex_triangle() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![1.0], 2),
            Motif::new(2, vec![0.5], 3),
        ];
        let complex = MotivicComplex::build(&motifs, 1.5);
        assert_eq!(complex.edges.len(), 3);
        assert_eq!(complex.triangles.len(), 1);
    }

    #[test]
    fn test_num_components() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![1.0], 2),
            Motif::new(2, vec![10.0], 3),
        ];
        let complex = MotivicComplex::build(&motifs, 2.0);
        assert_eq!(complex.num_components(), 2);
    }

    #[test]
    fn test_euler_characteristic_motif() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![1.0], 2),
            Motif::new(2, vec![0.5], 3),
        ];
        let complex = MotivicComplex::build(&motifs, 1.5);
        assert_eq!(complex.euler_characteristic(), 3 - 3 + 1);
    }

    #[test]
    fn test_motivic_persistence() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![1.0], 2),
            Motif::new(2, vec![5.0], 3),
        ];
        let pairs = motivic_persistence(&motifs, 10.0, 10);
        // Should have at least one death (merge) and one essential class
        assert!(pairs.len() >= 2);
    }

    #[test]
    fn test_cluster_motifs() {
        let motifs = vec![
            Motif::new(0, vec![0.0], 1),
            Motif::new(1, vec![0.5], 2),
            Motif::new(2, vec![10.0], 3),
        ];
        let clusters = cluster_motifs(&motifs, 1.0);
        assert_eq!(clusters.len(), 2);
    }

    #[test]
    fn test_empty_motifs() {
        let complex = MotivicComplex::build(&[], 1.0);
        assert_eq!(complex.edges.len(), 0);
        assert_eq!(complex.num_components(), 0);
    }
}
