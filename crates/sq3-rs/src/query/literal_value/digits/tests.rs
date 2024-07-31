use super::{Digit0, Digit1, Digit2, Digit3, Digit4, Digit5, Digit6, Digit7, Digit8, Digit9};

// Tests for Digit0
#[test]
fn ok_on_parse_digit0() {
    let res = "0".parse::<Digit0>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit0() {
    let res = "".parse::<Digit0>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit1
#[test]
fn ok_on_parse_digit1() {
    let res = "1".parse::<Digit1>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit1() {
    let res = "".parse::<Digit1>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit2
#[test]
fn ok_on_parse_digit2() {
    let res = "2".parse::<Digit2>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit2() {
    let res = "".parse::<Digit2>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit3
#[test]
fn ok_on_parse_digit3() {
    let res = "3".parse::<Digit3>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit3() {
    let res = "".parse::<Digit3>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit4
#[test]
fn ok_on_parse_digit4() {
    let res = "4".parse::<Digit4>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit4() {
    let res = "".parse::<Digit4>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit5
#[test]
fn ok_on_parse_digit5() {
    let res = "5".parse::<Digit5>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit5() {
    let res = "".parse::<Digit5>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit6
#[test]
fn ok_on_parse_digit6() {
    let res = "6".parse::<Digit6>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit6() {
    let res = "".parse::<Digit6>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit7
#[test]
fn ok_on_parse_digit7() {
    let res = "7".parse::<Digit7>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit7() {
    let res = "".parse::<Digit7>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit8
#[test]
fn ok_on_parse_digit8() {
    let res = "8".parse::<Digit8>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit8() {
    let res = "".parse::<Digit8>();
    dbg!(&res);
    assert!(res.is_err());
}

// Tests for Digit9
#[test]
fn ok_on_parse_digit9() {
    let res = "9".parse::<Digit9>();
    dbg!(&res);
    assert!(res.is_ok());
}

#[test]
fn err_on_parse_digit9() {
    let res = "".parse::<Digit9>();
    dbg!(&res);
    assert!(res.is_err());
}
