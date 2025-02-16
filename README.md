# string_cmd

A Rust library for building powerful string editing components with support for Emacs and Vi-style keybindings. Perfect for terminal applications that need sophisticated text input handling with more flexibility than `readline` provides.

`string_cmd` provides default keybindings for Emacs and Vi modes, or you can forego the default keybindings and implement your own.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
string_cmd = "0.0.1"

# If you want crossterm events integration:
string_cmd = { version = "0.1.0", features = ["crossterm"] }
```

## Basic Usage

```rust
use string_cmd::StringEditor;

// Create a new empty editor
let mut editor = StringEditor::new();

// Or initialize with existing text
let mut editor = StringEditor::with_string("Hello, world!");

// Get the current text
println!("Current text: {}", editor.get_text());

// Get cursor position
println!("Cursor at: {}", editor.cursor_pos());
```

See the [documentation](https://docs.rs/string_cmd/latest/string_cmd/) for more details.

## Crossterm Integration

The library provides seamless integration with crossterm for terminal applications. Here's a complete example:

```rust
use std::time::Duration;
use crossterm::{
    cursor, event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    execute, terminal,
};
use string_cmd::{StringEditor, events::event_to_command};

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
    println!("\nFinal input: {:?}", input);

    Ok(())
}
```

## Supported Keybindings (with crossterm feature)

### Navigation
- `Ctrl+B` or `Left Arrow`: Move cursor left
- `Ctrl+F` or `Right Arrow`: Move cursor right
- `Ctrl+A` or `Home`: Move to start of line
- `Ctrl+E` or `End`: Move to end of line

### Editing
- `Ctrl+H` or `Backspace`: Delete character before cursor
- `Ctrl+D` or `Delete`: Delete character at cursor
- `Ctrl+K`: Delete from cursor to end of line
- `Ctrl+U`: Delete from start of line to cursor
- `Ctrl+W`: Delete word leading to cursor

## Usage
```rust
use string_cmd::StringEditor;

let mut editor = StringEditor::new();
// how you get the event is up to you;
// most commonly, you'll use `crossterm::event::poll()` and `crossterm::event::read()`
let event = get_crossterm_event(); 

if let Some(command) = event_to_command(&event) {
    editor.execute(command);
}
```

## License

This project is licensed under the MIT License. 
