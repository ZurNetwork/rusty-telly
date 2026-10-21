//! An in-memory Telegram: every domain port served from seeded data, with no
//! network. [`fixture::demo`] is a ready-made account for tests and the CLI.

pub mod fixture;

mod telegram;

pub use telegram::MemoryTelegram;
