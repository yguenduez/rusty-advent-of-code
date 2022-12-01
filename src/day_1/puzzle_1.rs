use crate::common::file_io;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> u64 {
        let calories = file_io::convert_numbers_listfile_to_vec(input_file);
        let mut biggest_sum: u64 = 0;
        let mut current_sum: u64 = 0;
        for calorie in calories {
            match calorie {
                None => {
                    if current_sum > biggest_sum {
                        biggest_sum = current_sum;
                    }

                    current_sum = 0;
                }
                Some(num) => {
                    current_sum += num;
                }
            }
        }

        // Last item
        if current_sum > biggest_sum {
            biggest_sum = current_sum;
        }

        biggest_sum
    }

    fn solve_second_puzzle(input_file: &str) -> u64 {
        let calories = file_io::convert_numbers_listfile_to_vec(input_file);
        let mut heap = BinaryHeap::new();
        let mut current_sum: u64 = 0;
        for calorie in calories {
            match calorie {
                None => {
                    heap.push(current_sum);
                    current_sum = 0;
                }
                Some(num) => {
                    current_sum += num;
                }
            }
        }

        // Last item
        heap.push(current_sum);

        let mut sum_of_three_most_calorie_elves = 0;
        for _ in 0..3 {
            sum_of_three_most_calorie_elves += heap.pop().unwrap();
        }

        sum_of_three_most_calorie_elves
    }
}

#[cfg(test)]
mod tests {
    use crate::day_1::puzzle_1::Solution;

    #[test]
    fn print_solution_for_first_puzzle() {
        let solution: u64 = Solution::solve_first_puzzle("inputs/day_1/input.txt");
        print!("{}", solution);
    }

    #[test]
    fn print_solution_for_second_puzzle() {
        let solution: u64 = Solution::solve_second_puzzle("inputs/day_1/input.txt");
        print!("{}", solution);
    }
}
