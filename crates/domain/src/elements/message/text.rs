use super::MessageTextError;

/// The text of one message: not blank, and at most [`MessageText::MAX_LENGTH`]
/// UTF-16 units, the unit Telegram counts in.
#[derive(Clone, Debug, PartialEq, Eq, derive_more::Display, derive_more::AsRef)]
#[as_ref(str)]
pub struct MessageText(String);

impl MessageText {
    /// The server's default `message_length_limit_default`, as tdesktop reads it
    /// (`data/data_premium_limits.cpp`).
    pub const MAX_LENGTH: usize = 4096;
}

impl TryFrom<String> for MessageText {
    type Error = MessageTextError;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        if text.trim().is_empty() {
            return Err(MessageTextError::Blank);
        }
        let length = text.encode_utf16().count();
        if length > Self::MAX_LENGTH {
            return Err(MessageTextError::TooLong { length });
        }
        Ok(Self(text))
    }
}

impl TryFrom<&str> for MessageText {
    type Error = MessageTextError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Self::try_from(text.to_owned())
    }
}

#[cfg(test)]
mod tests;
