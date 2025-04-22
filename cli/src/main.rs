mod args_declaration;
mod utils;

use args_declaration::{CommandToken, HELP_INFO};
/// A Ratatui example that demonstrates a basic hello world application.
///
/// This example runs with the Ratatui library code in the branch that you are currently
/// reading. See the [`latest`] branch for the code which works with the most recent Ratatui
/// release.
///
/// [`latest`]: https://github.com/ratatui/ratatui/tree/latest
use color_eyre::Result;
use color_eyre::eyre::{Context, Ok};
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
use std::time::Duration;
use utils::args::{get_command_name, get_command_type, parce_args_to_tokens};

fn draw_project_activity(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn draw_module_activity(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.area());
}

fn deploy_activity(main_token: CommandToken, name_value: String, type_value: String) -> Result<()> {
    let activity_callback = if main_token == CommandToken::Project {
        draw_project_activity
    } else {
        draw_module_activity
    };

    color_eyre::install()?;
    let mut terminal: DefaultTerminal = ratatui::init();

    loop {
        terminal.draw(activity_callback)?;

        if should_quit()? {
            break;
        }
    }

    ratatui::restore();
    Ok(())
}

fn show_help_info() -> Result<()> {
    println!("{}", HELP_INFO);
    Ok(())
}

fn create_project(name_value: String, type_value: String) -> Result<()> {
    // TODO: Implement project creation logic
    Ok(())
}

fn create_module(name_value: String, type_value: String) -> Result<()> {
    // TODO: Implement module creation logic
    Ok(())
}

fn main() -> Result<()> {
    let tokens = parce_args_to_tokens();

    if tokens.is_empty() {
        return show_help_info();
    }

    let main_token = tokens[0].command;
    match main_token {
        CommandToken::Project | CommandToken::Module => {
            let name_value = get_command_name(&tokens);
            let type_value = get_command_type(&tokens);

            if !name_value.is_empty() && !type_value.is_empty() {
                if main_token == CommandToken::Project {
                    create_project(name_value, type_value).context("Failed to create project")
                } else {
                    create_module(name_value, type_value).context("Failed to create module")
                }
            } else {
                deploy_activity(main_token, name_value, type_value)
                    .context("Failed to create activity")
            }
        }
        CommandToken::Version => {
            println!("v1.0.0");
            Ok(())
        }
        _ => show_help_info(),
    }
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll to ensure that the terminal is rendered at
/// least once every 250ms. This allows you to do other work in the application loop, such as
/// updating the application state, without blocking the event loop for too long.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
