use crate::day_7::file_tree::Directory;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct DirSizeCalculator;

impl DirSizeCalculator {
    pub(crate) fn calculate_dir_sizes(curr_node: Rc<RefCell<Directory>>) {
        let mut sum = curr_node
            .borrow()
            .child_files
            .iter()
            .fold(0, |acc, file| acc + file.borrow().size);

        for dir in &curr_node.borrow().child_dirs {
            DirSizeCalculator::calculate_dir_sizes(dir.clone());
            sum += dir.borrow().size;
        }
        curr_node.borrow_mut().size = sum;
    }
}

#[cfg(test)]
mod test {
    use crate::day_7::dir_size_calc::DirSizeCalculator;
    use crate::day_7::test_utils::create_test_tree_2;

    #[test]
    fn sums_up_all_dir_nodes_with_correct_size() {
        // Given
        let input_tree = create_test_tree_2();

        // When
        DirSizeCalculator::calculate_dir_sizes(input_tree.clone());

        // Then
        assert_eq!(183, input_tree.borrow().size);
        assert_eq!(60, input_tree.borrow().child_dirs[0].borrow().size);
    }
}
