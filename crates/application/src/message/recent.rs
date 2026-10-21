use domain::elements::{ChatId, Message};
use domain::ports::History;

use crate::UseCaseError;

/// Asks for a chat's most recent messages.
#[derive(Clone, Copy, Debug)]
pub struct RecentMessages {
    pub chat: ChatId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecentMessagesResult {
    /// Oldest first.
    pub messages: Vec<Message>,
}

/// Reads the most recent messages of `query.chat`.
pub async fn recent(
    query: RecentMessages,
    history: &dyn History,
) -> Result<RecentMessagesResult, UseCaseError> {
    let messages = history.recent(query.chat).await?;
    Ok(RecentMessagesResult { messages })
}
