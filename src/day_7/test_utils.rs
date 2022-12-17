use crate::day_7::file_tree::{Directory, File};
use std::cell::RefCell;
use std::rc::Rc;

pub fn create_test_tree() -> Rc<RefCell<Directory>> {
    let dir_x = Directory {
        name: "x".to_string(),
        ..Default::default()
    };

    let dir_y = Directory {
        name: "y".to_string(),
        ..Default::default()
    };

    let file = File {
        name: "hallo".to_string(),
        size: 123,
    };

    let expected_root_node = Rc::new(RefCell::new(Directory {
        parent_dir: None,
        name: "/".to_string(),
        size: 0,
        child_dirs: vec![Rc::new(RefCell::new(dir_x)), Rc::new(RefCell::new(dir_y))],
        child_files: vec![Rc::new(RefCell::new(file))],
    }));
    expected_root_node
        .borrow_mut()
        .child_dirs
        .iter_mut()
        .for_each(|child| child.borrow_mut().parent_dir = Some(expected_root_node.clone()));
    expected_root_node
}
