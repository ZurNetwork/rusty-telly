use pollster::block_on;

use super::*;
use crate::fixture;

#[test]
fn send_appends_with_the_next_id_in_the_chat() {
    let telegram = fixture::demo();
    let chat = fixture::direct().id;
    let last_before = block_on(telegram.recent(chat))
        .unwrap()
        .last()
        .map(|message| message.id)
        .unwrap();

    let text = MessageText::try_from("Sketch attached").unwrap();
    let sent = block_on(telegram.send(chat, text)).unwrap();
    let after = block_on(telegram.recent(chat)).unwrap();
    assert_eq!(sent.id.as_i32(), last_before.as_i32() + 1);
    assert_eq!(after.last(), Some(&sent));
}

#[test]
fn a_chat_outside_the_dialogs_is_not_found() {
    let telegram = fixture::demo();
    let stranger = ChatRef::from(ChatId::from(999));

    let not_found = block_on(telegram.resolve(stranger)).unwrap_err();
    assert!(
        matches!(not_found, PortError::ChatNotFound(reference) if reference == stranger),
        "{not_found:?}"
    );
}

#[test]
fn a_signed_out_account_serves_nothing() {
    let telegram = MemoryTelegram::default();
    let chat = fixture::direct().id;

    assert_eq!(block_on(telegram.status()).unwrap(), AuthStatus::SignedOut);
    assert!(matches!(
        block_on(telegram.recent(chat)),
        Err(PortError::SignedOut)
    ));
}
