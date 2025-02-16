use std::time::Duration;

use crossterm::cursor::{self};
use crossterm::event::{self, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, terminal};
use string_cmd::events::event_to_command;
use string_cmd::StringEditor;

fn main() -> std::io::Result<()> {
    let mut editor = StringEditor::new();
    let mut stdout = std::io::stdout();

    println!("Enter text below:");
    enable_raw_mode()?;

    let input = loop {
        if event::poll(Duration::from_millis(10))? {
            let event = event::read()?;
            if let event::Event::Key(key) = &event {
                let ctrl = key.modifiers.contains(event::KeyModifiers::CONTROL);

                match key.code {
                    KeyCode::Esc => break None,
                    KeyCode::Char('q') if ctrl => break None,
                    KeyCode::Enter => break Some(editor.get_text()),
                    _ => {}
                }
            }

            if let Some(command) = event_to_command(&event) {
                editor.execute(command);
            }
        }

        execute!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;
        print!("{}", editor.get_text());
        execute!(stdout, cursor::MoveToColumn(editor.cursor_pos() as u16))?;
    };

    disable_raw_mode()?;
    println!("\nGot input: {:?}", input);

    Ok(())
}
