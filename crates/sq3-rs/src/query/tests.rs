use crate::query::SqliteQuery;

fn valid_queries() -> Vec<String> {
    vec![
        "SELECT 1".into(),
        "SELECT 1,2".into(),
        "SELECT 1,2,1,3".into(),
        "SELECT DISTINCT 1,2,1,3".into(),
        "SELECT (1)".into(),
        "SELECT (5+2)".into(),
        "SELECT (5-2)".into(),
        "SELECT (5*2)".into(),
        "SELECT (5/2)".into(),
        "SELECT id,name FROM users WHERE age > 18".into(),
        "SELECT ALL id,name FROM users WHERE age > 18".into(),
        "SELECT DISTINCT id,name FROM users WHERE age > 18".into(),
        "UPDATE users SET name = 'John' WHERE id = 1".into(),
        "INSERT INTO users (name, age) VALUES ('Alice', 30)".into(),
        "DELETE FROM users WHERE id = 5".into(),
    ]
}

fn invalid_queries() -> Vec<String> {
    vec!["TRUNCATE TABLE users".into()]
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
