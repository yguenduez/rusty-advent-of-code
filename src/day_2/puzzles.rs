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
    use crate::day_2::puzzles::Solution;

    #[test]
    fn print_solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_2/input.txt");
        print!("{solution}");
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_2/input.txt");
        print!("{solution}");
    }
}
