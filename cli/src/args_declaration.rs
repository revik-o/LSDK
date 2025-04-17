pub const HELP_INFO: &str = include_str!("../resources/help.txt");

#[derive(Clone, Copy, Debug)]
pub enum CommandToken {
    New = 0,
    Help = 1,
    Type = 2,
    List = 3,
    Name = 4,
    Project = 5,
    Module = 6,
    AutoComplete = 7,
    Version = 8,
}

#[derive(Clone)]
pub struct ArgsDeclaration {
    pub command: CommandToken,
    pub args: Vec<String>,
}

#[derive(Clone)]
pub struct CommandDeclarationPosition {
    pub command: CommandToken,
    pub position: usize,
}

impl CommandDeclarationPosition {
    pub fn new(command: CommandToken, position: usize) -> Self {
        CommandDeclarationPosition { command, position }
    }
}

impl ArgsDeclaration {
    pub fn new(command: CommandToken, args: Vec<&str>) -> Self {
        let args = args.into_iter().map(|s| s.to_string()).collect();
        ArgsDeclaration { command, args }
    }
}

pub fn supported_commands() -> [ArgsDeclaration; 9] {
    [
        ArgsDeclaration::new(CommandToken::New, vec!["new"]),
        ArgsDeclaration::new(CommandToken::Help, vec!["help", "h", "-?", "--help", "-h"]),
        ArgsDeclaration::new(CommandToken::Type, vec!["type", "t", "--type", "-t"]),
        ArgsDeclaration::new(CommandToken::List, vec!["list", "l", "ls", "--list", "-l"]),
        ArgsDeclaration::new(CommandToken::Name, vec!["name", "n", "--name", "-n"]),
        ArgsDeclaration::new(CommandToken::Project, vec!["project", "p"]),
        ArgsDeclaration::new(CommandToken::Module, vec!["module", "m"]),
        ArgsDeclaration::new(CommandToken::AutoComplete, vec!["auto-complete"]),
        ArgsDeclaration::new(CommandToken::Version, vec!["version", "-v", "--version"]),
    ]
}
