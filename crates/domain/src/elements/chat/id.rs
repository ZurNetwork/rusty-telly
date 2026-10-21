/// A chat's id: Telegram's peer id for it.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
)]
pub struct ChatId(i64);

impl ChatId {
    /// The peer id, for an adapter speaking to Telegram.
    pub fn as_i64(self) -> i64 {
        self.0
    }
}
