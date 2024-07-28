use std::{borrow::Cow, str::FromStr};

use crate::result::SqliteError;

#[derive(Debug)]
pub(super) struct SelectQuery<'a> {
    query: String,
    columns: Vec<&'a str>,
    table: Box<&'a str>,
    conditions: Option<Vec<&'a str>>,
}

impl FromStr for SelectQuery<'_> {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let query = s.to_string();
        let parts: Vec<&str> = query.split_whitespace().collect();
        let mut index = 1; // Start after "SELECT"
        let mut columns = Vec::new();
        let mut table = "";
        let mut conditions = None;

        // Parse columns
        while index < parts.len() && parts[index].to_lowercase() != "from" {
            columns.push(parts[index]);
            index += 1;
        }

        if index >= parts.len() || parts[index].to_lowercase() != "from" {
            return Err(SqliteError::Custom("FROM clause is missing".to_string()));
        }
        index += 1;

        // Parse table name
        if index < parts.len() {
            table = parts[index];
            index += 1;
        } else {
            return Err(SqliteError::Custom("Table name is missing".to_string()));
        }

        // Parse WHERE clause if it exists
        if index < parts.len() && parts[index].to_lowercase() == "where" {
            index += 1;
            let mut where_conditions = Vec::new();
            while index < parts.len() {
                where_conditions.push(parts[index]);
                index += 1;
            }
            conditions = Some(where_conditions);
        }

        Ok(SelectQuery {
            query,
            columns,
            table: Box::new(table),
            conditions,
        })
    }
}
