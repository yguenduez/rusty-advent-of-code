use crate::day_7::command_parser::{CommandParser, Commands};
use crate::day_7::dir_finder::{sum_up_size, DirFinder};
use crate::day_7::dir_size_calc::DirSizeCalculator;
use crate::day_7::file_tree::{Directory, TreeBuilder};
use std::cell::RefCell;
use std::fs::read_to_string;
use std::rc::Rc;

struct Solution;

impl Solution {
    fn solve_first_puzzle(input_file: &str) -> usize {
        let commands = CommandParser::parse_str(&read_to_string(input_file).unwrap());
        let tree_root_node = TreeBuilder::from(&commands);
        DirSizeCalculator::calculate_dir_sizes(tree_root_node.clone());
        let max_dir_size: usize = 100_000;
        let dirs = DirFinder::find_dirs_with_max_size_of(max_dir_size, tree_root_node);

        sum_up_size(&dirs)
    }

    fn solve_second_puzzle(input_file: &str) -> usize {
        let commands = CommandParser::parse_str(&read_to_string(input_file).unwrap());
        let tree_root_node: Rc<RefCell<Directory>> = TreeBuilder::from(&commands);
        DirSizeCalculator::calculate_dir_sizes(tree_root_node.clone());

        let current_total_size: usize = tree_root_node.as_ref().borrow().size;
        const DISK_SPACE: usize = 70_000_000;
        const UPDATE_SIZE: usize = 30_000_000;
        let unused_space = DISK_SPACE - current_total_size;
        let min_needed_space = UPDATE_SIZE - unused_space;

        let mut dirs = DirFinder::find_dirs_with_min_size_of(min_needed_space, tree_root_node);
        dirs.sort_by(|a, b| b.as_ref().borrow().size.cmp(&a.as_ref().borrow().size));

        let smallest_dir_size: usize = dirs.last().unwrap().as_ref().borrow().size;
        smallest_dir_size
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::command_parser::CommandParser;
    use crate::day_7::dir_size_calc::DirSizeCalculator;
    use crate::day_7::file_tree::{Directory, TreeBuilder};
    use crate::day_7::puzzles::Solution;
    use std::cell::RefCell;
    use std::fs::read_to_string;
    use std::rc::Rc;

    #[test]
    fn solution_for_first_puzzle() {
        let solution: usize = Solution::solve_first_puzzle("inputs/day_7/input.txt");
        assert_eq!(1243729, solution);
    }

    fn generate_depth_visualization(depth: usize) -> String {
        let mut whitespaces = String::from("-");
        for _ in 0..depth {
            whitespaces.push_str("-");
        }
        whitespaces
    }

    fn print_tree_recursive(node: Rc<RefCell<Directory>>, tree_depth: usize) {
        let current_depth_vis = generate_depth_visualization(tree_depth);
        let current_depth_vis_childs = generate_depth_visualization(tree_depth + 1);

        let dir_name = &node.borrow().name;
        let dir_size = &node.borrow().size;
        println!("{current_depth_vis} dir: ({dir_name} : {dir_size})");
        for file in &node.borrow().child_files {
            let filename = &file.borrow().name;
            let filesize = &file.borrow().size;

            println!("{current_depth_vis_childs} file: ({filename} : {filesize})");
        }
        for dir in &node.borrow().child_dirs {
            print_tree_recursive(dir.clone(), tree_depth + 1);
        }
    }

    #[test]
    fn tree_is_build_like_in_the_tutorial() {
        // Debug purpose
        let input: &str = "$ cd /\n\
$ ls\n\
dir a\n\
14848514 b.txt\n\
8504156 c.dat\n\
dir d\n\
$ cd a\n\
$ ls\n\
dir e\n\
29116 f\n\
2557 g\n\
62596 h.lst\n\
$ cd e\n\
$ ls\n\
584 i\n\
$ cd ..\n\
$ cd ..\n\
$ cd d\n\
$ ls\n\
4060174 j\n\
8033020 d.log\n\
5626152 d.ext\n\
7214296 k";
        let commands = CommandParser::parse_str(&input);
        let tree_root_node = TreeBuilder::from(&commands);
        DirSizeCalculator::calculate_dir_sizes(tree_root_node.clone());
        print_tree_recursive(tree_root_node.clone(), 0);
    }

    #[test]
    fn solution_for_second_puzzle() {
        let solution: usize = Solution::solve_second_puzzle("inputs/day_7/input.txt");
        assert_eq!(4443914, solution);
    }
}
