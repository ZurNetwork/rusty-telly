//! The command engine: the use cases over the domain's ports, the [`Session`] a
//! UI drives, and the [`commands`] registry that names every command,
//! PhotoCraft-style (DD 2026101202 D1).
//!
//! A use case is a plain `async fn` with its ports in its signature, so it is
//! go-to-definition from a test or another use case. The registry is the door
//! a UI or the CLI drives: it maps a command id and JSON params onto those
//! functions, filtered by [`Edition`], so Limited's surface cannot name a
//! command it must not have.

pub mod auth;
pub mod chat;
pub mod commands;
pub mod message;

mod edition;
mod errors;
mod ports;
mod session;

pub use edition::Edition;
pub use errors::UseCaseError;
pub use ports::Ports;
pub use session::Session;
