use serde_json::Value;

use super::{CommandError, CommandSpec, Unavailable, auth, chat, message};
use crate::{Edition, Session};

const BOTH: &[Edition] = &[Edition::Limited, Edition::Full];
const FULL: &[Edition] = &[Edition::Full];

static COMMANDS: &[CommandSpec] = &[
    CommandSpec {
        id: "auth.status",
        label: "Sign-in status",
        params: "{}",
        editions: BOTH,
        enabled: always,
        run: auth::status,
    },
    CommandSpec {
        id: "chat.open",
        label: "Open chat",
        params: r#"{"chat": <chat id>}"#,
        editions: FULL,
        enabled: always,
        run: chat::open,
    },
    CommandSpec {
        id: "chat.current",
        label: "Current chat",
        params: "{}",
        editions: BOTH,
        enabled: needs_chat,
        run: chat::current,
    },
    CommandSpec {
        id: "message.recent",
        label: "Recent messages",
        params: "{}",
        editions: BOTH,
        enabled: needs_chat,
        run: message::recent,
    },
    CommandSpec {
        id: "message.send",
        label: "Send message",
        params: r#"{"text": <string>}"#,
        editions: BOTH,
        enabled: needs_chat,
        run: message::send,
    },
];

fn always(_: &Session) -> Result<(), Unavailable> {
    Ok(())
}

fn needs_chat(session: &Session) -> Result<(), Unavailable> {
    if !session.has_chat() {
        return Err(Unavailable::NoChatOpen);
    }
    Ok(())
}

/// Every command `edition` offers, in registry order.
pub fn for_edition(edition: Edition) -> impl Iterator<Item = &'static CommandSpec> {
    COMMANDS
        .iter()
        .filter(move |spec| spec.editions.contains(&edition))
}

/// The command `id`, if `edition` offers it.
pub fn find(edition: Edition, id: &str) -> Option<&'static CommandSpec> {
    for_edition(edition).find(|spec| spec.id == id)
}

/// Runs command `id` on `session` with `params`, if the session's edition
/// offers it and it can run now.
pub async fn dispatch(
    session: &mut Session,
    id: &str,
    params: Value,
) -> Result<Value, CommandError> {
    let spec =
        find(session.edition(), id).ok_or_else(|| CommandError::Unknown { id: id.to_owned() })?;
    (spec.enabled)(session)?;
    (spec.run)(session, params).await
}

#[cfg(test)]
mod tests;
