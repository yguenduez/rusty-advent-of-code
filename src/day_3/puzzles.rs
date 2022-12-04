use crate::common::file_io;
use crate::day_3::common_item_finder::CommonItemFinder;
use crate::day_3::priority::PriorityMapper;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        let rucksacks = file_io::convert_list_of_rucksack_inputs_to_vec_of_compartments(input_file);
        let mut sum = 0;

        let priorities = PriorityMapper::new();
        for rucksack in rucksacks {
            let first_compartment = rucksack.0;
            let second_compartment = rucksack.1;
            let common_item = CommonItemFinder::find_item(first_compartment, second_compartment);

            let priority = priorities.get_priority(&common_item);
            sum += priority;
        }

        sum
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
        assert_eq!(7903, solution);
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_3/input.txt");
        assert_eq!(0, solution);
    }
}
