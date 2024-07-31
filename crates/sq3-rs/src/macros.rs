#[macro_export]
macro_rules! impl_from_str {
    ($struct_name:ident, $match_str:expr) => {
        impl std::str::FromStr for $struct_name {
            type Err = SqliteError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $match_str => Ok(Self),
                    _ => Err(SqliteError::SqlParser(SqlParserError(format!(
                        "Error on parsing {}",
                        Self::NAME,
                    )))),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! field_parsing_error {
    ($entity_name:expr) => {
        $crate::result::SqliteError::ParsingField($crate::result::FieldParsingError(format!(
            "Invalid payload for {}",
            Self::NAME
        )))
    };
}

// #[macro_export]
// macro_rules! impl_name {
//     ($struct_name:ty) => {
//         impl $crate::traits::TypeName for $struct_name {
//             const NAME: &'static str = stringify!($struct_name);
//         }
//     };
// }
