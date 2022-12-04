use crate::common::file_io::convert_file_to_lines_of_string;
use crate::day_4::section::Section;
use crate::day_4::sections_parser::SectionsParser;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        let lines = convert_file_to_lines_of_string(&input_file);
        let section_pairs: Vec<(Section, Section)> = lines
            .iter()
            .map(|line| SectionsParser::from_str(line))
            .collect();

        let mut containment_count = 0;
        for pair in section_pairs {
            let first_section = &pair.0;
            let second_section = &pair.1;
            if first_section.is_fully_contained_in(second_section)
                || second_section.is_fully_contained_in(first_section)
            {
                containment_count += 1;
            }
        }

        containment_count
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        let lines = convert_file_to_lines_of_string(&input_file);
        let section_pairs: Vec<(Section, Section)> = lines
            .iter()
            .map(|line| SectionsParser::from_str(line))
            .collect();

        let overlap_count = section_pairs.iter().fold(0, |acc, pair| {
            if pair.0.does_overlap_with(&pair.1) {
                acc + 1u64
            } else {
                acc
            }
        });

        overlap_count
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::puzzles::Solution;

    #[test]
    fn solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_4/input.txt");
        assert_eq!(536, solution);
    }

    #[test]
    fn solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_4/input.txt");
        assert_eq!(845, solution);
    }
}
