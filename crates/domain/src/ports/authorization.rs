use async_trait::async_trait;

use super::PortError;
use crate::elements::UserId;

/// Whether this client holds a signed-in Telegram login.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthStatus {
    SignedOut,
    SignedIn { user: UserId },
}

/// The Telegram login this client holds. It never leaves the client's own
/// origin and never touches Zurfur (DD 2026101202 D4). Signing in (QR and 2FA,
/// then phone and code) lands here in A1.
#[async_trait(?Send)]
pub trait Authorization {
    /// The login's status.
    async fn status(&self) -> Result<AuthStatus, PortError>;
}
