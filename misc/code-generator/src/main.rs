use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> AppResult<()> {
    // panic!("Sorry. You should check it before run.");

    let file_path = "keywords.txt";
    // generate_keyword_from_str(file_path)?;
    generate_modules(file_path)?;

    Ok(())
}

fn generate_keyword_from_str<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let file = File::open(path)?;
    let io = BufReader::new(file);
    let mut output = r#"
impl FromStr for Keyword {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
"#
    .to_string();

    let footer = r#"
        }
    }
}"#;

    for line in io.lines().flatten() {
        if let Some(keyword) = line.trim().split_whitespace().next() {
            let filename = keyword.to_lowercase();

            let struct_name = capitalize(&filename);
            let generate_code =
            
                format!("            \"{keyword}\" => Ok(Self(Box::new(s.parse::<{struct_name}>()?) as Box<dyn SqliteKeyword>)),\n");
            output.push_str(&generate_code);
        }
        
    }
    output.push_str(&footer);
        println!("{output}");
    Ok(())
}

fn generate_modules<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let file = File::open(path)?;
    let io = BufReader::new(file);
    for line in io.lines().flatten() {
        if let Some(keyword) = line.trim().split_whitespace().next() {
            let filename = keyword.to_lowercase();

            let mut output_file = File::create(format!("{filename}.rs"))?;
            let struct_name = capitalize(&filename);
            let output = format!(
                r#"

use std::{{fmt::Display, str::FromStr}};

use crate::{{
    query::traits::SqliteKeyword,
    result::{{SqlParserError, SqliteError}},
}};

#[derive(Debug)]
pub(crate) struct {struct_name};

impl FromStr for {struct_name} {{
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {{
        match s {{
            "{keyword}" => Ok(Self),
            _ => Err(SqliteError::SqlParser(SqlParserError("Keyword {keyword} not found.".into()))),
        }}
    }}
}}

impl Display for {struct_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{keyword}")
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
