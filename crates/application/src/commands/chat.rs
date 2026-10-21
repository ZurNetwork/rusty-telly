use domain::elements::{Chat, ChatRef};
use serde_json::{Value, json};

use super::{CommandFuture, params};
use crate::Session;

fn chat_json(chat: &Chat) -> Value {
    let kind: &'static str = chat.kind.into();
    json!({ "chat": { "id": chat.id.as_i64(), "kind": kind, "title": chat.title.to_string() } })
}

pub(super) fn open(session: &mut Session, params: Value) -> CommandFuture<'_> {
    Box::pin(async move {
        let reference = ChatRef::from(params::chat_id(&params, "chat")?);
        let chat = session.open_chat(reference).await?;
        Ok(chat_json(chat))
    })
}

pub(super) fn current(session: &mut Session, _params: Value) -> CommandFuture<'_> {
    Box::pin(async move {
        let chat = session.chat().await?;
        Ok(chat_json(chat))
    })
}
