use super::{HexDigitA, HexDigitB, HexDigitC, HexDigitD, HexDigitE, HexDigitF};

// Tests for HexDigitA
#[test]
fn ok_on_parse_hex_digit_a() {
    let res = "a".parse::<HexDigitA>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_a() {
    let res = "".parse::<HexDigitA>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for HexDigitB
#[test]
fn ok_on_parse_hex_digit_b() {
    let res = "b".parse::<HexDigitB>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_b() {
    let res = "".parse::<HexDigitB>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for HexDigitC
#[test]
fn ok_on_parse_hex_digit_c() {
    let res = "c".parse::<HexDigitC>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_c() {
    let res = "".parse::<HexDigitC>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for HexDigitD
#[test]
fn ok_on_parse_hex_digit_d() {
    let res = "d".parse::<HexDigitD>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_d() {
    let res = "".parse::<HexDigitD>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for HexDigitE
#[test]
fn ok_on_parse_hex_digit_e() {
    let res = "e".parse::<HexDigitE>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_e() {
    let res = "".parse::<HexDigitE>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for HexDigitF
#[test]
fn ok_on_parse_hex_digit_f() {
    let res = "f".parse::<HexDigitF>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_hex_digit_f() {
    let res = "".parse::<HexDigitF>();
    dbg!(&res);
    assert!(res.is_err());
}
