use strum::VariantArray;

use super::*;

#[test]
fn tokens_are_pinned() {
    let tokens: Vec<&'static str> = Edition::VARIANTS.iter().map(<&'static str>::from).collect();
    assert_eq!(tokens, ["limited", "full"]);
}

#[test]
fn tokens_round_trip() {
    for edition in Edition::VARIANTS {
        let parsed: Edition = edition.to_string().parse().unwrap();
        assert_eq!(parsed, *edition);
    }
}
