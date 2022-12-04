use std::collections::HashSet;

pub(crate) struct CommonItemFinder;

impl CommonItemFinder {
    pub fn find_item(first: String, second: String) -> String {
        let set: HashSet<String> = first.chars().map(|c| String::from(c)).collect();
        CommonItemFinder::find_common_item(&set, &second)
    }

    pub fn find_common_item(set: &HashSet<String>, characters: &str) -> String {
        for c in characters.chars() {
            if let Some(character) = set.get(&String::from(c)) {
                return String::from(character);
            }
        }
        panic!("There is not such double item!");
    }

    pub fn find_items(first: &str, second: &str) -> HashSet<String> {
        let set: HashSet<char> = first.chars().collect();
        let mut common_items = HashSet::new();
        for c in second.chars() {
            if let Some(character) = set.get(&c) {
                common_items.insert(String::from(*character));
            }
        }

        common_items
    }
}

#[cfg(test)]
mod tests {
    use crate::day_3::common_item_finder::CommonItemFinder;

    #[test]
    fn returns_correct_duplicate_item() {
        let first = String::from("afcghtT");
        let second = String::from("ipmjqTu");
        assert_eq!(
            String::from("T"),
            CommonItemFinder::find_item(first, second)
        );
    }
}
