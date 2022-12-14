use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn convert_char_sequence_to_tuple_list(filepath: &str) -> Vec<(String, String)> {
    let file = File::open(filepath).expect("Could not open file!");
    let lines = io::BufReader::new(file).lines();
    let mut out = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            panic!("Could not read line!");
        };

        let letters = {
            let iter: Vec<&str> = line.split_whitespace().collect();
            (iter[0].to_owned(), iter[1].to_owned())
        };

        out.push(letters);
    }

    out
}

pub fn convert_file_to_lines_of_string(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).expect("Could not open file!");
    let lines = io::BufReader::new(file).lines();

    let lines: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
    lines
}

pub fn convert_list_of_rucksack_inputs_to_vec_of_compartments(
    filepath: &str,
) -> Vec<(String, String)> {
    let file = File::open(filepath).expect("Could not open file!");
    let lines = io::BufReader::new(file).lines();
    let mut out = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            panic!("Could not read line!");
        };

        let string_len = line.len();
        let first_half = line[..string_len / 2].to_owned();
        let second_half = line[string_len / 2..].to_owned();

        out.push((first_half, second_half));
    }

    out
}

pub fn convert_list_of_rucksack_inputs_groups_of_three(filepath: &str) -> Vec<Vec<String>> {
    let file = File::open(filepath).expect("Could not open file!");
    let lines = io::BufReader::new(file).lines();
    let mut list_of_groups = Vec::new();
    let mut grp = Vec::new();
    for (line_cnt, line) in lines.enumerate() {
        let Ok(line) = line else {
            panic!("Could not read line!");
        };

        grp.push(line);

        if (line_cnt + 1) % 3 == 0 {
            list_of_groups.push(grp);
            grp = Vec::new()
        }
    }

    list_of_groups
}

pub fn convert_input_file_stacks_to_list_of_stacks(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = Vec::new();
    for line in lines {
        const ITEM_BEGIN: char = '[';
        let begin_indices: Vec<usize> = line
            .chars()
            .enumerate()
            .filter(|(_, c)| c.eq(&ITEM_BEGIN))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        if begin_indices.is_empty() {
            // we reached the last line
            break;
        }

        for index in begin_indices {
            const SEPERATION_SIZE: usize = 4usize;
            let stack_index = index / SEPERATION_SIZE;
            let new_char = line.chars().nth(index + 1).unwrap();
            if stack_index >= out.len() {
                out.resize(stack_index + 1, vec![]);
            }

            out[stack_index].push(new_char);
        }
    }

    // revert stacks
    for stack in &mut out {
        stack.reverse();
    }

    out
}

pub fn convert_numbers_listfile_to_vec(filepath: &str) -> Vec<Option<u64>> {
    let file = File::open(filepath).expect("Could not open file!");

    let lines = io::BufReader::new(file).lines();

    let mut out = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            panic!("Could not read line!");
        };

        let num = match line.parse::<u64>() {
            Ok(num) => Some(num),
            Err(_) => None,
        };

        out.push(num);
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::common::file_io::{
        convert_char_sequence_to_tuple_list, convert_file_to_lines_of_string,
        convert_input_file_stacks_to_list_of_stacks,
        convert_list_of_rucksack_inputs_groups_of_three,
        convert_list_of_rucksack_inputs_to_vec_of_compartments, convert_numbers_listfile_to_vec,
    };

    #[test]
    fn converts_char_sequence_to_tuples_vec() {
        let expected_vec: Vec<(String, String)> = vec![
            (String::from("A"), String::from("Y")),
            (String::from("B"), String::from("X")),
            (String::from("C"), String::from("Z")),
        ];
        assert_eq!(
            expected_vec,
            convert_char_sequence_to_tuple_list("test_inputs/rock_paper_scissors.txt")
        );
    }

    #[test]
    pub fn convert_file_stacks_to_list_of_stacks() {
        let expected_stacks: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let lines = convert_file_to_lines_of_string("test_inputs/init_stack.txt");

        assert_eq!(
            expected_stacks,
            convert_input_file_stacks_to_list_of_stacks(&lines)
        );
    }

    #[test]
    pub fn convert_file_move_operations_to_move_operation_tuple() {}

    #[test]
    fn converts_rucksack_input_to_compartment_vec() {
        let expected_vec: Vec<(String, String)> = vec![
            (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp")),
            (
                String::from("jqHRNqRjqzjGDLGL"),
                String::from("rsFMfFZSrLrFZsSL"),
            ),
            (String::from("PmmdzqPrV"), String::from("vPwwTWBwg")),
        ];
        assert_eq!(
            expected_vec,
            convert_list_of_rucksack_inputs_to_vec_of_compartments(
                "test_inputs/rucksack_input.txt"
            )
        );
    }

    #[test]
    fn converts_rucksack_input_to_list_of_groups_of_three() {
        let expected_vec: Vec<Vec<String>> = vec![
            vec![
                String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
                String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                String::from("PmmdzqPrVvPwwTWBwg"),
            ],
            vec![
                String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
                String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                String::from("PmmdzqPrVvPwwTWBwg"),
            ],
        ];
        assert_eq!(
            expected_vec,
            convert_list_of_rucksack_inputs_groups_of_three("test_inputs/rucksack_input_2.txt")
        );
    }

    #[test]
    fn converts_file_to_list_of_lines() {
        let expected_vec: Vec<String> = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            String::from("PmmdzqPrVvPwwTWBwg"),
        ];
        assert_eq!(
            expected_vec,
            convert_file_to_lines_of_string("test_inputs/rucksack_input_2.txt")
        );
    }

    #[test]
    fn converts_list_with_numbers_to_vec() {
        let expected_vec: Vec<Option<u64>> = vec![
            Some(1000),
            Some(2000),
            Some(3000),
            None,
            Some(4000),
            None,
            Some(5000),
            Some(6000),
            None,
            Some(7000),
            Some(8000),
            Some(9000),
            None,
            Some(10_000),
        ];
        assert_eq!(
            expected_vec,
            convert_numbers_listfile_to_vec("test_inputs/linear_file_with_new_line.txt")
        );
    }
}
