use async_trait::async_trait;

use super::PortError;
use crate::elements::{ChatId, Message};

/// The messages already in a chat.
#[async_trait(?Send)]
pub trait History {
    /// The chat's most recent messages, oldest first. The adapter picks the page size.
    async fn recent(&self, chat: ChatId) -> Result<Vec<Message>, PortError>;
}
