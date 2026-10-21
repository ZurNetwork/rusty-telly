use domain::ports::{AuthStatus, Authorization};

use crate::UseCaseError;

/// Asks whether this client holds a signed-in login.
#[derive(Clone, Copy, Debug, Default)]
pub struct StatusQuery;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusResult {
    pub status: AuthStatus,
}

/// Reports whether this client holds a signed-in Telegram login.
pub async fn status(
    _query: StatusQuery,
    authorization: &dyn Authorization,
) -> Result<StatusResult, UseCaseError> {
    let status = authorization.status().await?;
    Ok(StatusResult { status })
}
