#![doc = include_str!("../README.md")]

use commands::Command;

/// Commands to be executed on the editor.
pub mod commands;
/// Crossterm event handling (requires the `crossterm` feature)
pub mod events;

/// A `StringEditor` instance wraps a `String` and provides a way to edit it using a variety of commands.
/// It also keeps track of the cursor position, which can be used for rendering.
///
/// # Example
///
/// ```rust
/// use string_cmd::StringEditor;
///
/// let mut editor = StringEditor::new();
///
/// // Insert text
/// editor.execute(Command::Insert('H'));
/// editor.execute(Command::Insert('e'));
/// editor.execute(Command::Insert('l'));
/// editor.execute(Command::Insert('l'));
/// editor.execute(Command::Insert('o'));
///
/// // Get the current text
/// let text = editor.get_text(); // "Hello"
///
/// // Get the cursor position
/// let cursor = editor.cursor_pos(); // 5
/// ```
///
/// In most cases, you'll want to use the `StringEditor` in conjunction with a terminal event loop.
/// If you're using `crossterm` and want to use the default keybindings, check out the [`events`] module
/// (requires the `crossterm` feature).
#[derive(Debug, Clone, Default)]
pub struct StringEditor {
    text: String,
    cursor: usize,
    // editing_mode: EditingMode,
}

impl StringEditor {
    /// Create a new `StringEditor` with an empty string.
    pub fn new() -> Self {
        Self {
            text: String::new(),
            cursor: 0,
            // editing_mode: EditingMode::Emacs,
        }
    }

    /// Create a new `StringEditor` with a given string. Sets the cursor position to just after the end of the string.
    pub fn with_string(text: &str) -> Self {
        Self {
            text: text.to_string(),
            cursor: text.len(),
            // editing_mode: EditingMode::Emacs,
        }
    }

    /// Get the current text of the editor.
    pub fn get_text(&self) -> &str {
        &self.text
    }

    /// Get the current cursor position.
    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }
}

impl StringEditor {
    /// Execute a command on the editor.
    pub fn execute(&mut self, command: Command) {
        match command {
            Command::Insert(c) => {
                self.text.insert(self.cursor, c);
                self.cursor += 1;
            }
            Command::Type(s) => {
                self.text.insert_str(self.cursor, &s);
                self.cursor += s.len();
            }
            Command::CursorLeft(amt) => self.cursor = self.cursor.saturating_sub(amt),
            Command::CursorRight(amt) => {
                self.cursor += amt;
                if self.cursor > self.text.len() {
                    self.cursor = self.text.len();
                }
            }
            Command::CursorToStartOfLine => self.cursor = 0,
            Command::CursorToEndOfLine => self.cursor = self.text.len(),
            Command::Delete => {
                if self.cursor < self.text.len() {
                    self.text.remove(self.cursor);
                }
            }
            Command::Backspace => {
                if self.cursor > 0 {
                    self.text.remove(self.cursor - 1);
                    self.cursor -= 1;
                }
            }
            Command::DeleteStartOfLineToCursor => {
                self.text.replace_range(0..self.cursor, "");
                self.cursor = 0;
            }
            Command::DeleteToEndOfLine => {
                self.text.replace_range(self.cursor..self.text.len(), "");
            }
            Command::DeleteWordLeadingToCursor => {
                if self.cursor > 0 {
                    let mut pos = self.cursor - 1;

                    while pos > 0
                        && !self.text[pos - 1..pos]
                            .chars()
                            .next()
                            .unwrap()
                            .is_whitespace()
                        && !matches!(
                            self.text[pos - 1..pos].chars().next().unwrap(),
                            '-' | '_'
                                | '+'
                                | '='
                                | ','
                                | '.'
                                | '/'
                                | '\\'
                                | ':'
                                | ';'
                                | '!'
                                | '?'
                                | '@'
                                | '#'
                                | '$'
                                | '%'
                                | '^'
                                | '&'
                                | '*'
                                | '('
                                | ')'
                                | '['
                                | ']'
                                | '{'
                                | '}'
                        )
                    {
                        pos -= 1;
                    }

                    while pos < self.cursor {
                        self.text.remove(pos);
                        self.cursor -= 1;
                    }
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_commands() {
        let mut editor = StringEditor::new();

        // Text insertion
        editor.execute(Command::Insert('a'));
        editor.execute(Command::Insert('b'));
        assert_eq!(editor.get_text(), "ab");

        // Left and right cursor movement
        editor.execute(Command::CursorLeft(1));
        editor.execute(Command::Insert('c'));
        assert_eq!(editor.get_text(), "acb");
        editor.execute(Command::CursorRight(1));
        assert_eq!(editor.cursor_pos(), 3);
        editor.execute(Command::CursorRight(1));
        assert_eq!(editor.cursor_pos(), 3);
        editor.execute(Command::Insert('d'));
        assert_eq!(editor.get_text(), "acbd");

        // Start of string
        editor.execute(Command::CursorToStartOfLine);
        editor.execute(Command::Insert('e'));
        assert_eq!(editor.get_text(), "eacbd");

        // End of string
        editor.execute(Command::CursorToEndOfLine);
        editor.execute(Command::Insert('f'));
        assert_eq!(editor.get_text(), "eacbdf");

        // Delete
        editor.execute(Command::Delete);
        assert_eq!(editor.get_text(), "eacbdf");
        editor.execute(Command::Backspace);
        assert_eq!(editor.get_text(), "eacbd");
        editor.execute(Command::CursorLeft(1));
        editor.execute(Command::Backspace);
        assert_eq!(editor.get_text(), "eacd");
        editor.execute(Command::Delete);
        assert_eq!(editor.get_text(), "eac");
    }

    #[test]
    fn test_larger_edits() {
        let mut editor = StringEditor::with_string("Hello, world!");
        editor.execute(Command::CursorLeft(6));
        editor.execute(Command::DeleteStartOfLineToCursor);
        assert_eq!(editor.get_text(), "world!");
        editor.execute(Command::Type("Hello, ".to_string()));
        assert_eq!(editor.get_text(), "Hello, world!");
        editor.execute(Command::DeleteToEndOfLine);
        assert_eq!(editor.get_text(), "Hello, ");
        editor.execute(Command::Insert('w'));
        assert_eq!(editor.get_text(), "Hello, w");
    }
}
