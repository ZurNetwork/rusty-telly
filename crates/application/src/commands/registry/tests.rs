use std::collections::HashSet;
use std::rc::Rc;

use adapter_mem::fixture;
use domain::elements::ChatRef;
use pollster::block_on;
use serde_json::json;

use super::*;
use crate::commands::ParamProblem;
use crate::{Edition, Ports, Session};

fn ids(edition: Edition) -> Vec<&'static str> {
    for_edition(edition).map(|spec| spec.id).collect()
}

fn demo_ports() -> Ports {
    Ports::from(Rc::new(fixture::demo()))
}

#[test]
fn limited_offers_no_way_to_open_another_chat() {
    assert_eq!(
        ids(Edition::Limited),
        [
            "auth.status",
            "chat.current",
            "message.recent",
            "message.send"
        ]
    );
}

#[test]
fn full_offers_every_command() {
    assert_eq!(
        ids(Edition::Full),
        [
            "auth.status",
            "chat.open",
            "chat.current",
            "message.recent",
            "message.send"
        ]
    );
}

#[test]
fn command_ids_are_unique() {
    let unique: HashSet<&'static str> = COMMANDS.iter().map(|spec| spec.id).collect();
    assert_eq!(unique.len(), COMMANDS.len());
}

#[test]
fn limited_cannot_dispatch_chat_open() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::direct().id));
    let open_group = json!({ "chat": fixture::group().id.as_i64() });

    let outcome = block_on(dispatch(&mut session, "chat.open", open_group));
    assert!(
        matches!(outcome, Err(CommandError::Unknown { .. })),
        "{outcome:?}"
    );
}

#[test]
fn limited_sends_to_its_spawned_chat() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::direct().id));

    let sent = block_on(dispatch(
        &mut session,
        "message.send",
        json!({ "text": "On my way!" }),
    ))
    .unwrap();
    let recent = block_on(dispatch(&mut session, "message.recent", json!({}))).unwrap();
    assert_eq!(sent["message"]["text"], "On my way!");
    assert_eq!(
        recent["messages"].as_array().unwrap().last(),
        Some(&sent["message"])
    );
}

#[test]
fn full_sends_only_once_a_chat_is_open() {
    let mut session = Session::full(demo_ports());
    let hello = json!({ "text": "Hello" });

    let before_open = block_on(dispatch(&mut session, "message.send", hello.clone()));
    assert!(
        matches!(
            before_open,
            Err(CommandError::Unavailable(Unavailable::NoChatOpen))
        ),
        "{before_open:?}"
    );

    let open_group = json!({ "chat": fixture::group().id.as_i64() });
    let opened = block_on(dispatch(&mut session, "chat.open", open_group)).unwrap();
    let sent = block_on(dispatch(&mut session, "message.send", hello)).unwrap();
    assert_eq!(opened["chat"]["title"], fixture::group().title.to_string());
    assert_eq!(sent["message"]["text"], "Hello");
}

#[test]
fn blank_text_is_bad_params() {
    let mut session = Session::limited(demo_ports(), ChatRef::from(fixture::direct().id));
    let blank_text_problem = ParamProblem::Text(domain::elements::MessageTextError::Blank);

    let outcome = block_on(dispatch(
        &mut session,
        "message.send",
        json!({ "text": "  " }),
    ));
    assert!(
        matches!(outcome, Err(CommandError::BadParams(problem)) if problem == blank_text_problem),
        "{outcome:?}"
    );
}
