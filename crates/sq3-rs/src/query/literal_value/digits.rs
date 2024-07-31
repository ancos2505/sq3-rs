#[cfg(test)]
mod tests;

use sq3_derive::{Name, ParseChar};

use crate::{
    result::{SqlParserError, SqliteError},
    traits::TypeName,
};

use super::traits::{Digit, HexDigit};

impl Digit for Digit0 {}
impl Digit for Digit1 {}
impl Digit for Digit2 {}
impl Digit for Digit3 {}
impl Digit for Digit4 {}
impl Digit for Digit5 {}
impl Digit for Digit6 {}
impl Digit for Digit7 {}
impl Digit for Digit8 {}
impl Digit for Digit9 {}

impl HexDigit for Digit0 {}
impl HexDigit for Digit1 {}
impl HexDigit for Digit2 {}
impl HexDigit for Digit3 {}
impl HexDigit for Digit4 {}
impl HexDigit for Digit5 {}
impl HexDigit for Digit6 {}
impl HexDigit for Digit7 {}
impl HexDigit for Digit8 {}
impl HexDigit for Digit9 {}

#[derive(Debug, Name, ParseChar)]
#[char = "0"]
struct Digit0;

#[derive(Debug, Name, ParseChar)]
#[char = "1"]
struct Digit1;

#[derive(Debug, Name, ParseChar)]
#[char = "2"]
struct Digit2;

#[derive(Debug, Name, ParseChar)]
#[char = "3"]
struct Digit3;

#[derive(Debug, Name, ParseChar)]
#[char = "4"]
struct Digit4;

#[derive(Debug, Name, ParseChar)]
#[char = "5"]
struct Digit5;

#[derive(Debug, Name, ParseChar)]
#[char = "6"]
struct Digit6;

#[derive(Debug, Name, ParseChar)]
#[char = "7"]
struct Digit7;

#[derive(Debug, Name, ParseChar)]
#[char = "8"]
struct Digit8;

#[derive(Debug, Name, ParseChar)]
#[char = "9"]
struct Digit9;
