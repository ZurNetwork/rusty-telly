use async_trait::async_trait;

use super::PortError;
use crate::elements::{ChatId, Message, MessageText};

/// Where outgoing messages go.
#[async_trait(?Send)]
pub trait Outbox {
    /// Sends `text` to `chat`, and returns the message as Telegram stored it.
    async fn send(&self, chat: ChatId, text: MessageText) -> Result<Message, PortError>;
}
