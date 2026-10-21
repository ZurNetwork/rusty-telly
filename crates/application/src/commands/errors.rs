use domain::elements::MessageTextError;

use crate::UseCaseError;

/// Why a dispatched command did not run, or failed.
#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("no command `{id}` in this edition")]
    Unknown { id: String },
    #[error(transparent)]
    Unavailable(#[from] Unavailable),
    #[error("bad params: {0}")]
    BadParams(#[from] ParamProblem),
    #[error(transparent)]
    UseCase(#[from] UseCaseError),
}

/// Why a command cannot run right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Unavailable {
    #[error("no chat is open")]
    NoChatOpen,
}

/// What is wrong with a command's params.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ParamProblem {
    #[error("`{key}` is missing")]
    Missing { key: &'static str },
    #[error("`{key}` must be {expected}")]
    WrongType {
        key: &'static str,
        expected: &'static str,
    },
    #[error(transparent)]
    Text(#[from] MessageTextError),
}
