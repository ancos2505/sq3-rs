use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic!("Sorry. You should check it before run.");
    let file = File::open("keywords.txt")?;
    let io = BufReader::new(file);
    for line in io.lines().flatten() {
        if let Some(keyword) = line.trim().split_whitespace().next() {
            let filename = keyword.to_lowercase();

            let mut output_file = File::create(format!("{filename}.rs"))?;
            let struct_name = capitalize(&filename);
            let output = format!(
                r#"

use std::str::FromStr;

use crate::{{
    query::traits::SqliteKeyword,
    result::{{SqlParserError, SqliteError}},
}};

#[derive(Debug)]
pub(super) struct {struct_name};

impl FromStr for {struct_name} {{
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            "{keyword}" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError(Box::new(Self)))),
        }}
    }}
}}

impl SqliteKeyword for {struct_name} {{}}
"#
            );
            output_file.write_all(output.as_bytes())?;
            println!("File {filename}.rs created!");
        }
    }
    Ok(())
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
