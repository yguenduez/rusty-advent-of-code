use std::collections::HashMap;

static ALPHABET: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z",
];

pub(crate) struct PriorityMapper {
    inner: HashMap<String, u64>,
}

impl PriorityMapper {
    pub fn new() -> Self {
        let mut mapping = HashMap::new();

        for (cnt, character) in ALPHABET.iter().enumerate() {
            mapping.insert(String::from(*character), (cnt + 1) as u64);
            mapping.insert(String::from(*character).to_uppercase(), (cnt + 27) as u64);
        }

        Self { inner: mapping }
    }
    pub fn get_priority(&self, input: &str) -> u64 {
        *self.inner.get(input).expect("Something went wrong here!")
    }
}

#[cfg(test)]
mod tests {
    use crate::day_3::priority::PriorityMapper;

    #[test]
    pub fn return_correct_priority() {
        let mapper = PriorityMapper::new();
        assert_eq!(2, mapper.get_priority("b"));
        assert_eq!(27, mapper.get_priority("A"));
        assert_eq!(28, mapper.get_priority("B"));
    }
}
