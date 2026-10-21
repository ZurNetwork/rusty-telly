//! Reading and sending the messages of a chat.

mod recent;
mod send;

pub use recent::{RecentMessages, RecentMessagesResult, recent};
pub use send::{SendMessage, SendMessageResult, send};
