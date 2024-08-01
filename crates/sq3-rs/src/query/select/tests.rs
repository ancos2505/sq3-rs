use crate::query::{select::SelectStmt, traits::SqliteStatement};

#[test]
#[ignore = "Todo"]
fn ok_on_run_select_queries() {
    for query in queries() {
        println!("Query: {}", query);
        match SelectStmt::run(&query) {
            Ok(parsed_query) => println!("Parsed query: {:#?}", parsed_query),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn queries() -> Vec<String> {
    vec![
        "SELECT 1".into(),
        "SELECT 1,2".into(),
        "SELECT 1,2,1,3".into(),
        "SELECT DISTINCT 1,2,1,3".into(),
        // "SELECT (1)".into(),
        // "SELECT (5+2)".into(),
        // "SELECT (5-2)".into(),
        // "SELECT (5*2)".into(),
        // "SELECT (5/2)".into(),
        // "SELECT id,name FROM users WHERE age > 18".into(),
        // "SELECT ALL id,name FROM users WHERE age > 18".into(),
        // "SELECT DISTINCT id,name FROM users WHERE age > 18".into(),
        // "UPDATE users SET name = 'John' WHERE id = 1".into(),
        // "INSERT INTO users (name, age) VALUES ('Alice', 30)".into(),
        // "DELETE FROM users WHERE id = 5".into(),
        // "TRUNCATE TABLE users".into(),
    ]
}
