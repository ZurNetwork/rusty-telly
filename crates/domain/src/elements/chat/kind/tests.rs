use strum::VariantArray;

use super::*;

#[test]
fn tokens_are_pinned() {
    let tokens: Vec<&'static str> = ChatKind::VARIANTS
        .iter()
        .map(<&'static str>::from)
        .collect();
    assert_eq!(tokens, ["direct", "group", "broadcast"]);
}

#[test]
fn tokens_round_trip() {
    for kind in ChatKind::VARIANTS {
        let parsed: ChatKind = kind.to_string().parse().unwrap();
        assert_eq!(parsed, *kind);
    }
}

#[test]
fn only_a_broadcast_channel_stays_closed() {
    let openable: Vec<ChatKind> = ChatKind::VARIANTS
        .iter()
        .copied()
        .filter(|kind| kind.may_open())
        .collect();
    assert_eq!(openable, [ChatKind::Direct, ChatKind::Group]);
}
