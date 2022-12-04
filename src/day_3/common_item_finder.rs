use std::collections::HashSet;

pub(crate) struct CommonItemFinder;

impl CommonItemFinder {
    pub fn find_item(first: String, second: String) -> String {
        let set: HashSet<char> = first.chars().collect();
        for c in second.chars() {
            if let Some(character) = set.get(&c) {
                return String::from(*character);
            }
        }
        panic!("There is not such double item!");
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
