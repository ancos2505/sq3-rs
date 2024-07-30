use proc_macro::TokenStream;

#[proc_macro_derive(Name)]
pub fn name_derive(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let struct_name = input
        .lines()
        .find(|line| line.trim().contains("struct") || line.trim().contains("enum"))
        .and_then(|line| {
            dbg!(&line);
            line.split_whitespace().nth(2).map(|s| {
                if s.contains(';') {
                    s.split(';').next()
                } else if s.contains('{') {
                    s.split('{').next()
                } else if s.contains('(') {
                    s.split(')').next()
                } else {
                    Some(s)
                }
            })
        })
        .flatten()
        .expect(format!("Error on macro parsing input: {input}").as_str());

    let output = format!(
        "impl TypeName for {0} {{
            const NAME: &'static str = \"{0}\";
        }}",
        struct_name
    );

    output.parse().unwrap()
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
