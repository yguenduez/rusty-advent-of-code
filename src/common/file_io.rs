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
        convert_char_sequence_to_tuple_list, convert_numbers_listfile_to_vec,
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
