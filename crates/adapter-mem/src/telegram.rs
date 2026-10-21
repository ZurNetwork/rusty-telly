use std::cell::RefCell;

use async_trait::async_trait;
use domain::elements::{Chat, ChatId, ChatRef, Message, MessageId, MessageText, UserId};
use domain::ports::{AuthStatus, Authorization, Dialogs, History, Outbox, PortError};

/// A Telegram account held in memory: whether it is signed in, its dialogs,
/// and their messages. Single-threaded, like the ports it serves.
#[derive(Debug, Default)]
pub struct MemoryTelegram {
    user: Option<UserId>,
    chats: Vec<Chat>,
    messages: RefCell<Vec<Message>>,
}

impl MemoryTelegram {
    /// An account signed in as `user`, with `chats` in its dialogs and
    /// `messages` in their history. [`Default`] is a signed-out account.
    pub fn signed_in(
        user: UserId,
        chats: impl IntoIterator<Item = Chat>,
        messages: impl IntoIterator<Item = Message>,
    ) -> Self {
        Self {
            user: Some(user),
            chats: chats.into_iter().collect(),
            messages: RefCell::new(messages.into_iter().collect()),
        }
    }

    fn signed_in_user(&self) -> Result<UserId, PortError> {
        self.user.ok_or(PortError::SignedOut)
    }

    fn dialog(&self, chat: ChatId) -> Result<&Chat, PortError> {
        self.chats
            .iter()
            .find(|dialog| dialog.id == chat)
            .ok_or(PortError::ChatNotFound(ChatRef::from(chat)))
    }
}

#[async_trait(?Send)]
impl Authorization for MemoryTelegram {
    async fn status(&self) -> Result<AuthStatus, PortError> {
        let status = match self.user {
            Some(user) => AuthStatus::SignedIn { user },
            None => AuthStatus::SignedOut,
        };
        Ok(status)
    }
}

#[async_trait(?Send)]
impl Dialogs for MemoryTelegram {
    async fn resolve(&self, reference: ChatRef) -> Result<Chat, PortError> {
        self.signed_in_user()?;
        self.dialog(reference.chat_id()).cloned()
    }
}

#[async_trait(?Send)]
impl History for MemoryTelegram {
    async fn recent(&self, chat: ChatId) -> Result<Vec<Message>, PortError> {
        self.signed_in_user()?;
        self.dialog(chat)?;
        let in_chat: Vec<Message> = self
            .messages
            .borrow()
            .iter()
            .filter(|message| message.chat == chat)
            .cloned()
            .collect();
        Ok(in_chat)
    }
}

#[async_trait(?Send)]
impl Outbox for MemoryTelegram {
    async fn send(&self, chat: ChatId, text: MessageText) -> Result<Message, PortError> {
        let author = self.signed_in_user()?;
        self.dialog(chat)?;
        let mut messages = self.messages.borrow_mut();
        let last_id = messages
            .iter()
            .filter(|message| message.chat == chat)
            .map(|message| message.id.as_i32())
            .max();
        let id = MessageId::from(last_id.unwrap_or(0) + 1);
        let message = Message {
            id,
            chat,
            author,
            text,
        };
        messages.push(message.clone());
        Ok(message)
    }
}

#[cfg(test)]
mod tests;
