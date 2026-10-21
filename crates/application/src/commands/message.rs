use domain::elements::Message;
use serde_json::{Value, json};

use super::{CommandFuture, params};
use crate::Session;
use crate::message::{self, RecentMessages, RecentMessagesResult, SendMessage, SendMessageResult};

fn message_json(message: &Message) -> Value {
    json!({ "id": message.id.as_i32(), "author": message.author.as_i64(), "text": message.text.to_string() })
}

impl From<RecentMessagesResult> for Value {
    fn from(result: RecentMessagesResult) -> Self {
        let messages: Vec<Value> = result.messages.iter().map(message_json).collect();
        json!({ "messages": messages })
    }
}

impl From<SendMessageResult> for Value {
    fn from(result: SendMessageResult) -> Self {
        json!({ "message": message_json(&result.message) })
    }
}

pub(super) fn recent(session: &mut Session, _params: Value) -> CommandFuture<'_> {
    Box::pin(async move {
        let chat = session.chat().await?.id;
        let result =
            message::recent(RecentMessages { chat }, session.ports().history.as_ref()).await?;
        Ok(Value::from(result))
    })
}

pub(super) fn send(session: &mut Session, params: Value) -> CommandFuture<'_> {
    Box::pin(async move {
        let text = params::message_text(&params, "text")?;
        let chat = session.chat().await?.id;
        let result =
            message::send(SendMessage { chat, text }, session.ports().outbox.as_ref()).await?;
        Ok(Value::from(result))
    })
}
