//! The nouns of the model: plain values, with no I/O and no port.

mod chat;
mod message;
mod user;

pub use chat::{Chat, ChatId, ChatKind, ChatRef, ChatTitle};
pub use message::{Message, MessageId, MessageText, MessageTextError};
pub use user::UserId;
