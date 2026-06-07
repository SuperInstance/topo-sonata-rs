//! Harmonic tension landscape.
//!
//! Models harmonic tension as a function of position in the sonata,
//! creating a landscape that can be analyzed topologically.

use crate::form::SonataForm;

/// A tension value at a specific measure.
#[derive(Debug, Clone)]
pub struct TensionPoint {
    /// Measure number.
    pub measure: usize,
    /// Tension value (higher = more tense).
    pub tension: f64,
}

/// A harmonic tension profile for a piece.
#[derive(Debug, Clone)]
pub struct TensionLandscape {
    /// Tension values by measure.
    pub profile: Vec<TensionPoint>,
}

impl TensionLandscape {
    /// Create a tension landscape from raw values.
    pub fn new(values: &[(usize, f64)]) -> Self {
        Self {
            profile: values
                .iter()
                .map(|(m, t)| TensionPoint {
                    measure: *m,
                    tension: *t,
                })
                .collect(),
        }
    }

    /// Generate a synthetic tension landscape for a sonata form.
    /// Uses a simple model based on section types.
    pub fn from_sonata(sonata: &SonataForm) -> Self {
        let mut profile = Vec::new();

        for section in &sonata.sections {
            let (base_tension, slope) = match section.section_type {
                crate::form::SectionType::ExpositionTheme1 => (0.3, 0.0),
                crate::form::SectionType::ExpositionTransition => (0.3, 0.4),
                crate::form::SectionType::ExpositionTheme2 => (0.5, 0.0),
                crate::form::SectionType::ExpositionClosing => (0.4, -0.1),
                crate::form::SectionType::Development => (0.6, 0.3),
                crate::form::SectionType::RecapitulationTheme1 => (0.3, 0.0),
                crate::form::SectionType::RecapitulationTransition => (0.3, 0.3),
                crate::form::SectionType::RecapitulationTheme2 => (0.4, 0.0),
                crate::form::SectionType::Coda => (0.3, -0.2),
                crate::form::SectionType::Introduction => (0.2, 0.1),
            };

            let duration = section.duration() as f64;
            for i in 0..section.duration() {
                let t = base_tension + slope * (i as f64 / duration.max(1.0));
                profile.push(TensionPoint {
                    measure: section.start_measure + i,
                    tension: t.max(0.0),
                });
            }
        }

        Self { profile }
    }

    /// Get tension at a specific measure (linear interpolation).
    pub fn tension_at(&self, measure: usize) -> Option<f64> {
        // Find bracketing points
        let idx = self.profile.iter().position(|p| p.measure >= measure)?;

        if self.profile[idx].measure == measure {
            return Some(self.profile[idx].tension);
        }

        if idx == 0 {
            return Some(self.profile[0].tension);
        }

        let prev = &self.profile[idx - 1];
        let next = &self.profile[idx];
        let t = (measure - prev.measure) as f64 / (next.measure - prev.measure) as f64;
        Some(prev.tension * (1.0 - t) + next.tension * t)
    }

    /// Maximum tension.
    pub fn max_tension(&self) -> f64 {
        self.profile
            .iter()
            .map(|p| p.tension)
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Minimum tension.
    pub fn min_tension(&self) -> f64 {
        self.profile
            .iter()
            .map(|p| p.tension)
            .fold(f64::INFINITY, f64::min)
    }

    /// Average tension.
    pub fn average_tension(&self) -> f64 {
        if self.profile.is_empty() {
            return 0.0;
        }
        self.profile.iter().map(|p| p.tension).sum::<f64>() / self.profile.len() as f64
    }

    /// Find local maxima (tension peaks).
    pub fn peaks(&self) -> Vec<usize> {
        let mut peaks = Vec::new();
        for i in 1..self.profile.len().saturating_sub(1) {
            if self.profile[i].tension > self.profile[i - 1].tension
                && self.profile[i].tension > self.profile[i + 1].tension
            {
                peaks.push(i);
            }
        }
        peaks
    }

    /// Find local minima (tension valleys).
    pub fn valleys(&self) -> Vec<usize> {
        let mut valleys = Vec::new();
        for i in 1..self.profile.len().saturating_sub(1) {
            if self.profile[i].tension < self.profile[i - 1].tension
                && self.profile[i].tension < self.profile[i + 1].tension
            {
                valleys.push(i);
            }
        }
        valleys
    }

    /// Compute the tension arc: area under the tension curve.
    pub fn tension_arc(&self) -> f64 {
        let mut total = 0.0;
        for i in 1..self.profile.len() {
            let dx = (self.profile[i].measure - self.profile[i - 1].measure) as f64;
            let avg_y = (self.profile[i].tension + self.profile[i - 1].tension) / 2.0;
            total += dx * avg_y;
        }
        total
    }

    /// Number of measures in the landscape.
    pub fn len(&self) -> usize {
        self.profile.len()
    }

    /// Check if landscape is empty.
    pub fn is_empty(&self) -> bool {
        self.profile.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::form::{Section, SectionType};

    fn simple_sonata() -> SonataForm {
        SonataForm::new(vec![
            Section::new(SectionType::ExpositionTheme1, 1, 10, 0),
            Section::new(SectionType::Development, 11, 30, 0),
            Section::new(SectionType::RecapitulationTheme1, 31, 40, 0),
        ])
    }

    #[test]
    fn test_tension_landscape_from_sonata() {
        let sonata = simple_sonata();
        let landscape = TensionLandscape::from_sonata(&sonata);
        assert_eq!(landscape.len(), 40);
    }

    #[test]
    fn test_tension_at_measure() {
        let sonata = simple_sonata();
        let landscape = TensionLandscape::from_sonata(&sonata);
        let t = landscape.tension_at(5);
        assert!(t.is_some());
        assert!(t.unwrap() >= 0.0);
    }

    #[test]
    fn test_max_min_tension() {
        let landscape = TensionLandscape::new(&[(1, 0.5), (2, 1.0), (3, 0.3)]);
        assert!((landscape.max_tension() - 1.0).abs() < 1e-10);
        assert!((landscape.min_tension() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_average_tension() {
        let landscape = TensionLandscape::new(&[(1, 0.0), (2, 1.0)]);
        assert!((landscape.average_tension() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_peaks() {
        let landscape = TensionLandscape::new(&[(1, 0.5), (2, 1.0), (3, 0.5)]);
        let peaks = landscape.peaks();
        assert_eq!(peaks, vec![1]);
    }

    #[test]
    fn test_valleys() {
        let landscape = TensionLandscape::new(&[(1, 1.0), (2, 0.5), (3, 1.0)]);
        let valleys = landscape.valleys();
        assert_eq!(valleys, vec![1]);
    }

    #[test]
    fn test_tension_arc() {
        let landscape = TensionLandscape::new(&[(1, 1.0), (2, 1.0)]);
        let arc = landscape.tension_arc();
        assert!((arc - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_landscape() {
        let landscape = TensionLandscape::new(&[]);
        assert!(landscape.is_empty());
        assert_eq!(landscape.len(), 0);
        assert!((landscape.average_tension()).abs() < 1e-10);
    }
}
