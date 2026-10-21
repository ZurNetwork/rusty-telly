use domain::ports::AuthStatus;
use serde_json::{Value, json};

use super::CommandFuture;
use crate::Session;
use crate::auth::{self, StatusQuery, StatusResult};

impl From<StatusResult> for Value {
    fn from(result: StatusResult) -> Self {
        match result.status {
            AuthStatus::SignedOut => json!({ "signed_in": false }),
            AuthStatus::SignedIn { user } => json!({ "signed_in": true, "user": user.as_i64() }),
        }
    }
}

pub(super) fn status(session: &mut Session, _params: Value) -> CommandFuture<'_> {
    Box::pin(async move {
        let result = auth::status(StatusQuery, session.ports().authorization.as_ref()).await?;
        Ok(Value::from(result))
    })
}
