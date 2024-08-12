use crate::SqliteQuery;

fn valid_queries() -> Vec<&'static str> {
    vec![
        "sElEcT 1;",
        "SeLeCt 1;",
        "SElECT 1,2;",
        "SELECT ALL 1,2,1,3;",
        "SeLECT DISTINCT 1,2,1,3;",
        "SELeCT (1);",
        "SElEcT (5+2);",
        "SELECT (5-2);",
        "sELECt (5*2);",
        "SELECT (5/2);",
        "SELECT id,name FROM users WHERE age > 18;",
        "SELECT ALL id,name FROM users WHERE age > 18;",
        "SELECT DISTINCT id,name FROM users WHERE age > 18;",
        "UPDATE users SET name = 'John' WHERE id = 1;",
        "INSERT INTO users (name, age) VALUES ('Alice', 30);",
        "DELETE FROM users WHERE id = 5;",
        // TODO: Not supported yet
        // "ExPlAiN QUERY;",
        // "EXPLAIN QUeRy PLaN;",
    ]
}

fn invalid_queries() -> Vec<&'static str> {
    vec![
        "TRUNCATE TABLE users;",
        "EXPLAIN PLAN;",
        "EXPLAIN;",
        "QUERY PLAN;",
        "QUERY;",
        "PLAN;",
    ]
}

#[test]
#[ignore = "Todo"]
fn ok_on_run_blind_valid_queries() {
    for query in valid_queries() {
        println!("Query: {}", query);

        let res = SqliteQuery::run(&query);
        println!("{res:?}");
        assert!(res.is_ok());
    }
}

#[test]
#[ignore = "Todo"]
fn err_on_run_blind_invalid_queries() {
    for query in invalid_queries() {
        println!("Query: {}", query);
        let res = SqliteQuery::run(&query);
        println!("{res:?}");
        assert!(res.is_err());
    }
}
