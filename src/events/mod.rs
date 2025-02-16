#[cfg(feature = "crossterm")]
/// Crossterm event handling (requires the `crossterm` feature)
pub mod crossterm;

#[cfg(feature = "crossterm")]
/// Convert a crossterm event to a command using the default keybindings.
pub use crate::events::crossterm::event_to_command;
