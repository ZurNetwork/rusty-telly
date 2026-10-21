/// Which client a [`Session`](crate::Session) serves. Each command names the
/// editions that offer it.
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
pub enum Edition {
    /// Unofficial Mini Telegram, in the browser: exactly one chat, fixed when
    /// the instance is spawned.
    Limited,
    /// Unofficial Rusty Telegram, on the desktop.
    Full,
}

#[cfg(test)]
mod tests;
