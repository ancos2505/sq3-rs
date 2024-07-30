use proc_macro::TokenStream;

#[proc_macro_derive(Name)]
pub fn name_derive(input: TokenStream) -> TokenStream {
    // The implementation from the previous message goes here
    let input = input.to_string();

    let struct_name = input
        .lines()
        .find(|line| line.trim().starts_with("struct") || line.trim().starts_with("enum"))
        .and_then(|line| line.split_whitespace().nth(1))
        .expect("Could not find struct or enum name");

    let output = format!(
        "impl Name for {0} {{
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
