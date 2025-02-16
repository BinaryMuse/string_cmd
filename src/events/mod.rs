#[cfg(feature = "crossterm")]
pub mod crossterm;

#[cfg(feature = "crossterm")]
pub use crate::events::crossterm::event_to_command;
