#[cfg(test)]
mod tests;

use sq3_derive::{Name, ParseChar};

use crate::{
    result::{SqlParserError, SqliteError},
    traits::TypeName,
};

#[derive(Debug, Name, ParseChar)]
#[char = "~"]
pub struct Tilde;

#[derive(Debug, Name, ParseChar)]
#[char = "+"]
pub struct Plus;

#[derive(Debug, Name, ParseChar)]
#[char = "-"]
pub struct Minus;

#[derive(Debug, Name, ParseChar)]
#[char = "*"]
pub struct Multiplication;

#[derive(Debug, Name, ParseChar)]
#[char = "%"]
pub struct Percent;

#[derive(Debug, Name, ParseChar)]
#[char = "("]
pub struct LeftParenthesis;

#[derive(Debug, Name, ParseChar)]
#[char = ")"]
pub struct RightParenthesis;
