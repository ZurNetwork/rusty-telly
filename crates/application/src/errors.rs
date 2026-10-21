use domain::elements::ChatKind;
use domain::ports::PortError;

/// Why a use case failed.
#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error(transparent)]
    Port(#[from] PortError),
    #[error("no chat is open")]
    NoChatOpen,
    #[error("a {kind} chat cannot be opened")]
    ChatClosed { kind: ChatKind },
    #[error("this session's chat was fixed when it was spawned")]
    ChatFixed,
}
