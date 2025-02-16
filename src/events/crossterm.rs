use crossterm::event::{self, KeyCode};

use crate::commands::Command;

/// Convert a crossterm event to a command using the default keybindings.
pub fn event_to_command(event: &event::Event) -> Option<Command> {
    if let event::Event::Key(key) = event {
        let modifiers = key.modifiers;
        let ctrl = modifiers.contains(event::KeyModifiers::CONTROL);

        match key.code {
            KeyCode::Esc => None,
            KeyCode::Enter => None,
            KeyCode::Char('q') if ctrl => None,

            KeyCode::Backspace => Some(Command::Backspace),
            KeyCode::Char('h') if ctrl => Some(Command::Backspace),

            KeyCode::Delete => Some(Command::Delete),
            KeyCode::Char('d') if ctrl => Some(Command::Delete),

            KeyCode::Left => Some(Command::CursorLeft(1)),
            KeyCode::Char('b') if ctrl => Some(Command::CursorLeft(1)),

            KeyCode::Right => Some(Command::CursorRight(1)),
            KeyCode::Char('f') if ctrl => Some(Command::CursorRight(1)),

            KeyCode::Home => Some(Command::CursorToStartOfLine),
            KeyCode::Char('a') if ctrl => Some(Command::CursorToStartOfLine),

            KeyCode::End => Some(Command::CursorToEndOfLine),
            KeyCode::Char('e') if ctrl => Some(Command::CursorToEndOfLine),

            KeyCode::Char('u') if ctrl => Some(Command::DeleteStartOfLineToCursor),
            KeyCode::Char('k') if ctrl => Some(Command::DeleteToEndOfLine),
            KeyCode::Char('u') if ctrl => Some(Command::DeleteStartOfLineToCursor),
            KeyCode::Char('w') if ctrl => Some(Command::DeleteWordLeadingToCursor),

            KeyCode::Char(char) => Some(Command::Insert(char)),
            _ => None,
        }
    } else {
        None
    }
}
