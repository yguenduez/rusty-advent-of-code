use crate::day_7::command_parser::Commands;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Default)]
struct Directory {
    parent_dir: Option<Rc<RefCell<Directory>>>,
    name: String,
    size: usize,
    child_dirs: Vec<Rc<RefCell<Directory>>>,
    child_files: Vec<Rc<RefCell<File>>>,
}

#[derive(PartialEq, Debug)]
struct File {
    name: String,
    size: usize,
}

struct TreeBuilder;

impl TreeBuilder {
    fn from(commands: &[Commands]) -> Directory {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::command_parser::{Commands, DirName, FileName, FileWithSize, LsFinds};
    use crate::day_7::file_tree::{Directory, File, TreeBuilder};
    use std::cell::RefCell;
    use std::default;
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

        let dir_x = Directory {
            name: "x".to_string(),
            ..Default::default()
        };

        let dir_y = Directory {
            name: "x".to_string(),
            ..Default::default()
        };

        let file = File {
            name: "hallo".to_string(),
            size: 123,
        };

        let mut rootDir = Rc::new(RefCell::new(Directory {
            parent_dir: None,
            name: "/".to_string(),
            size: 0,
            child_dirs: vec![Rc::new(RefCell::new(dir_x)), Rc::new(RefCell::new(dir_y))],
            child_files: vec![Rc::new(RefCell::new(file))],
        }));
        rootDir
            .borrow_mut()
            .child_dirs
            .iter_mut()
            .for_each(|child| child.borrow_mut().parent_dir = Some(rootDir.clone()));

        assert_eq!(rootDir.take(), TreeBuilder::from(&commands));
    }
}
