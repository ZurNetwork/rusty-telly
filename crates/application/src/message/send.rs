use domain::elements::{ChatId, Message, MessageText};
use domain::ports::Outbox;

use crate::UseCaseError;

/// Sends a text message to a chat.
#[derive(Clone, Debug)]
pub struct SendMessage {
    pub chat: ChatId,
    pub text: MessageText,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SendMessageResult {
    /// The message as Telegram stored it.
    pub message: Message,
}

/// Sends `command.text` to `command.chat`.
pub async fn send(
    command: SendMessage,
    outbox: &dyn Outbox,
) -> Result<SendMessageResult, UseCaseError> {
    let message = outbox.send(command.chat, command.text).await?;
    Ok(SendMessageResult { message })
}
