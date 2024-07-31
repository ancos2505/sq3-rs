#[cfg(test)]
mod tests;

use crate::{
    result::{SqlParserError, SqliteError},
    SqliteResult,
};

#[derive(Debug, PartialEq)]
pub(super) enum NumericLiteral {
    Integer(i64),
    Float(f32),
    Hexadecimal(i64),
}

pub(super) struct LiteralParser<'a> {
    input: &'a str,
    char_indices: std::str::CharIndices<'a>,
    current_index: usize,
    parsed: Option<&'a str>,
}

impl<'a> LiteralParser<'a> {
    pub fn new(input: &'a str) -> Self {
        LiteralParser {
            input,
            char_indices: input.char_indices(),
            current_index: 0,
            parsed: None,
        }
    }

    pub fn run(&mut self) -> SqliteResult<NumericLiteral> {
        if self.input.contains('X') || self.input.contains('x') {
            return self.parse_hex();
        } else if self.input.contains('.') {
            return self.parse_float();
        } else {
            let chars: Vec<char> = self
                .input
                .chars()
                .filter(|c| c.is_ascii_digit() || *c == '-' || *c == '+')
                .collect();
            if chars.len() > 0 {
                return self.parse_integer();
            } else {
                Err(SqliteError::SqlParser(SqlParserError(format!(
                    "Not a NumericLiteral"
                ))))
            }
        }
    }
    fn parse_hex(&mut self) -> SqliteResult<NumericLiteral> {
        let parsed_hex = parse_sqlite_hex_to_i64(self.input)?;
        Ok(NumericLiteral::Hexadecimal(parsed_hex))
    }
    fn parse_float(&mut self) -> SqliteResult<NumericLiteral> {
        let parsed_float = self.input.parse::<f32>()?;
        Ok(NumericLiteral::Float(parsed_float))
    }
    fn parse_integer(&mut self) -> SqliteResult<NumericLiteral> {
        let parsed_number = self.input.parse::<i64>()?;
        Ok(NumericLiteral::Integer(parsed_number))
    }
    fn advance(&mut self) -> Option<char> {
        if let Some((byte_index, ch)) = self.char_indices.next() {
            self.current_index = byte_index;
            self.parsed = Some(&self.input[..self.current_index]);
            Some(ch)
        } else {
            None
        }
    }

    fn current_slice(&self) -> &'a str {
        &self.input[self.current_index..]
    }
}

fn parse_sqlite_hex_to_i64(hex_str: &str) -> SqliteResult<i64> {
    // Check if the input starts with '0x' or '0X'
    if !hex_str.starts_with("0x") && !hex_str.starts_with("0X") {
        return Err(SqliteError::SqlParser(SqlParserError(format!(
            "Hexadecimal literal must start with '0x' or '0X'"
        ))));
    }

    // Remove the '0x' or '0X' prefix
    let hex_digits = &hex_str[2..];

    // Check if the number of digits is 16 or less
    if hex_digits.len() > 16 {
        return Err(SqliteError::SqlParser(SqlParserError(format!(
            "Hexadecimal literal cannot have more than 16 digits"
        ))));
    }

    // Parse the hexadecimal string
    match i64::from_str_radix(hex_digits, 16) {
        Ok(value) => Ok(value),
        Err(err) => {
            return Err(SqliteError::SqlParser(SqlParserError(format!(
                "Invalid hexadecimal literal. Reason: {err}"
            ))))
        }
    }
}
