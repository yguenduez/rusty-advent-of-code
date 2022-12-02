use crate::common::file_io;
use crate::day_2::match_outcome::outcome_match_in_points;
use crate::day_2::paper::Paper;
use crate::day_2::rock::Rock;
use crate::day_2::scissor::Scissors;
use crate::day_2::shape_factory::ShapeFactory;
use crate::day_2::shape_type::ShapeType;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        let matches = file_io::convert_char_sequence_to_tuple_list(input_file);

        let mut sum = 0;
        for m in matches {
            let other_type = ShapeFactory::from_str(&m.0);
            let user_type = ShapeFactory::from_str(&m.1);

            sum += outcome_match_in_points(user_type, other_type);
        }

        sum
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::puzzles::Solution;

    #[test]
    fn print_solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_2/input.txt");
        assert_eq!(10718, solution);
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_2/input.txt");
        print!("{solution}");
    }
}
