use std::collections::{HashMap, HashSet, VecDeque};

pub struct Signaler;

impl Signaler {
    pub fn find_marker(input: &str) -> Option<usize> {
        let mut char_queue: VecDeque<char> = VecDeque::new();
        for (i, c) in input.chars().enumerate() {
            if char_queue.len() >= 4 {
                char_queue.pop_front();
            }
            char_queue.push_back(c);

            let set: HashSet<char> = char_queue.iter().cloned().collect();
            if set.len() == 4 {
                return Some(i + 1);
            }
        }
        None
    }
}

mod tests {
    use crate::day_6::signaler::Signaler;

    #[test]
    fn find_marker_returns_index_of_marker_when_marker_exists_in_sequence() {
        let input_sequence: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let expected_marker_sequence: usize = 10;

        assert_eq!(
            expected_marker_sequence,
            Signaler::find_marker(input_sequence).unwrap()
        );
    }
}
