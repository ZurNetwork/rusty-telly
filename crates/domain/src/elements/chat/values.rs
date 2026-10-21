/// A chat's title as Telegram reports it. For a direct chat it is the other
/// user's name.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, derive_more::Display, derive_more::From, derive_more::AsRef,
)]
#[as_ref(str)]
pub struct ChatTitle(String);
