//! The domain of the Unofficial Telegram clients: the pure-data model and the
//! ports it is served through, with no I/O of its own.
//!
//! [`elements`] holds the nouns (chats, messages, users) as plain values;
//! [`ports`] holds the role-named traits the adapter crates implement. The
//! dependency rule runs one way: `application` and the adapters point at this
//! crate, and `elements` never imports `ports`.

pub mod elements;
pub mod ports;
