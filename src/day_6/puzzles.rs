use crate::day_6::signaler::Signaler;
use std::fs::read_to_string;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> usize {
        let file_input_str = read_to_string(input_file).unwrap();
        let marker_index = Signaler::find_marker(&file_input_str).unwrap();
        marker_index
    }

    fn solve_second_puzzle(input_file: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_6::puzzles::Solution;

    #[test]
    fn solution_for_first_puzzle() {
        let solution: usize = Solution::solve_first_puzzle("inputs/day_6/input.txt");
        assert_eq!(1282, solution);
    }

    #[test]
    fn solution_for_second_puzzle() {
        let solution: usize = Solution::solve_second_puzzle("inputs/day_6/input.txt");
        assert_eq!(0, solution);
    }
}
