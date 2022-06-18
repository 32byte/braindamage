use braindamage::Token;

#[test]
fn parse_inc_ptr() {
    assert_eq!(Token::from_char('>').unwrap(), Token::IncPtr);
}

#[test]
fn parse_dec_ptr() {
    assert_eq!(Token::from_char('<').unwrap(), Token::DecPtr);
}

#[test]
fn parse_inc_byte() {
    assert_eq!(Token::from_char('+').unwrap(), Token::IncByte);
}

#[test]
fn parse_dec_byte() {
    assert_eq!(Token::from_char('-').unwrap(), Token::DecByte);
}

#[test]
fn parse_write_byte() {
    assert_eq!(Token::from_char('.').unwrap(), Token::WriteByte);
}

#[test]
fn parse_read_byte() {
    assert_eq!(Token::from_char(',').unwrap(), Token::ReadByte);
}

#[test]
fn parse_loop_start() {
    assert_eq!(Token::from_char('[').unwrap(), Token::LoopStart(0));
}

#[test]
fn parse_loop_end() {
    assert_eq!(Token::from_char(']').unwrap(), Token::LoopEnd(0));
}
