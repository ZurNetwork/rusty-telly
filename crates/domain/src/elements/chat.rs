//! A chat (a direct conversation, a group or a broadcast channel) and the
//! reference a Limited instance is spawned with.

mod entity;
mod id;
mod kind;
mod reference;
mod values;

pub use entity::Chat;
pub use id::ChatId;
pub use kind::ChatKind;
pub use reference::ChatRef;
pub use values::ChatTitle;
