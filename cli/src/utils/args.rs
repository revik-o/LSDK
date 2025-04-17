use std::env;

use crate::args_declaration::{
    ArgsDeclaration, CommandDeclarationPosition, CommandToken, supported_commands,
};

fn recognize_command(dec_ptr: &ArgsDeclaration, arg_val_ptr: &String) -> bool {
    let args = dec_ptr.clone().args;
    args.into_iter().any(|val| val == (*arg_val_ptr))
}

pub fn parce_args_to_tokens() -> Vec<CommandDeclarationPosition> {
    let mut result: Vec<CommandDeclarationPosition> = vec![];
    let supported_commands = supported_commands();

    for (index, value) in env::args().enumerate() {
        let command = supported_commands
            .iter()
            .find(|ptr| recognize_command(*ptr, &value));

        match command {
            None => continue,
            Some(ptr) => {
                let declaration = ptr.clone();
                result.push(CommandDeclarationPosition::new(declaration.command, index));
            }
        }
    }

    result
}

pub fn get_command_value(position: CommandDeclarationPosition) -> Option<String> {
    match position.command {
        CommandToken::Name | CommandToken::Type => {
            let args: Vec<String> = env::args().collect();

            if args.len() > position.position + 1 {
                return Option::Some(args[position.position + 1].clone());
            }

            Option::None
        }
        _ => Option::None,
    }
}
