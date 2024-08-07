use crate::query::{
    expression::SqliteExpression,
    keyword::{Distinct, KeywordFrom},
    stmt::{
        select::{ColumnName, ResultColumns, TableName},
        SelectStmt,
    },
};

#[test]
fn ok_on_test_select_parser() {
    use crate::query::stmt::select::{Initial, SelectParser};
    let input = String::from("SELECT id, name FROM users WHERE id = 1");
    dbg!(&input);
    let result = SelectParser::<Initial>::new(input)
        .parse_select()
        .and_then(|p| p.parse_columns())
        .and_then(|p| p.parse_table())
        .and_then(|p| p.parse_condition());

    match result {
        Ok(_) => println!("Parsing completed successfully"),
        Err(e) => println!("Parsing error: {}", e),
    }
}

#[test]
fn ok_on_run_valid_select_query() {
    let expected = SelectStmt {
        distinct: Some(Box::new(Distinct)),
        result_columns: Some(ResultColumns::Filter(vec![
            ColumnName("id"),
            ColumnName("name"),
        ])),
        from: Some(KeywordFrom),
        origin: Some(TableName("users")),
        expr: None,
        ..Default::default()
    };
    let query = "SELECT DISTINCT id,name FROM users WHERE age > 18;";

    println!("Query: {}", query);
    if let Some((_, next_to_parse)) = query.split_once(' ') {
        let select = SelectStmt::parse(next_to_parse).unwrap();
        {
            let maybe_expected = &expected.distinct;
            let maybe_retrieved = &select.distinct;
            match (maybe_expected, maybe_retrieved) {
                (Some(dyn_expected), Some(dyn_retrieved)) => {
                    let expected = dyn_expected.as_any().downcast_ref::<Distinct>().unwrap();
                    let retrieved = dyn_retrieved.as_any().downcast_ref::<Distinct>().unwrap();
                    // dbg!(expected, retrieved);
                    assert_eq!(expected, retrieved);
                }
                _ => panic!(),
            }
        }
        {
            let maybe_expected = &expected.result_columns;
            let maybe_retrieved = &select.result_columns;
            match (maybe_expected, maybe_retrieved) {
                (Some(expected), Some(retrieved)) => {
                    // dbg!(expected, retrieved);
                    assert_eq!(expected, retrieved);
                }
                _ => panic!(),
            }
        }
        {
            let maybe_expected = &expected.from;
            let maybe_retrieved = &select.from;
            match (maybe_expected, maybe_retrieved) {
                (Some(expected), Some(retrieved)) => {
                    // dbg!(expected, retrieved);
                    assert_eq!(expected, retrieved);
                }
                _ => panic!(),
            }
        }
        {
            let maybe_expected = &expected.origin;
            let maybe_retrieved = &select.origin;
            match (maybe_expected, maybe_retrieved) {
                (Some(expected), Some(retrieved)) => {
                    // dbg!(expected, retrieved);
                    assert_eq!(expected, retrieved);
                }
                _ => panic!(),
            }
        }
        // {
        //     let maybe_expected = &expected.expr;
        //     let maybe_retrieved = &select.expr;
        //     dbg!(maybe_expected, maybe_retrieved);
        //     match (maybe_expected, maybe_retrieved) {
        //         (Some(expected), Some(retrieved)) => {
        //             // dbg!(expected, retrieved);
        //             assert_eq!(expected, retrieved);
        //         }
        //         _ => panic!(),
        //     }
        // }
    } else {
        panic!()
    }
}

// #[test]
// #[ignore = "Todo"]
// fn err_on_run_invalid_select_queries() {
//     for query in invalid_queries() {
//         println!("Query: {}", query);
//         let res = SqliteQuery::run(&query);
//         println!("{res:?}");
//         assert!(res.is_err());
//     }
// }
