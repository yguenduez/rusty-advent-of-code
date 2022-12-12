use crate::common::file_io;
use crate::day_5::move_operation::MoveOperation;
use std::collections::BinaryHeap;
use std::str::FromStr;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> Vec<char> {
        let lines = file_io::convert_file_to_lines_of_string(input_file);
        const MOVE_KEY: &str = "move";
        let move_operation_line_start_number = lines
            .iter()
            .enumerate()
            .find(|(_, str)| str.contains(MOVE_KEY))
            .map(|(num, _)| num)
            .unwrap();

        let mut initial_stacks: Vec<Vec<char>> =
            file_io::convert_input_file_stacks_to_list_of_stacks(&lines);
        let move_operations: Vec<MoveOperation> = lines[move_operation_line_start_number..]
            .into_iter()
            .map(|line| MoveOperation::from_str(line).unwrap())
            .collect();

        move_operations
            .iter()
            .for_each(|op| op.apply_to(&mut initial_stacks));

        initial_stacks
            .into_iter()
            .map(|mut stack| stack.pop().unwrap())
            .collect()
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_5::puzzles::Solution;

    #[test]
    fn solution_for_first_puzzle() {
        let solution: Vec<char> = Solution::solve_first_puzzle("inputs/day_5/input.txt");
        let expected: Vec<char> = vec!['W', 'S', 'F', 'T', 'M', 'R', 'H', 'P', 'P'];
        assert_eq!(expected, solution);
    }

    #[test]
    fn solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_5/input.txt");
        assert_eq!(0, solution);
    }
}
