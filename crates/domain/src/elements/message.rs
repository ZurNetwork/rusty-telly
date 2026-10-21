//! A message in a chat. Text only for now; stickers join in A3 (DD 2026101202 D10).

mod entity;
mod errors;
mod id;
mod text;

pub use entity::Message;
pub use errors::MessageTextError;
pub use id::MessageId;
pub use text::MessageText;
