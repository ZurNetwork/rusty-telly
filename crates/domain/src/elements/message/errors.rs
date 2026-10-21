use super::MessageText;

/// Why a string is not a [`MessageText`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MessageTextError {
    #[error("a message cannot be blank")]
    Blank,
    #[error("a message is at most {max} UTF-16 units long, this one is {length}", max = MessageText::MAX_LENGTH)]
    TooLong { length: usize },
}
