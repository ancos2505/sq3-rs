#[derive(Debug, Default, PartialEq, Eq)]
pub struct SqliteRecord(String);

impl From<SqliteQueryOutcome> for SqliteRecord {
    fn from(value: SqliteQueryOutcome) -> Self {
        Self(format!("{value:?}"))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SqliteQueryOutcome {
    Success,
    Failure(SqliteDatabaseError),
}

impl Default for SqliteQueryOutcome {
    fn default() -> Self {
        Self::Failure(Default::default())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) enum SqliteDatabaseError {
    #[default]
    _Todo,
}
