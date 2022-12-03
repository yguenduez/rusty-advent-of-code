struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        todo!()
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_3::puzzles::Solution;

    #[test]
    fn print_solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_3/input.txt");
        assert_eq!(0, solution);
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_3/input.txt");
        assert_eq!(0, solution);
    }
}
