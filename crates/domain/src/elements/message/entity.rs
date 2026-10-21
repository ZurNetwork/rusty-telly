use super::{MessageId, MessageText};
use crate::elements::{ChatId, UserId};

/// One text message in a chat.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: MessageId,
    pub chat: ChatId,
    pub author: UserId,
    pub text: MessageText,
}
