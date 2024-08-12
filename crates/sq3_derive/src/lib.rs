use proc_macro::TokenStream;

#[proc_macro_derive(Name)]
pub fn name_derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let type_name =
        extract_type_name(&input).expect(format!("Error on macro parsing input: {input}").as_str());

    let output = format!(
        "impl TypeName for {0} {{
            const NAME: &'static str = \"{0}\";
        }}",
        type_name
    );

    output.parse().unwrap()
}

#[proc_macro_derive(ParseChar, attributes(char))]
pub fn derive_parse_char(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let type_name =
        extract_type_name(&input).expect(format!("Error on macro parsing input: {input}").as_str());

    let char_value = input
        .lines()
        .find(|line| {
            let s = line.trim();
            s.contains("#") && s.contains("[char")
        })
        .and_then(|line| {
            line.split('"')
                .nth(1)
                .map(|s| s.trim().trim_matches(|c| c == '"' || c == ']'))
        })
        .expect(format!("Missing [char = \"...\"] attribute {input}",).as_str());

    let output = format!(
        r#"impl std::str::FromStr for {0} {{
            type Err = Sq3ParserError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {{
                match s {{
                    "{1}" => Ok(Self),
                    _ => Err(Sq3ParserError(format!(
                        "Error on parsing {{}}",
                        Self::NAME,
                    ))),
                }}
            }}
        }}"#,
        type_name, char_value
    );

    output.parse().unwrap()
}

fn extract_type_name(input: &str) -> Option<&str> {
    input
        .lines()
        .find(|line| line.trim().contains("struct") || line.trim().contains("enum"))
        .and_then(|line| {
            let type_name = if line.contains("struct") {
                line.split("struct").nth(1)
            } else {
                line.split("enum").nth(1)
            };
            type_name
                .unwrap()
                .split_whitespace()
                .next()
                .map(|s| {
                    let mut modified: Option<&str> = Some(s);

                    for c in [';', '{', '('].iter() {
                        modified = modified
                            .map(|char_found| char_found.split(*c).next())
                            .flatten()
                    }

                    modified
                })
                .flatten()
        })
}
