//! A demo account: signed in, with a direct chat, a group and a broadcast
//! channel in its dialogs, and a short conversation in the direct chat.

use domain::elements::{
    Chat, ChatId, ChatKind, ChatTitle, Message, MessageId, MessageText, UserId,
};

use crate::MemoryTelegram;

/// The signed-in user.
pub fn me() -> UserId {
    UserId::from(1)
}

/// The other user in [`direct`].
pub fn artist() -> UserId {
    UserId::from(2)
}

pub fn direct() -> Chat {
    Chat {
        id: ChatId::from(100),
        kind: ChatKind::Direct,
        title: ChatTitle::from("Artist".to_owned()),
    }
}

pub fn group() -> Chat {
    Chat {
        id: ChatId::from(200),
        kind: ChatKind::Group,
        title: ChatTitle::from("The Den".to_owned()),
    }
}

pub fn broadcast() -> Chat {
    Chat {
        id: ChatId::from(300),
        kind: ChatKind::Broadcast,
        title: ChatTitle::from("Den News".to_owned()),
    }
}

#[expect(
    clippy::expect_used,
    reason = "every fixture text is a literal that is a valid MessageText"
)]
fn message(id: i32, author: UserId, text: &str) -> Message {
    let text = MessageText::try_from(text).expect("fixture text is valid");
    Message {
        id: MessageId::from(id),
        chat: direct().id,
        author,
        text,
    }
}

/// The demo account.
pub fn demo() -> MemoryTelegram {
    let conversation = [
        message(1, artist(), "Hi! Your commission slot is open."),
        message(2, me(), "Great, sending the references now."),
    ];
    MemoryTelegram::signed_in(me(), [direct(), group(), broadcast()], conversation)
}
