use crate::{stmt::select::parser::result_column::ResultColumns, ParserResult};

#[test]
#[ignore = "Todo"]
fn ok_on_test_result_column_parser() -> ParserResult<()> {
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
fn ok_on_test_result_column_parser_all_columns() -> ParserResult<()> {
    let expected = ResultColumns::AllColumns;
    let input = "*";
    dbg!(&input);

    let result_columns = ResultColumns::parse(input)?;
    assert_eq!(result_columns, expected);

    Ok(())
}

#[test]
fn ok_on_test_select_parser() -> ParserResult<()> {
    use super::SelectParser;
    let input = "SELECT * FROM users;";
    // let input = "SELECT * FROM users WHERE id = 1";
    // let input = "SELECT id, name FROM users WHERE id = 1";
    // let input = "SELECT DISTINCT id, name FROM users WHERE id = 1";
    // let input = "SELECT ALL id, name FROM users WHERE id = 1";
    dbg!(&input);
    let parser = SelectParser::new(input)
        .select()
        .and_then(|p| p.distinct())
        .and_then(|p| p.result_columns())
        .and_then(|p| p.from())
        .and_then(|p| p.table())
        .and_then(|p| p.r#where())
        .and_then(|p| p.condition());
    assert!(parser.is_ok());
    match parser?.finish() {
        Ok(stmt) => {
            dbg!(stmt);
            println!("Parsing completed successfully")
        }
        Err(e) => println!("Parsing error: {}", e),
    }

    Ok(())
}
