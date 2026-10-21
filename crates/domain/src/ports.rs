//! Ports: the traits the core is served through, named by the role they play
//! and implemented by the adapter crates.
//!
//! They are `?Send`. In the browser no future is `Send`, and one core serves the
//! browser and the desktop (DD 2026101202 D1).

mod authorization;
mod dialogs;
mod errors;
mod history;
mod outbox;

pub use authorization::{AuthStatus, Authorization};
pub use dialogs::Dialogs;
pub use errors::PortError;
pub use history::History;
pub use outbox::Outbox;
