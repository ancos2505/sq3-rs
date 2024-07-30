use std::str::FromStr;

use crate::result::{SqlParserError, SqliteError};

trait Digit {}

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

#[derive(Debug)]
struct Digit0;

impl FromStr for Digit0 {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(format!(
                "Error on parsing tilde"
            )))),
        }
    }
}

#[derive(Debug)]
struct Digit1;

#[derive(Debug)]
struct Digit2;

#[derive(Debug)]
struct Digit3;

#[derive(Debug)]
struct Digit4;

#[derive(Debug)]
struct Digit5;

#[derive(Debug)]
struct Digit6;

#[derive(Debug)]
struct Digit7;

#[derive(Debug)]
struct Digit8;

#[derive(Debug)]
struct Digit9;
