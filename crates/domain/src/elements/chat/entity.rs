use super::{ChatId, ChatKind, ChatTitle};

/// A chat in the signed-in user's dialogs, resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chat {
    pub id: ChatId,
    pub kind: ChatKind,
    pub title: ChatTitle,
}
