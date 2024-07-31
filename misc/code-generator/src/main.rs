use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> AppResult<()> {
    // panic!("Sorry. You should check it before run.");

    let file_path = "keywords.txt";
    generate_keyword_from_str(file_path)?;
    // generate_modules(file_path)?;

    Ok(())
}

fn generate_keyword_from_str<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let file = File::open(path)?;
    let io = BufReader::new(file);
    let mut output = r#"
impl FromStr for Keyword {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
"#
    .to_string();

    let footer = r#"
    }
}"#;

    for line in io.lines().flatten() {
        if let Some(keyword) = line.trim().split_whitespace().next() {
            let filename = keyword.to_lowercase();

            let struct_name = capitalize(&filename);
            let generate_code = format!("            if {struct_name}::as_str() == input {{ return Ok(Self(Box::new({struct_name}) as Box<dyn Any>)); }},\n");
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
use std::fmt::Display;

use crate::query::traits::SqliteKeyword;

#[derive(Debug)]
pub(crate) struct {struct_name};
impl {struct_name} {{
    pub const fn as_str() -> &'static str {{
        "{keyword}"
    }}
}}


impl PartialEq<&str> for {struct_name} {{
    fn eq(&self, other: &&str) -> bool {{
        {struct_name}::as_str().eq_ignore_ascii_case(other)
    }}
}}

impl PartialEq<{struct_name}> for &str {{
    fn eq(&self, _: &{struct_name}) -> bool {{
        {struct_name}::as_str().eq_ignore_ascii_case(self)
    }}
}}

impl Display for {struct_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", Self::as_str())
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
