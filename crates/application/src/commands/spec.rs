use std::future::Future;
use std::pin::Pin;

use serde_json::Value;

use super::{CommandError, Unavailable};
use crate::{Edition, Session};

/// What a command's body returns: a single-threaded future borrowing the session.
pub type CommandFuture<'s> = Pin<Box<dyn Future<Output = Result<Value, CommandError>> + 's>>;

type Run = for<'s> fn(&'s mut Session, Value) -> CommandFuture<'s>;
type Enabled = fn(&Session) -> Result<(), Unavailable>;

/// Metadata and implementation for one command.
pub struct CommandSpec {
    /// `noun.verb`, e.g. `message.send`.
    pub id: &'static str,
    pub label: &'static str,
    /// Human- and agent-readable description of the params (JSON-ish).
    pub params: &'static str,
    /// The editions that offer it.
    pub editions: &'static [Edition],
    /// Whether it can run now, from the session's state alone (no I/O).
    pub enabled: Enabled,
    pub run: Run,
}
