/// What kind of chat a [`Chat`](super::Chat) is.
///
/// A supergroup is a [`Group`](ChatKind::Group): Telegram serves it as a
/// channel, but people talk in it the way they do in a group.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    strum::Display,
    strum::EnumString,
    strum::IntoStaticStr,
    strum::VariantArray,
)]
#[strum(serialize_all = "snake_case")]
pub enum ChatKind {
    /// A one-to-one conversation with another user.
    Direct,
    /// A basic group or a supergroup.
    Group,
    /// A broadcast channel, where only its admins post.
    Broadcast,
}

impl ChatKind {
    /// Whether the clients may open a chat of this kind. Broadcast channels stay
    /// closed until sponsored messages are shown (Telegram API terms §3.3;
    /// DD 2026101202 D6).
    pub fn may_open(self) -> bool {
        !matches!(self, Self::Broadcast)
    }
}

#[cfg(test)]
mod tests;
