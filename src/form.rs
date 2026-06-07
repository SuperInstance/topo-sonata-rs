//! Sonata form structure — exposition, development, recapitulation.

/// A section of sonata form.
#[derive(Debug, Clone, PartialEq)]
pub enum SectionType {
    /// Introduction (optional).
    Introduction,
    /// First theme group in exposition.
    ExpositionTheme1,
    /// Transition in exposition.
    ExpositionTransition,
    /// Second theme group in exposition.
    ExpositionTheme2,
    /// Closing section of exposition.
    ExpositionClosing,
    /// Development section.
    Development,
    /// First theme group in recapitulation.
    RecapitulationTheme1,
    /// Transition in recapitulation.
    RecapitulationTransition,
    /// Second theme group in recapitulation.
    RecapitulationTheme2,
    /// Coda.
    Coda,
}

/// A musical section with timing information.
#[derive(Debug, Clone)]
pub struct Section {
    /// Type of section.
    pub section_type: SectionType,
    /// Start measure.
    pub start_measure: usize,
    /// End measure (inclusive).
    pub end_measure: usize,
    /// Key (as number of sharps, negative for flats).
    pub key: i32,
}

impl Section {
    /// Create a new section.
    pub fn new(section_type: SectionType, start: usize, end: usize, key: i32) -> Self {
        Self {
            section_type,
            start_measure: start,
            end_measure: end,
            key,
        }
    }

    /// Duration in measures.
    pub fn duration(&self) -> usize {
        if self.end_measure >= self.start_measure {
            self.end_measure - self.start_measure + 1
        } else {
            0
        }
    }

    /// Check if a measure is within this section.
    pub fn contains_measure(&self, measure: usize) -> bool {
        measure >= self.start_measure && measure <= self.end_measure
    }
}

/// A complete sonata form structure.
#[derive(Debug, Clone)]
pub struct SonataForm {
    /// All sections in order.
    pub sections: Vec<Section>,
}

impl SonataForm {
    /// Create a new sonata form from sections.
    pub fn new(sections: Vec<Section>) -> Self {
        Self { sections }
    }

    /// Create a standard sonata form template.
    #[allow(clippy::too_many_arguments)]
    pub fn standard(
        exp1_start: usize,
        exp1_end: usize,
        exp2_start: usize,
        exp2_end: usize,
        dev_start: usize,
        dev_end: usize,
        rec1_start: usize,
        rec1_end: usize,
        rec2_start: usize,
        rec2_end: usize,
        key: i32,
    ) -> Self {
        Self {
            sections: vec![
                Section::new(SectionType::ExpositionTheme1, exp1_start, exp1_end, key),
                Section::new(SectionType::ExpositionTransition, exp1_end + 1, exp2_start - 1, key),
                Section::new(SectionType::ExpositionTheme2, exp2_start, exp2_end, key + 5), // dominant
                Section::new(SectionType::Development, dev_start, dev_end, key),
                Section::new(SectionType::RecapitulationTheme1, rec1_start, rec1_end, key),
                Section::new(SectionType::RecapitulationTransition, rec1_end + 1, rec2_start - 1, key),
                Section::new(SectionType::RecapitulationTheme2, rec2_start, rec2_end, key),
            ],
        }
    }

    /// Total number of measures.
    pub fn total_measures(&self) -> usize {
        self.sections
            .iter()
            .map(|s| s.duration())
            .sum()
    }

    /// Get sections of a specific type.
    pub fn sections_of_type(&self, section_type: &SectionType) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|s| &s.section_type == section_type)
            .collect()
    }

    /// Get exposition sections.
    pub fn exposition(&self) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|s| {
                matches!(
                    s.section_type,
                    SectionType::ExpositionTheme1
                        | SectionType::ExpositionTransition
                        | SectionType::ExpositionTheme2
                        | SectionType::ExpositionClosing
                )
            })
            .collect()
    }

    /// Get development section.
    pub fn development(&self) -> Option<&Section> {
        self.sections
            .iter()
            .find(|s| s.section_type == SectionType::Development)
    }

    /// Get recapitulation sections.
    pub fn recapitulation(&self) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|s| {
                matches!(
                    s.section_type,
                    SectionType::RecapitulationTheme1
                        | SectionType::RecapitulationTransition
                        | SectionType::RecapitulationTheme2
                )
            })
            .collect()
    }

    /// Find which section contains a given measure.
    pub fn section_at_measure(&self, measure: usize) -> Option<&Section> {
        self.sections
            .iter()
            .find(|s| s.contains_measure(measure))
    }

    /// Proportion of the piece that is exposition.
    pub fn exposition_proportion(&self) -> f64 {
        let total = self.total_measures() as f64;
        if total == 0.0 {
            return 0.0;
        }
        let exp: usize = self.exposition().iter().map(|s| s.duration()).sum();
        exp as f64 / total
    }

    /// Proportion of the piece that is development.
    pub fn development_proportion(&self) -> f64 {
        let total = self.total_measures() as f64;
        if total == 0.0 {
            return 0.0;
        }
        self.development()
            .map(|d| d.duration() as f64 / total)
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_duration() {
        let s = Section::new(SectionType::ExpositionTheme1, 1, 16, 0);
        assert_eq!(s.duration(), 16);
    }

    #[test]
    fn test_section_contains_measure() {
        let s = Section::new(SectionType::ExpositionTheme1, 5, 20, 0);
        assert!(s.contains_measure(5));
        assert!(s.contains_measure(20));
        assert!(s.contains_measure(10));
        assert!(!s.contains_measure(4));
        assert!(!s.contains_measure(21));
    }

    #[test]
    fn test_standard_sonata_form() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        assert_eq!(sf.sections.len(), 7);
        assert_eq!(sf.total_measures(), 120);
    }

    #[test]
    fn test_exposition() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let exp = sf.exposition();
        assert_eq!(exp.len(), 3);
    }

    #[test]
    fn test_development() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let dev = sf.development().unwrap();
        assert_eq!(dev.start_measure, 41);
        assert_eq!(dev.end_measure, 70);
    }

    #[test]
    fn test_recapitulation() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let rec = sf.recapitulation();
        assert_eq!(rec.len(), 3);
    }

    #[test]
    fn test_section_at_measure() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let s = sf.section_at_measure(50).unwrap();
        assert_eq!(s.section_type, SectionType::Development);
    }

    #[test]
    fn test_proportions() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let exp = sf.exposition_proportion();
        let dev = sf.development_proportion();
        assert!(exp > 0.0 && exp < 1.0);
        assert!(dev > 0.0 && dev < 1.0);
        assert!((exp + dev) < 1.0);
    }

    #[test]
    fn test_sections_of_type() {
        let sf = SonataForm::standard(1, 16, 20, 40, 41, 70, 71, 86, 90, 120, 0);
        let dev = sf.sections_of_type(&SectionType::Development);
        assert_eq!(dev.len(), 1);
    }
}
