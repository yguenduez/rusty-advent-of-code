use crate::day_7::command_parser::Commands;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Default)]
pub struct Directory {
    pub name: String,
    pub size: usize,
    pub parent_dir: Option<Rc<RefCell<Directory>>>,
    pub child_dirs: Vec<Rc<RefCell<Directory>>>,
    pub child_files: Vec<Rc<RefCell<File>>>,
}

#[derive(PartialEq, Debug, Default)]
pub struct File {
    pub name: String,
    pub size: usize,
}

pub struct TreeBuilder;

impl TreeBuilder {
    pub fn from(commands: &[Commands]) -> Rc<RefCell<Directory>> {
        if commands.is_empty() {
            panic!("No commands given!");
        };
        let Commands::CdDown(dir_name) = &commands[0] else {
            panic!(r"First Command must be a 'cd /' command");
        };

        let root_dir = Directory {
            name: String::to_string(&dir_name.0),
            ..Default::default()
        };

        let root_dir = Rc::new(RefCell::new(root_dir));
        let mut current_dir = root_dir.clone();

        for command in &commands[1..] {
            match command {
                Commands::CdUp => {
                    let parent_node = current_dir.borrow().parent_dir.clone();
                    if let Some(node) = parent_node {
                        current_dir = node;
                    }
                }
                Commands::CdDown(dir_name) => {
                    let dir_to_cd_into = current_dir
                        .borrow_mut()
                        .child_dirs
                        .iter()
                        .find(|dir| dir.borrow().name == dir_name.0).cloned()
                        .expect("There is no such dir!");
                    current_dir = dir_to_cd_into;
                }
                Commands::Ls(found_items) => {
                    found_items
                        .files
                        .iter()
                        .map(|f| File {
                            name: f.name.0.clone(),
                            size: f.size,
                        })
                        .for_each(|tree_file| {
                            current_dir
                                .borrow_mut()
                                .child_files
                                .push(Rc::new(RefCell::new(tree_file)))
                        });

                    found_items
                        .dirs
                        .iter()
                        .map(|dir| Directory {
                            name: dir.0.clone(),
                            parent_dir: Some(current_dir.clone()),
                            ..Default::default()
                        })
                        .for_each(|mut dir| {
                            current_dir
                                .borrow_mut()
                                .child_dirs
                                .push(Rc::new(RefCell::new(dir)));
                        });
                }
            }
        }
        root_dir
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::command_parser::{Commands, DirName, FileName, FileWithSize, LsFinds};
    use crate::day_7::file_tree::{Directory, File, TreeBuilder};
    use crate::day_7::test_utils::create_test_tree;
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    #[test]
    fn build_tree_from_commands() {
        let commands = vec![
            Commands::CdDown(DirName('/'.to_string())),
            Commands::Ls(LsFinds {
                files: vec![FileWithSize {
                    name: FileName("hallo".to_string()),
                    size: 123,
                }],
                dirs: vec![DirName('x'.to_string()), DirName('y'.to_string())],
            }),
        ];

        let expected_root_node = create_test_tree();
        let result_root_node = TreeBuilder::from(&commands);

        assert_eq!(
            expected_root_node.borrow().child_dirs.len(),
            result_root_node.borrow().child_dirs.len()
        );

        for i in 0usize..1usize {
            let expected_dir = expected_root_node.borrow().child_dirs[i].clone();
            let result_dir = result_root_node.borrow().child_dirs[i].clone();
            assert_eq!(expected_dir.borrow().name, result_dir.borrow().name);

            let expected_root_node = expected_root_node.clone();
            let result_root_node = result_dir.borrow().parent_dir.clone().unwrap();

            assert_eq!(
                expected_root_node.borrow().name,
                result_root_node.borrow().name
            );
        }

        let expected_file = expected_root_node.borrow().child_files[0].clone();
        let result_file = result_root_node.borrow().child_files[0].clone();
        assert_eq!(expected_file.borrow().name, result_file.borrow().name);
    }
}
