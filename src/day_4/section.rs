#[derive(PartialEq, Debug)]
pub(crate) struct Section {
    start: u64,
    end: u64,
}

impl Section {
    pub fn new(start: u64, end: u64) -> Self {
        Section { start, end }
    }

    pub fn is_fully_contained_in(&self, other: &Section) -> bool {
        other.start <= self.start && other.end >= self.end
    }

    pub fn does_overlap_with(&self, other: &Section) -> bool {
        self.end >= other.start && other.end >= self.start
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::section::Section;

    #[test]
    fn first_section_does_overlap_with_other_section() {
        let first_section = Section::new(1, 2);
        let second_section = Section::new(2, 5);

        assert!(first_section.does_overlap_with(&second_section));

        let first_section = Section::new(3, 5);
        let second_section = Section::new(1, 3);

        assert!(first_section.does_overlap_with(&second_section));

        let first_section = Section::new(1, 2);
        let second_section = Section::new(1, 5);

        assert!(first_section.does_overlap_with(&second_section));
    }

    #[test]
    fn first_section_does_not_overlap_with_other_section() {
        let first_section = Section::new(1, 2);
        let second_section = Section::new(3, 5);

        assert!(!first_section.does_overlap_with(&second_section));

        let first_section = Section::new(3, 5);
        let second_section = Section::new(1, 2);

        assert!(!first_section.does_overlap_with(&second_section));
    }

    #[test]
    fn first_section_is_fully_contained_in_other_section() {
        let first_section = Section::new(1, 2);
        let second_section = Section::new(1, 5);

        assert!(first_section.is_fully_contained_in(&second_section))
    }
}
