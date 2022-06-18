use braindamage::*;

#[test]
fn test_parse_valid() {
    let result = parse("[->+<]");

    assert_eq!(
        result,
        Ok(vec![
            Token::LoopStart(5),
            Token::DecByte,
            Token::IncPtr,
            Token::IncByte,
            Token::DecPtr,
            Token::LoopEnd(0)
        ])
    );
}

#[test]
fn test_parse_unexpected_closing_bracket() {
    let result = parse("[->]+<]");

    assert_eq!(result, Err(ParseError::UnexpectedClosingBracket(6)));
}

#[test]
fn test_parse_expected_closing_bracket() {
    let result = parse("[->[[+<]");

    assert_eq!(result, Err(ParseError::ExpectedClosingBracket(2)));
}
