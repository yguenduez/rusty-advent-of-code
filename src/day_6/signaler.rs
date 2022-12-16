use std::collections::{HashMap, HashSet, VecDeque};

pub struct Signaler {
    distinct_marker_sequence_length: usize,
}

impl Signaler {
    pub fn new(marker_sequence_len: usize) -> Self {
        Signaler {
            distinct_marker_sequence_length: marker_sequence_len,
        }
    }

    pub fn find_marker(&self, input: &str) -> Option<usize> {
        let mut char_queue: VecDeque<char> = VecDeque::new();
        for (i, c) in input.chars().enumerate() {
            if char_queue.len() >= self.distinct_marker_sequence_length {
                char_queue.pop_front();
            }
            char_queue.push_back(c);

            let set: HashSet<char> = char_queue.iter().cloned().collect();
            if set.len() == self.distinct_marker_sequence_length {
                return Some(i + 1);
            }
        }
        None
    }
}

mod tests {
    use crate::day_6::signaler::Signaler;

    #[test]
    fn find_marker_returns_index_of_marker_when_marker_exists_in_sequence_with_4_sequence_len() {
        let input_sequence: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let expected_marker_sequence: usize = 10;

        let marker_signaler = Signaler::new(4);
        assert_eq!(
            expected_marker_sequence,
            marker_signaler.find_marker(input_sequence).unwrap()
        );
    }

    #[test]
    fn find_marker_returns_index_of_marker_when_marker_exists_in_sequence_with_14_sequence_len() {
        let input_sequence: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected_marker_sequence: usize = 19;

        let marker_signaler = Signaler::new(14);
        assert_eq!(
            expected_marker_sequence,
            marker_signaler.find_marker(input_sequence).unwrap()
        );
    }
}
