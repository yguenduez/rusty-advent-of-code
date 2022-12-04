use crate::day_4::section::Section;

pub(crate) struct SectionsParser;
impl SectionsParser {
    pub fn from_str(input: &str) -> (Section, Section) {
        let sections: Vec<&str> = input.split(',').collect();
        if sections.len() != 2 {
            panic!("Could not split into two sections");
        }

        let first_section = SectionsParser::section_from_str(sections[0]);
        let second_section = SectionsParser::section_from_str(sections[1]);

        (first_section, second_section)
    }

    fn section_from_str(input: &str) -> Section {
        let start_end: Vec<&str> = input.split('-').collect();
        if start_end.len() != 2 {
            panic!("Could not parse into section")
        }

        let start: u64 = start_end[0].parse().unwrap();
        let end: u64 = start_end[1].parse().unwrap();

        Section::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::section::Section;
    use crate::day_4::sections_parser::SectionsParser;

    #[test]
    fn section_from_str() {
        let string = "1-3";
        let expected_section = Section::new(1, 3);

        assert_eq!(expected_section, SectionsParser::section_from_str(string));
    }

    #[test]
    fn parses_two_section_from_given_string() {
        let string = "1-3,3-62";
        let expected_first_section = Section::new(1, 3);
        let expected_second_section = Section::new(3, 62);

        assert_eq!(
            (expected_first_section, expected_second_section),
            SectionsParser::from_str(string)
        );
    }
}
