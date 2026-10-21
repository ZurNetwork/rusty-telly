use crate::elements::ChatRef;

/// Why a port call failed.
#[derive(Debug, thiserror::Error)]
pub enum PortError {
    #[error("not signed in to Telegram")]
    SignedOut,
    #[error("no dialog with chat {0}")]
    ChatNotFound(ChatRef),
    #[error("the connection to Telegram failed")]
    Transport(#[source] Box<dyn std::error::Error>),
}
