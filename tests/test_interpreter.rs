use std::io::{stdin, stdout};

use braindamage::*;

#[test]
fn test_interpreter() {
    let tokens = parse("++[->+<]").unwrap();
    let buf = interpret(&tokens, &mut stdin(), &mut stdout()).unwrap();

    assert_eq!(buf.0[0], 0);
    assert_eq!(buf.0[1], 2);
}

#[test]
fn test_out_of_lower_bounds() {
    let tokens = parse("<.").unwrap();
    let buf = interpret(&tokens, &mut stdin(), &mut stdout());

    assert_eq!(buf, Err(RuntimeError::PtrOutOfBounds(-1)));
}

#[test]
fn test_out_of_upper_bounds() {
    let tokens = parse("+[>+]").unwrap();
    let buf = interpret(&tokens, &mut stdin(), &mut stdout());

    assert_eq!(buf, Err(RuntimeError::PtrOutOfBounds(BUF_SIZE as i32)));
}
