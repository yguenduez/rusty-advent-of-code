#[derive(Debug, PartialEq)]
pub struct FileWithSize {
    pub name: FileName,
    pub size: usize,
}

#[derive(Default, Debug, PartialEq)]
pub struct LsFinds {
    pub files: Vec<FileWithSize>,
    pub dirs: Vec<DirName>,
}

#[derive(Debug, PartialEq)]
pub struct DirName(pub String);

#[derive(Debug, PartialEq)]
pub struct FileName(pub String);

#[derive(Debug, PartialEq)]
pub enum Commands {
    CdUp,
    CdDown(DirName),
    Ls(LsFinds),
}

trait AddFile {
    fn add_file(&mut self, file: FileWithSize);
}
trait AddDir {
    fn add_dir(&mut self, dir: DirName);
}

impl AddFile for Commands {
    fn add_file(&mut self, file: FileWithSize) {
        if let Commands::Ls(ls_finds) = self {
            ls_finds.files.push(file);
        }
    }
}

impl AddDir for Commands {
    fn add_dir(&mut self, dir: DirName) {
        if let Commands::Ls(ls_finds) = self {
            ls_finds.dirs.push(dir);
        }
    }
}

struct CommandParser;

impl CommandParser {
    fn parse_str(input: &str) -> Vec<Commands> {
        let mut commands = Vec::new();
        let mut lines = input.lines();
        for line in lines {
            if line.starts_with("$ cd") {
                let token_or_name = &line["$ cd ".len()..];
                if token_or_name == ".." {
                    commands.push(Commands::CdUp);
                    continue;
                }
                let dir_name = DirName(token_or_name.to_string());
                commands.push(Commands::CdDown(dir_name));
            } else if line.starts_with("$ ls") {
                let ls_finds = LsFinds::default();
                commands.push(Commands::Ls(ls_finds));
            } else {
                let ls_find = commands.last_mut().expect("Something bad happened");
                let (token, name): (&str, &str) = {
                    let token_and_name: Vec<&str> = line.split_whitespace().collect();
                    if token_and_name.len() != 2 {
                        panic!("Input is incorrect to parse");
                    }
                    (token_and_name[0], token_and_name[1])
                };

                if token == "dir" {
                    let dirname = DirName(name.to_string());
                    ls_find.add_dir(dirname);
                } else {
                    let filename = FileName(name.to_string());
                    let file_with_size = FileWithSize {
                        name: filename,
                        size: token.parse::<usize>().expect("Could not parse filesize"),
                    };
                    ls_find.add_file(file_with_size);
                }
            }
        }

        commands
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::command_parser::{
        CommandParser, Commands, DirName, FileName, FileWithSize, LsFinds,
    };

    #[test]
    fn parses_commands_from_str() {
        let lines: &str = "$ cd /\n$ ls\n123 hello\ndir x\ndir y\n$ cd ..";
        assert_eq!(
            vec![
                Commands::CdDown(DirName('/'.to_string())),
                Commands::Ls(LsFinds {
                    files: vec![FileWithSize {
                        name: FileName("hello".to_string()),
                        size: 123
                    }],
                    dirs: vec![DirName('x'.to_string()), DirName('y'.to_string())]
                }),
                Commands::CdUp
            ],
            CommandParser::parse_str(lines)
        );
    }
}
