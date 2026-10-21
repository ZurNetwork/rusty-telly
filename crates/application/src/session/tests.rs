use std::rc::Rc;

use adapter_mem::fixture;
use domain::elements::{ChatKind, ChatRef};
use pollster::block_on;

use super::*;

fn demo_ports() -> Ports {
    Ports::from(Rc::new(fixture::demo()))
}

#[test]
fn limited_opens_its_spawned_chat_on_first_use() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::direct().id));

    let chat = block_on(session.chat()).unwrap().clone();
    assert_eq!(chat, fixture::direct());
}

#[test]
fn limited_refuses_to_open_another_chat() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::direct().id));

    let refusal = block_on(session.open_chat(ChatRef::from(fixture::group().id))).unwrap_err();
    assert!(matches!(refusal, UseCaseError::ChatFixed), "{refusal:?}");
}

#[test]
fn limited_spawned_on_a_broadcast_channel_never_opens_it() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::broadcast().id));

    let refusal = block_on(session.chat()).unwrap_err();
    assert!(
        matches!(
            refusal,
            UseCaseError::ChatClosed {
                kind: ChatKind::Broadcast
            }
        ),
        "{refusal:?}"
    );
}

#[test]
fn full_has_no_chat_until_one_is_opened() {
    let mut session = Session::full(demo_ports());
    assert!(!session.has_chat());

    let none_open = block_on(session.chat()).unwrap_err();
    assert!(
        matches!(none_open, UseCaseError::NoChatOpen),
        "{none_open:?}"
    );

    let opened = block_on(session.open_chat(ChatRef::from(fixture::group().id)))
        .unwrap()
        .clone();
    assert_eq!(opened, fixture::group());
}

#[test]
fn full_refuses_a_broadcast_channel() {
    let mut session = Session::full(demo_ports());

    let refusal = block_on(session.open_chat(ChatRef::from(fixture::broadcast().id))).unwrap_err();
    assert!(
        matches!(
            refusal,
            UseCaseError::ChatClosed {
                kind: ChatKind::Broadcast
            }
        ),
        "{refusal:?}"
    );
}
