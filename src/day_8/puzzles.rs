struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> usize {
        todo!()
    }

    fn solve_second_puzzle(input_file: &str) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_8::puzzles::Solution;

    #[test]
    fn solution_for_first_puzzle() {
        let solution: usize = Solution::solve_first_puzzle("inputs/day_8/input.txt");
        assert_eq!(0, solution);
    }

    #[test]
    fn solution_for_second_puzzle() {
        let solution: usize = Solution::solve_second_puzzle("inputs/day_8/input.txt");
        assert_eq!(0, solution);
    }
}
