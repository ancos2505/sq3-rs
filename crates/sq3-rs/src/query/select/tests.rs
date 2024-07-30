use crate::SqliteConnection;

// const TEST_QUERIES: [&str; 10] = [
const TEST_QUERIES: [&str; 5] = [
    "SELECT (1)",
    "SELECT (5+2)",
    "SELECT (5-2)",
    "SELECT (5*2)",
    "SELECT (5/2)",
    // "SELECT id,name FROM users WHERE age > 18",
    // "UPDATE users SET name = 'John' WHERE id = 1",
    // "INSERT INTO users (name, age) VALUES ('Alice', 30)",
    // "DELETE FROM users WHERE id = 5",
    // "TRUNCATE TABLE users",
];

#[test]
#[ignore = "Todo"]
fn run_queries() {
    let conn_str = "./data/small.sqlite3";
    let mut conn = SqliteConnection::connect(conn_str).unwrap();
    for query in TEST_QUERIES {
        println!("Query: {}", query);

        match conn.run_query(query) {
            Ok(parsed_query) => println!("Parsed query: {:#?}", parsed_query),
            Err(e) => println!("Error: {}", e),
        }
    }
}
