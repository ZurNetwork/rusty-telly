use domain::elements::{Chat, ChatRef};
use domain::ports::Dialogs;

use crate::UseCaseError;

/// Opens the chat a reference names.
#[derive(Clone, Copy, Debug)]
pub struct OpenChat {
    pub reference: ChatRef,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenChatResult {
    pub chat: Chat,
}

/// Resolves `command.reference` among the user's own dialogs. A chat whose kind
/// may not be opened (a broadcast channel) is refused.
pub async fn open(
    command: OpenChat,
    dialogs: &dyn Dialogs,
) -> Result<OpenChatResult, UseCaseError> {
    let chat = dialogs.resolve(command.reference).await?;
    if !chat.kind.may_open() {
        return Err(UseCaseError::ChatClosed { kind: chat.kind });
    }
    Ok(OpenChatResult { chat })
}
