use std::fmt::Debug;
use std::fmt::Display;

// pub(super) trait SqliteExpression {}
pub(super) trait SqliteKeyword: Display + Debug {}

/// ## DistictProcessing
///
/// **Removal of duplicate rows (DISTINCT processing).**
///
///  One of the ALL or DISTINCT keywords may follow the SELECT keyword in a
/// simple SELECT statement. If the simple SELECT is a SELECT ALL, then the
/// entire set of result rows are returned by the SELECT. If neither ALL or
/// DISTINCT are present, then the behavior is as if ALL were specified. If the
/// simple SELECT is a SELECT DISTINCT, then duplicate rows are removed from the
/// set of result rows before it is returned. For the purposes of detecting
/// duplicate rows, two NULL values are considered to be equal. The usual rules
/// apply for selecting a collation sequence to compare text values.
///
/// **Reference:** https://www.sqlite.org/lang_select.html#distinct
pub(super) trait DistinctProcessing: SqliteKeyword {}

/// ## DeterminationOfInputData
///
/// Determination of input data (FROM clause processing)
///
///  The input data used by a simple SELECT query is a set of N rows each M
/// columns wide.
///
///  If the FROM clause is omitted from a simple SELECT statement, then the
/// input data is implicitly a single row zero columns wide (i.e. N=1 and M=0).
///
///  If a FROM clause is specified, the data on which a simple SELECT query
/// operates comes from the one or more tables or subqueries (SELECT statements
/// in parentheses) specified following the FROM keyword. A subquery specified
/// in the table-or-subquery following the FROM clause in a simple SELECT
/// statement is handled as if it was a table containing the data returned by
/// executing the subquery statement. Each column of the subquery has the
/// collation sequence and affinity of the corresponding expression in the
/// subquery statement.
///
///  If there is only a single table or subquery in the FROM clause, then the
/// input data used by the SELECT statement is the contents of the named table.
/// If there is more than one table or subquery in FROM clause then the contents
/// of all tables and/or subqueries are joined into a single dataset for the
/// simple SELECT statement to operate on. Exactly how the data is combined
/// depends on the specific join-operator and join-constraint used to connect
/// the tables or subqueries together.
///
/// **Reference:** https://www.sqlite.org/lang_select.html#fromclause
pub(super) trait DeterminationOfInputData: SqliteKeyword {}
