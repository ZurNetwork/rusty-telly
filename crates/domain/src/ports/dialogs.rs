use async_trait::async_trait;

use super::PortError;
use crate::elements::{Chat, ChatRef};

/// The signed-in user's own dialogs: the only place a [`ChatRef`] resolves.
#[async_trait(?Send)]
pub trait Dialogs {
    /// The chat `reference` names among the user's own dialogs, or
    /// [`PortError::ChatNotFound`] when they have no such dialog.
    async fn resolve(&self, reference: ChatRef) -> Result<Chat, PortError>;
}
