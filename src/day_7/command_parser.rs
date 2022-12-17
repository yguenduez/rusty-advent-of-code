#[derive(Debug, PartialEq)]
struct FileWithSize {
    name: FileName,
    size: usize,
}

#[derive(Debug, PartialEq)]
struct LsFinds {
    files: Vec<FileWithSize>,
    dirs: Vec<DirName>,
}

#[derive(Debug, PartialEq)]
struct DirName(String);

#[derive(Debug, PartialEq)]
struct FileName(String);

#[derive(Debug, PartialEq)]
enum Commands {
    CdUp,
    CdDown(DirName),
    Ls(LsFinds),
}

struct CommandParser;

impl CommandParser {
    fn parse_str(input: &str) -> Vec<Commands> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::day_7::command_parser::{
        CommandParser, Commands, DirName, FileName, FileWithSize, LsFinds,
    };

    #[test]
    fn parses_commands_from_str() {
        let lines: &str = r"$ cd /
        $ ls
        123 hallo
        dir x
        dir y
        $ cd ..";
        assert_eq!(
            vec![
                Commands::CdDown(DirName('/'.to_string())),
                Commands::Ls(LsFinds {
                    files: vec![FileWithSize {
                        name: FileName("hallo".to_string()),
                        size: 123
                    }],
                    dirs: vec![DirName('x'.to_string()), DirName('y'.to_string())]
                })
            ],
            CommandParser::parse_str(lines)
        );
    }
}
