use domain::elements::{ChatId, MessageText};
use serde_json::Value;

use super::ParamProblem;

fn field<'p>(params: &'p Value, key: &'static str) -> Result<&'p Value, ParamProblem> {
    params.get(key).ok_or(ParamProblem::Missing { key })
}

/// `params[key]` as a chat id: a JSON integer.
pub(super) fn chat_id(params: &Value, key: &'static str) -> Result<ChatId, ParamProblem> {
    let raw = field(params, key)?
        .as_i64()
        .ok_or(ParamProblem::WrongType {
            key,
            expected: "an integer",
        })?;
    Ok(ChatId::from(raw))
}

/// `params[key]` as message text: a JSON string that is a valid [`MessageText`].
pub(super) fn message_text(params: &Value, key: &'static str) -> Result<MessageText, ParamProblem> {
    let raw = field(params, key)?
        .as_str()
        .ok_or(ParamProblem::WrongType {
            key,
            expected: "a string",
        })?;
    MessageText::try_from(raw).map_err(ParamProblem::from)
}
