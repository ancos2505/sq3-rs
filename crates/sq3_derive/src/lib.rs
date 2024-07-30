use proc_macro::TokenStream;

#[proc_macro_derive(Name)]
pub fn name_derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    dbg!(&input);
    let struct_name = input
        .lines()
        .find(|line| line.trim().contains("struct") || line.trim().contains("enum"))
        .and_then(|line| {
            line.split_whitespace()
                .nth(2)
                .map(|s| {
                    let mut modified: Option<&str> = Some(s);
                    for c in [';', '{', '('].iter() {
                        modified = modified
                            .map(|s| {
                                dbg!(s);
                                s.split(*c).next()
                            })
                            .flatten()
                    }
                    modified
                })
                .flatten()
        })
        .expect(format!("Error on macro parsing input: {input}").as_str());

    let output = format!(
        "impl TypeName for {0} {{
            const NAME: &'static str = \"{0}\";
        }}",
        struct_name
    );

    output.parse().unwrap()
}
