use std::result;

use crate::{
    query::stmt::select::parser::result_column::{self, ResultColumns},
    SqliteResult,
};

#[test]
fn ok_on_test_result_column_parser() -> SqliteResult<()> {
    use super::result_column::{Initial, ResultColumnParser};
    let expected = vec!["id", "name"];
    let input = "id, name";
    dbg!(&input);

    let mut parser = ResultColumnParser::<Initial>::new(input)?;
    if parser.all_columns()?.is_some() {
        let result_column = parser.finish()?;
        dbg!(result_column);
        Ok(())
    } else if parser.table()?.is_some() {
        let result_column = parser.finish()?;
        dbg!(result_column);
        Ok(())
    } else if parser.expr()?.is_some() {
        let result_column = parser.finish()?;
        dbg!(result_column);
        Ok(())
    } else {
        // TODO: Insert Err(..)
        Ok(())
    }
}

#[test]
fn ok_on_test_result_column_parser_all_columns() -> SqliteResult<()> {
    let expected = ResultColumns::AllColumns;
    let input = "*";
    dbg!(&input);

    let result_columns = ResultColumns::parse(input)?;
    assert_eq!(result_columns, expected);

    Ok(())
}

#[test]
fn ok_on_test_select_parser() {
    use crate::query::stmt::select::SelectParser;
    let input = "SELECT * FROM users WHERE id = 1";
    // let input = "SELECT id, name FROM users WHERE id = 1";
    // let input = "SELECT DISTINCT id, name FROM users WHERE id = 1";
    // let input = "SELECT ALL id, name FROM users WHERE id = 1";
    dbg!(&input);
    let result = SelectParser::new(input)
        .select()
        .and_then(|p| p.distinct())
        .and_then(|p| p.result_columns())
        .and_then(|p| p.table())
        .and_then(|p| p.condition());

    match result {
        Ok(_) => println!("Parsing completed successfully"),
        Err(e) => println!("Parsing error: {}", e),
    }
}
