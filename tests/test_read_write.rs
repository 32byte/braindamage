use braindamage::*;

#[test]
fn test_read() {
    let tokens = parse(",").unwrap();

    let input = "A";

    let buf = interpret(&tokens, &mut input.as_bytes(), &mut Vec::new()).unwrap();

    assert_eq!(buf.0[0], input.as_bytes()[0]);
}

#[test]
fn test_write() {
    let tokens = parse("++++++++++.").unwrap();
    let mut output: Vec<u8> = Vec::new();
    let _ = interpret(&tokens, &mut "".as_bytes(), &mut output).unwrap();

    assert_eq!(output[0], NEW_LINE);
}
