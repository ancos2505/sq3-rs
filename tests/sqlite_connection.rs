use sq3_rs::SqliteConnection;

#[test]
fn ok_on_test_valid_connection() {
    let conn_str = "./data/small.sqlite3";
    let res = SqliteConnection::connect(conn_str);
    dbg!(&res);
    assert!(res.is_ok());
}
