//! The command registry, PhotoCraft-style: every command a UI can run, by id
//! (`noun.verb`), with its label, its params and the editions that offer it.
//!
//! [`dispatch`] is the one door: it finds the command in the session's
//! edition, checks it can run now, and runs it with JSON params. A command an
//! edition does not offer does not exist there, which is how Limited has no way
//! to open a second chat (DD 2026101202 D6).

mod auth;
mod chat;
mod errors;
mod message;
mod params;
mod registry;
mod spec;

pub use errors::{CommandError, ParamProblem, Unavailable};
pub use registry::{dispatch, find, for_edition};
pub use spec::{CommandFuture, CommandSpec};
