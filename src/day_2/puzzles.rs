use crate::common::file_io;
use crate::day_2::factories::{MatchOutComeFactory, ShapeFactory};
use crate::day_2::match_outcome::outcome_match_in_points;
use crate::day_2::paper::Paper;
use crate::day_2::points::Points;
use crate::day_2::rock::Rock;
use crate::day_2::scissor::Scissors;
use crate::day_2::shape_type::ShapeType;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        let matches = file_io::convert_char_sequence_to_tuple_list(input_file);

        let mut sum = 0;
        for m in matches {
            let other_shape = ShapeFactory::from_str(&m.0);
            let user_shape = ShapeFactory::from_str(&m.1);

            sum += outcome_match_in_points(user_shape, other_shape);
        }

        sum
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        let matches = file_io::convert_char_sequence_to_tuple_list(input_file);

        let mut sum = 0;
        for m in matches {
            let other_shape = ShapeFactory::from_str(&m.0);
            let expected_outcome = MatchOutComeFactory::from_str(&m.1);
            let user_shape = other_shape.needed_shape_in_order_to(expected_outcome.clone());

            let points = expected_outcome.points() + user_shape.points();
            sum += points;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::puzzles::Solution;

    #[test]
    fn print_solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_2/input.txt");
        assert_eq!(8392, solution);
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_2/input.txt");
        assert_eq!(10116, solution);
    }
}
