use crate::day_7::file_tree::Directory;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) struct RecurseSummation;

impl RecurseSummation {
    pub(crate) fn sum_up_dir_sizes(curr_node: Rc<RefCell<Directory>>) {
        let mut sum = curr_node
            .borrow()
            .child_files
            .iter()
            .fold(0, |acc, file| acc + file.borrow().size);
        for dir in &curr_node.borrow().child_dirs {
            RecurseSummation::sum_up_dir_sizes(dir.clone());
            sum += dir.borrow().size;
        }

        curr_node.borrow_mut().size = sum;
    }
}

#[cfg(test)]
mod test {
    use crate::day_7::recursive_summation::RecurseSummation;
    use crate::day_7::test_utils::create_test_tree;

    #[test]
    fn sums_up_all_dir_nodes_with_correct_size() {
        // Given
        let input_tree = create_test_tree();

        // When
        RecurseSummation::sum_up_dir_sizes(input_tree.clone());

        // Then
        assert_eq!(123, input_tree.borrow().size);
    }
}
