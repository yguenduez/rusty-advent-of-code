use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn read_linear_file_to_vec(filepath: &str) -> Vec<Option<u64>> {
    let file = File::open(filepath).expect("Could not open file!");

    let lines = io::BufReader::new(file).lines();

    let mut out = Vec::new();
    for line in lines {
        let Ok(line) = line else {
            panic!("Could not read file!");
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
    use crate::common::file_io::read_linear_file_to_vec;

    #[test]
    fn reads_linear_file_to_vec() {
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
            read_linear_file_to_vec("test_inputs/linear_file_with_new_line.txt")
        );
    }
}
