#[derive(Debug, Clone)]
pub(crate) enum EditingMode {
    Emacs,
    Vi(ViMode),
}

impl Default for EditingMode {
    fn default() -> Self {
        Self::Emacs
    }
}

/// A command to be executed on the editor. See [`crate::StringEditor::execute`].
#[derive(Debug, Clone)]
pub enum Command {
    /// Insert a character at the cursor.
    Insert(char),
    /// Insert a string at the cursor.
    Type(String),
    /// Move the cursor to the start of the line.
    CursorToStartOfLine,
    /// Move the cursor to the end of the line.
    CursorToEndOfLine,
    /// Move the cursor left by a given number of characters.
    CursorLeft(usize),
    /// Move the cursor right by a given number of characters.
    CursorRight(usize),
    /// Delete the character at the cursor.
    Delete,
    /// Delete the character before the cursor.
    Backspace,
    /// TODO
    Transpose,
    /// Delete from the start of the line to the cursor.
    DeleteStartOfLineToCursor,
    /// Delete the word leading to the cursor.
    DeleteWordLeadingToCursor,
    /// Paste the last yanked text at the cursor. (TBI)
    PasteFromYank,
    /// Undo the last command. (TBI)
    Undo,

    /// Delete to the end of the line.
    DeleteToEndOfLine,
    /// Move the cursor to the previous word. (TBI)
    CursorToPreviousWord,
    /// Capitalize the current word. (TBI)
    CapitalizeCurrentWord,
    /// Delete the current word. (TBI)
    DeleteWord,
    /// Move the cursor to the next word.
    CursorToNextWord,
    /// Lowercase the next word. (TBI)
    LowerCaseNextWord,
    /// Uppercase the next word. (TBI)
    UpperCaseNextWord,
    /// Transpose the words before and after the cursor. (TBI)
    TransposeWords,
    // vi
    // ChangeMode(ViMode),
    // RedoLastTextMod,
    // RedoLastCharFind,
    // RedoLastCharFindBackwards,
    // CursorToFirstNonBlankOfLine,
    // InsertAfterCursor,
    // InsertAtEndOfLine,
    // CursorTokenLeft,
    // CursorNonBlankWordLeft,
    // ChangeText(Movement),
    // DeleteText(Movement),
    // InsertBeforeCursor,
    // InsertAtBeginningOfLine,
    // PasteYankedAtCursor,
    // PasteYankedBeforeCursor,
    // ReplaceChar(char),
    // DeleteCharThenMode(ViMode),
    // ChangeCurrentLine,
    // Yank(Movement),
}

#[derive(Debug, Clone)]
pub(crate) enum ViMode {
    Command,
    Insert,
}

#[derive(Debug, Clone)]
pub(crate) enum Movement {
    ToEndOfLine,
    WordOrTokenLeft,
    NonBlankWordLeft,
    WordOrTokenRight,
    NonBlankWordRight,
    EndOfCurrentWord,
    EndOfCurrentNonBlankWord,
    NextOccurrenceOfChar(char),
    PreviousOccurrenceOfChar(char),
    CharLeft,
    CharRight,
    BeforeNextOccurrenceOfChar(char),
    AfterPreviousOccurrenceOfChar(char),
}
