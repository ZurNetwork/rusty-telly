use super::ChatId;

/// The chat a Limited instance is spawned with, before it is resolved.
///
/// It comes from outside (the instance's URL fragment), so it is untrusted: it
/// resolves only among the signed-in user's own dialogs, and it never carries an
/// access hash, since those are per account and per login session. The
/// fragment's versioned format is still open (DD 2026101202, "The Linked
/// Channel tie").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, derive_more::Display, derive_more::From)]
pub struct ChatRef(ChatId);

impl ChatRef {
    /// The chat id this reference names.
    pub fn chat_id(self) -> ChatId {
        self.0
    }
}
