mod operators;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SqliteExpression(String);

impl From<String> for SqliteExpression {
    fn from(value: String) -> Self {
        Self(value)
    }
}
