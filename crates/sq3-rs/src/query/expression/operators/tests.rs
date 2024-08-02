use super::{LeftParenthesis, Minus, Multiplication, Percent, Plus, RightParenthesis, Tilde};

// Tests for Tilde
#[test]
fn ok_on_parse_tilde() {
    let res = "~".parse::<Tilde>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_tilde() {
    let res = "".parse::<Tilde>();

    assert!(res.is_err());
}

// Tests for Plus
#[test]
fn ok_on_parse_plus() {
    let res = "+".parse::<Plus>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_plus() {
    let res = "".parse::<Plus>();

    assert!(res.is_err());
}

// Tests for Minus
#[test]
fn ok_on_parse_minus() {
    let res = "-".parse::<Minus>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_minus() {
    let res = "".parse::<Minus>();

    assert!(res.is_err());
}

// Tests for Multiplication
#[test]
fn ok_on_parse_multiplication() {
    let res = "*".parse::<Multiplication>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_multiplication() {
    let res = "".parse::<Multiplication>();

    assert!(res.is_err());
}

// Tests for Percent
#[test]
fn ok_on_parse_percent() {
    let res = "%".parse::<Percent>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_percent() {
    let res = "".parse::<Percent>();

    assert!(res.is_err());
}

// Tests for LeftParenthesis
#[test]
fn ok_on_parse_left_parenthesis() {
    let res = "(".parse::<LeftParenthesis>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_left_parenthesis() {
    let res = "".parse::<LeftParenthesis>();

    assert!(res.is_err());
}

// Tests for RightParenthesis
#[test]
fn ok_on_parse_right_parenthesis() {
    let res = ")".parse::<RightParenthesis>();

    assert!(res.is_ok());
}

#[test]
fn err_on_parse_right_parenthesis() {
    let res = "".parse::<RightParenthesis>();

    assert!(res.is_err());
}
