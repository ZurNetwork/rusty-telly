use super::*;

#[test]
fn blank_text_is_refused() {
    let blank_error = MessageText::try_from(" \n\t").unwrap_err();
    assert_eq!(blank_error, MessageTextError::Blank);
}

#[test]
fn text_at_the_limit_is_kept_as_written() {
    let longest = "a".repeat(MessageText::MAX_LENGTH);
    let text = MessageText::try_from(longest.as_str()).unwrap();
    assert_eq!(text.to_string(), longest);
}

#[test]
fn text_over_the_limit_is_refused() {
    let too_long = "a".repeat(MessageText::MAX_LENGTH + 1);
    let too_long_error = MessageTextError::TooLong {
        length: MessageText::MAX_LENGTH + 1,
    };
    assert_eq!(MessageText::try_from(too_long), Err(too_long_error));
}

#[test]
fn length_counts_utf16_units_not_chars() {
    // An emoji outside the Basic Multilingual Plane is one char but two UTF-16 units.
    let emoji = "🦊".repeat(MessageText::MAX_LENGTH / 2 + 1);
    let too_long_error = MessageTextError::TooLong {
        length: MessageText::MAX_LENGTH + 2,
    };
    assert_eq!(MessageText::try_from(emoji), Err(too_long_error));
}
