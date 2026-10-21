/// A message's id, unique within its chat.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Display,
    derive_more::From,
    derive_more::FromStr,
)]
pub struct MessageId(i32);

impl MessageId {
    /// The id, for an adapter speaking to Telegram.
    pub fn as_i32(self) -> i32 {
        self.0
    }
}
