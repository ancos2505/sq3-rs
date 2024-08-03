mod operators;

#[derive(Debug)]
pub(super) struct SqliteExpression(String);

impl From<String> for SqliteExpression {
    fn from(value: String) -> Self {
        Self(value)
    }
}
