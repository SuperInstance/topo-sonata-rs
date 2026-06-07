//! Recapitulation similarity measurement using Wasserstein distance.
//!
//! Measures how well the recapitulation mirrors the exposition.

use crate::form::{SectionType, SonataForm};

/// A feature vector for a section comparison.
#[derive(Debug, Clone)]
pub struct SectionFeatures {
    /// Duration in measures.
    pub duration: f64,
    /// Key (circle of fifths position).
    pub key: f64,
    /// Tension level.
    pub tension: f64,
    /// Section type encoding.
    pub section_id: f64,
}

impl SectionFeatures {
    /// Extract features from a section.
    pub fn from_section(
        section: &crate::form::Section,
        tension: f64,
    ) -> Self {
        let section_id = match section.section_type {
            SectionType::ExpositionTheme1 | SectionType::RecapitulationTheme1 => 1.0,
            SectionType::ExpositionTransition | SectionType::RecapitulationTransition => 2.0,
            SectionType::ExpositionTheme2 | SectionType::RecapitulationTheme2 => 3.0,
            SectionType::ExpositionClosing => 4.0,
            _ => 0.0,
        };

        Self {
            duration: section.duration() as f64,
            key: section.key as f64,
            tension,
            section_id,
        }
    }

    /// Euclidean distance to another feature vector.
    pub fn distance(&self, other: &SectionFeatures) -> f64 {
        let dd = (self.duration - other.duration).powi(2);
        let dk = (self.key - other.key).powi(2);
        let dt = (self.tension - other.tension).powi(2);
        let ds = (self.section_id - other.section_id).powi(2);
        (dd + dk + dt + ds).sqrt()
    }
}

/// Compute the 1-Wasserstein distance between exposition and recapitulation.
///
/// Uses a greedy matching (approximate, not optimal transport).
/// Matches corresponding sections (theme1↔theme1, transition↔transition, theme2↔theme2).
pub fn wasserstein_distance(
    exposition_features: &[SectionFeatures],
    recapitulation_features: &[SectionFeatures],
) -> f64 {
    if exposition_features.is_empty() || recapitulation_features.is_empty() {
        return f64::INFINITY;
    }

    // Match by section_id
    let mut total_distance = 0.0;
    let mut matched = 0;

    for exp_feature in exposition_features {
        // Find the best matching recap feature with the same section_id
        if let Some(best_match) = recapitulation_features
            .iter()
            .filter(|r| r.section_id == exp_feature.section_id)
            .min_by(|a, b| {
                exp_feature
                    .distance(a)
                    .partial_cmp(&exp_feature.distance(b))
                    .unwrap()
            })
        {
            total_distance += exp_feature.distance(best_match);
            matched += 1;
        }
    }

    if matched == 0 {
        f64::INFINITY
    } else {
        total_distance / matched as f64
    }
}

/// Full recapitulation analysis.
#[derive(Debug, Clone)]
pub struct RecapitulationAnalysis {
    /// Wasserstein distance between exposition and recapitulation.
    pub wasserstein_dist: f64,
    /// Duration ratio (recap/exp).
    pub duration_ratio: f64,
    /// Key fidelity: fraction of recap sections in the original key.
    pub key_fidelity: f64,
    /// Section-by-section comparison.
    pub section_comparisons: Vec<(String, f64)>,
}

/// Perform a complete recapitulation analysis.
pub fn analyze_recapitulation(sonata: &SonataForm) -> RecapitulationAnalysis {
    let exp_sections = sonata.exposition();
    let rec_sections = sonata.recapitulation();

    // Build feature vectors
    let exp_features: Vec<SectionFeatures> = exp_sections
        .iter()
        .map(|s| SectionFeatures::from_section(s, 0.5)) // simplified tension
        .collect();
    let rec_features: Vec<SectionFeatures> = rec_sections
        .iter()
        .map(|s| SectionFeatures::from_section(s, 0.5))
        .collect();

    let wasserstein = wasserstein_distance(&exp_features, &rec_features);

    // Duration ratio
    let exp_duration: usize = exp_sections.iter().map(|s| s.duration()).sum();
    let rec_duration: usize = rec_sections.iter().map(|s| s.duration()).sum();
    let duration_ratio = if exp_duration > 0 {
        rec_duration as f64 / exp_duration as f64
    } else {
        0.0
    };

    // Key fidelity
    let tonic_key = sonata
        .sections
        .first()
        .map(|s| s.key)
        .unwrap_or(0);
    let rec_in_tonic = rec_sections
        .iter()
        .filter(|s| s.key == tonic_key)
        .count();
    let key_fidelity = if !rec_sections.is_empty() {
        rec_in_tonic as f64 / rec_sections.len() as f64
    } else {
        0.0
    };

    // Section comparisons
    let mut section_comparisons = Vec::new();
    for exp in &exp_sections {
        let name = format!("{:?}", exp.section_type);
        let exp_feat = SectionFeatures::from_section(exp, 0.5);
        let best_dist = rec_features
            .iter()
            .filter(|r| r.section_id == exp_feat.section_id)
            .map(|r| exp_feat.distance(r))
            .fold(f64::INFINITY, f64::min);
        section_comparisons.push((name, best_dist));
    }

    RecapitulationAnalysis {
        wasserstein_dist: wasserstein,
        duration_ratio,
        key_fidelity,
        section_comparisons,
    }
}

/// Compare two sections directly.
pub fn compare_sections(
    section_a: &crate::form::Section,
    section_b: &crate::form::Section,
    tension_a: f64,
    tension_b: f64,
) -> f64 {
    let feat_a = SectionFeatures::from_section(section_a, tension_a);
    let feat_b = SectionFeatures::from_section(section_b, tension_b);
    feat_a.distance(&feat_b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{Section, SonataForm};

    fn standard_sonata() -> SonataForm {
        SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0)
    }

    #[test]
    fn test_section_features_distance() {
        let a = SectionFeatures {
            duration: 16.0,
            key: 0.0,
            tension: 0.5,
            section_id: 1.0,
        };
        let b = SectionFeatures {
            duration: 16.0,
            key: 0.0,
            tension: 0.5,
            section_id: 1.0,
        };
        assert!(a.distance(&b).abs() < 1e-10);
    }

    #[test]
    fn test_section_features_different() {
        let a = SectionFeatures {
            duration: 16.0,
            key: 0.0,
            tension: 0.5,
            section_id: 1.0,
        };
        let b = SectionFeatures {
            duration: 20.0,
            key: 5.0,
            tension: 0.5,
            section_id: 1.0,
        };
        assert!(a.distance(&b) > 0.0);
    }

    #[test]
    fn test_wasserstein_identical() {
        let features = vec![SectionFeatures {
            duration: 16.0,
            key: 0.0,
            tension: 0.5,
            section_id: 1.0,
        }];
        let dist = wasserstein_distance(&features, &features);
        assert!(dist.abs() < 1e-10);
    }

    #[test]
    fn test_wasserstein_empty() {
        let features = vec![SectionFeatures {
            duration: 16.0,
            key: 0.0,
            tension: 0.5,
            section_id: 1.0,
        }];
        let dist = wasserstein_distance(&features, &[]);
        assert!(dist.is_infinite());
    }

    #[test]
    fn test_recapitulation_analysis() {
        let sonata = standard_sonata();
        let analysis = analyze_recapitulation(&sonata);
        assert!(analysis.wasserstein_dist >= 0.0);
        assert!(analysis.duration_ratio > 0.0);
        assert!(analysis.key_fidelity >= 0.0 && analysis.key_fidelity <= 1.0);
    }

    #[test]
    fn test_compare_sections() {
        let a = Section::new(SectionType::ExpositionTheme1, 1, 16, 0);
        let b = Section::new(SectionType::RecapitulationTheme1, 71, 86, 0);
        let dist = compare_sections(&a, &b, 0.5, 0.5);
        assert!(dist >= 0.0);
    }

    #[test]
    fn test_key_fidelity() {
        let sonata = standard_sonata();
        let analysis = analyze_recapitulation(&sonata);
        // In our standard sonata, recapitulation theme2 is in the original key
        assert!(analysis.key_fidelity > 0.0);
    }

    #[test]
    fn test_duration_ratio() {
        let sonata = standard_sonata();
        let analysis = analyze_recapitulation(&sonata);
        // Should be close to 1.0 for a standard sonata
        assert!(analysis.duration_ratio > 0.5 && analysis.duration_ratio < 2.0);
    }
}
