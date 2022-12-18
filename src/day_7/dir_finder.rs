use crate::day_7::file_tree::Directory;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DirFinder;

impl DirFinder {
    pub fn find_dirs_with_max_size_of(
        max_dir_size: usize,
        node: Rc<RefCell<Directory>>,
    ) -> Vec<Rc<RefCell<Directory>>> {
        let mut out = Vec::new();
        if node.borrow().size < max_dir_size {
            out.push(node.clone());
        }
        for child_dir in &node.borrow().child_dirs {
            out.append(&mut DirFinder::find_dirs_with_max_size_of(
                max_dir_size,
                child_dir.clone(),
            ));
        }

        out
    }
}

pub fn sum_up_size(dirs: &[Rc<RefCell<Directory>>]) -> usize {
    dirs.iter().fold(0, |acc, dir| acc + dir.borrow().size)
}

#[cfg(test)]
mod test {
    use crate::day_7::dir_finder::{sum_up_size, DirFinder};
    use crate::day_7::dir_size_calc::DirSizeCalculator;
    use crate::day_7::file_tree::Directory;
    use crate::day_7::test_utils::create_test_tree;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn finds_all_folders_with_max_size_of_50() {
        // Given
        let input_tree = create_test_tree();
        DirSizeCalculator::calculate_dir_sizes(input_tree.clone());
        let expected_max_size: usize = 50;

        // When
        let dirs = DirFinder::find_dirs_with_max_size_of(expected_max_size, input_tree);

        // Then
        assert_eq!(2, dirs.len());
        assert!(dirs.iter().any(|node| node.borrow().name == "x"));
        assert!(dirs.iter().any(|node| node.borrow().name == "y"));
    }

    #[test]
    fn sums_up_sizes_correctly() {
        // Given
        let dir_0 = Rc::new(RefCell::new(Directory {
            size: 120,
            ..Default::default()
        }));
        let dir_1 = Rc::new(RefCell::new(Directory {
            size: 220,
            ..Default::default()
        }));

        // When
        let size = sum_up_size(&[dir_0, dir_1]);

        // Then
        assert_eq!(340, size);
    }
}
