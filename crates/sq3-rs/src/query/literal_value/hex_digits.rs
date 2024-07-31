#[cfg(test)]
mod tests;

use sq3_derive::{Name, ParseChar};

use crate::{
    result::{SqlParserError, SqliteError},
    traits::TypeName,
};

use super::traits::HexDigit;

impl HexDigit for HexDigitA {}
impl HexDigit for HexDigitB {}
impl HexDigit for HexDigitC {}
impl HexDigit for HexDigitD {}
impl HexDigit for HexDigitE {}
impl HexDigit for HexDigitF {}

#[derive(Debug, Name, ParseChar)]
#[char = "a"]
struct HexDigitA;

#[derive(Debug, Name, ParseChar)]
#[char = "b"]
struct HexDigitB;

#[derive(Debug, Name, ParseChar)]
#[char = "c"]
struct HexDigitC;

#[derive(Debug, Name, ParseChar)]
#[char = "d"]
struct HexDigitD;

#[derive(Debug, Name, ParseChar)]
#[char = "e"]
struct HexDigitE;

#[derive(Debug, Name, ParseChar)]
#[char = "f"]
struct HexDigitF;
