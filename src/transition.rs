//! Topological transitions between sonata sections.
//!
//! Models key areas and harmonic regions as points in a topological space,
//! where transitions between sections are continuous paths.

use crate::form::SonataForm;

/// A point in tonal space (circle of fifths + mode).
#[derive(Debug, Clone)]
pub struct TonalPoint {
    /// Position on circle of fifths (0=C, 1=G, -1=F, etc.)
    pub fifths: i32,
    /// Mode: 0 = major, 1 = minor.
    pub mode: u8,
}

impl TonalPoint {
    /// Create a new tonal point.
    pub fn new(fifths: i32, mode: u8) -> Self {
        Self { fifths, mode }
    }

    /// Distance on the circle of fifths (wrapping at 12).
    pub fn circle_distance(&self, other: &TonalPoint) -> f64 {
        let diff = (self.fifths - other.fifths).abs() % 12;
        let wrapped = diff.min(12 - diff);
        wrapped as f64
    }

    /// Full distance including mode difference.
    pub fn distance(&self, other: &TonalPoint) -> f64 {
        let fifths_dist = self.circle_distance(other);
        let mode_penalty = if self.mode != other.mode { 0.5 } else { 0.0 };
        fifths_dist + mode_penalty
    }
}

/// A transition between two tonal regions.
#[derive(Debug, Clone)]
pub struct TonalTransition {
    /// Source tonal region.
    pub from: TonalPoint,
    /// Target tonal region.
    pub to: TonalPoint,
    /// Length of transition path (in circle-of-fifths distance).
    pub path_length: f64,
    /// Whether the transition passes through intermediate keys.
    pub intermediates: Vec<TonalPoint>,
}

/// Compute the tonal transition between two keys.
pub fn compute_transition(from_key: i32, to_key: i32) -> TonalTransition {
    let from = TonalPoint::new(from_key, 0);
    let to = TonalPoint::new(to_key, 0);

    let direct_dist = from.circle_distance(&to);

    let mut intermediates = Vec::new();
    let diff = to.fifths - from.fifths;
    let steps = if diff.abs() <= 6 { diff } else if diff > 0 { diff - 12 } else { diff + 12 };

    if steps.abs() > 1 {
        let direction = steps.signum();
        let n_steps = steps.abs();
        for i in 1..n_steps {
            intermediates.push(TonalPoint::new(from.fifths + direction * i, 0));
        }
    }

    TonalTransition {
        path_length: direct_dist,
        intermediates,
        from,
        to,
    }
}

/// Build a transition map for all section boundaries in a sonata.
pub fn transition_map(sonata: &SonataForm) -> Vec<TonalTransition> {
    let mut transitions = Vec::new();
    for i in 0..sonata.sections.len().saturating_sub(1) {
        let from = &sonata.sections[i];
        let to = &sonata.sections[i + 1];
        transitions.push(compute_transition(from.key, to.key));
    }
    transitions
}

/// Compute the total tonal distance traveled in a sonata.
pub fn total_tonal_distance(sonata: &SonataForm) -> f64 {
    transition_map(sonata).iter().map(|t| t.path_length).sum()
}

/// Classify the type of transition.
#[derive(Debug, Clone, PartialEq)]
pub enum TransitionType {
    /// No key change.
    Static,
    /// Move to dominant (up a fifth).
    ToDominant,
    /// Move to subdominant (down a fifth).
    ToSubdominant,
    /// Move to relative key.
    Relative,
    /// Distant modulation.
    Distant,
}

/// Classify a transition between two keys.
pub fn classify_transition(from_key: i32, to_key: i32) -> TransitionType {
    let diff = (to_key - from_key).abs() % 12;
    if diff == 0 {
        TransitionType::Static
    } else if diff == 1 || diff == 11 {
        TransitionType::ToDominant
    } else if diff == 5 || diff == 7 {
        TransitionType::ToSubdominant
    } else if diff == 3 || diff == 9 {
        TransitionType::Relative
    } else {
        TransitionType::Distant
    }
}

/// Topological complexity of the tonal path through a sonata.
pub fn tonal_complexity(sonata: &SonataForm) -> f64 {
    let mut total_change = 0.0;
    for i in 0..sonata.sections.len().saturating_sub(1) {
        let from = sonata.sections[i].key;
        let to = sonata.sections[i + 1].key;
        let diff = to - from;
        let normalized = if diff > 6 {
            diff - 12
        } else if diff < -6 {
            diff + 12
        } else {
            diff
        };
        total_change += normalized.abs() as f64;
    }
    total_change
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{Section, SectionType};

    #[test]
    fn test_tonal_point_distance() {
        let c = TonalPoint::new(0, 0);
        let g = TonalPoint::new(1, 0);
        assert!((c.circle_distance(&g) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tonal_point_wrapping() {
        let f_sharp = TonalPoint::new(6, 0);
        let g_flat = TonalPoint::new(-6, 0);
        let dist = f_sharp.circle_distance(&g_flat);
        assert!(dist.abs() < 1e-10);
    }

    #[test]
    fn test_mode_distance() {
        let c_maj = TonalPoint::new(0, 0);
        let c_min = TonalPoint::new(0, 1);
        assert!((c_maj.distance(&c_min) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_compute_transition() {
        let t = compute_transition(0, 2);
        assert!((t.path_length - 2.0).abs() < 1e-10);
        assert_eq!(t.intermediates.len(), 1);
    }

    #[test]
    fn test_transition_no_intermediate() {
        let t = compute_transition(0, 1);
        assert!(t.intermediates.is_empty());
    }

    #[test]
    fn test_classify_static() {
        assert_eq!(classify_transition(0, 0), TransitionType::Static);
    }

    #[test]
    fn test_classify_dominant() {
        assert_eq!(classify_transition(0, 1), TransitionType::ToDominant);
    }

    #[test]
    fn test_classify_distant() {
        assert_eq!(classify_transition(0, 4), TransitionType::Distant);
    }

    #[test]
    fn test_transition_map() {
        let sonata = SonataForm::new(vec![
            Section::new(SectionType::ExpositionTheme1, 1, 10, 0),
            Section::new(SectionType::ExpositionTheme2, 11, 20, 1),
            Section::new(SectionType::Development, 21, 40, 0),
        ]);
        let map = transition_map(&sonata);
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_tonal_complexity() {
        let sonata = SonataForm::new(vec![
            Section::new(SectionType::ExpositionTheme1, 1, 10, 0),
            Section::new(SectionType::ExpositionTheme2, 11, 20, 5),
        ]);
        let complexity = tonal_complexity(&sonata);
        assert!(complexity > 0.0);
    }
}
