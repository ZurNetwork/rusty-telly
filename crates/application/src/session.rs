use domain::elements::{Chat, ChatRef};

use crate::chat::{self, OpenChat};
use crate::{Edition, Ports, UseCaseError};

/// What a UI drives: the ports, the edition it serves and the chat it has open.
pub struct Session {
    ports: Ports,
    edition: Edition,
    chat: ChatSlot,
}

/// The chat a session has open.
enum ChatSlot {
    /// Full, before any chat is opened.
    Empty,
    /// Limited, before its first command: the chat it was spawned with.
    Spawned(ChatRef),
    Open(Chat),
}

impl Session {
    /// A Limited session, fixed to `chat` for its whole life. Nothing talks to
    /// Telegram here: the chat resolves on the first command that needs it, so
    /// nothing happens at spawn without a click (DD 2026101202 D6).
    pub fn limited(ports: Ports, chat: ChatRef) -> Self {
        Self {
            ports,
            edition: Edition::Limited,
            chat: ChatSlot::Spawned(chat),
        }
    }

    /// A Full session with no chat open.
    pub fn full(ports: Ports) -> Self {
        Self {
            ports,
            edition: Edition::Full,
            chat: ChatSlot::Empty,
        }
    }

    pub fn edition(&self) -> Edition {
        self.edition
    }

    pub fn ports(&self) -> &Ports {
        &self.ports
    }

    /// Whether a chat is open or will open on first use. No I/O.
    pub fn has_chat(&self) -> bool {
        !matches!(self.chat, ChatSlot::Empty)
    }

    /// The open chat. A Limited session resolves the chat it was spawned with
    /// the first time this is called.
    pub async fn chat(&mut self) -> Result<&Chat, UseCaseError> {
        if let ChatSlot::Spawned(reference) = self.chat {
            let opened = chat::open(OpenChat { reference }, self.ports.dialogs.as_ref()).await?;
            self.chat = ChatSlot::Open(opened.chat);
        }
        let ChatSlot::Open(chat) = &self.chat else {
            return Err(UseCaseError::NoChatOpen);
        };
        Ok(chat)
    }

    /// Opens the chat `reference` names, replacing the open one. A Limited
    /// session refuses: its chat was fixed at spawn.
    pub async fn open_chat(&mut self, reference: ChatRef) -> Result<&Chat, UseCaseError> {
        if self.edition == Edition::Limited {
            return Err(UseCaseError::ChatFixed);
        }
        let opened = chat::open(OpenChat { reference }, self.ports.dialogs.as_ref()).await?;
        self.chat = ChatSlot::Open(opened.chat);
        self.chat().await
    }
}

#[cfg(test)]
mod tests;
