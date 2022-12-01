use crate::common::file_io;

struct Solution;

impl Solution {
    fn solve_puzzle(input_file: &str) -> u64 {
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
}

#[cfg(test)]
mod tests {
    use crate::day_1::puzzle_1::Solution;

    #[test]
    fn print_solution_for_puzzle() {
        let solution: u64 = Solution::solve_puzzle("inputs/day_1/input.txt");
        print!("{}", solution);
    }
}
