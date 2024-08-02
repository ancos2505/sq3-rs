use std::{
    fmt::Display,
    io::Write,
    path::PathBuf,
    str::FromStr,
    time::{Instant, SystemTime},
};

use crate::{
    query::{SqliteQuery, SqliteQueryOutcome},
    SqliteResult,
};

#[derive(Debug)]
struct FuzzingResult {
    input: String,
    result: SqliteResult<SqliteQueryOutcome>,
    elapsed_micro: u128,
}

impl Display for FuzzingResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (is_success, result) = {
            match &self.result {
                Ok(outcome) => (true, format!("{outcome:?}")),
                Err(err) => (false, format!("{err:?}")),
            }
        };
        let output = format!(
            r#"{{ "input": "{input}", "is_success": {is_success}, "result": "{result}", "elapsed_micro": {elapsed_micro} }}"#,
            input = self.input,
            is_success = is_success,
            result = result.replace("\\\"", "\"").replace("\"", "\\\""),
            elapsed_micro = self.elapsed_micro
        );
        write!(f, "{output}")
    }
}

#[test]
#[ignore = "fuzzing"]
/// How to run:
/// ```sh
/// time cargo test fuzzing_on_valid_queries -- --test-threads=1 --ignored --nocapture >/dev/null 2>&1
/// find ./ -name "fuzzing-log*.json" | xargs jq '. | length'
/// ```
/// TODO: Migrate to xtask pattern
fn fuzzing_on_valid_queries() {
    use std::fs::File;
    let fuzzing_session_id: String = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_secs()
        .to_string();
    let mut path = PathBuf::from_str(env!("FUZZING_ARTIFACTS_DIR")).unwrap();
    path.push(format!("fuzzing-log-{fuzzing_session_id}.json"));

    let mut file = File::create_new(path).unwrap();
    let mut outcome = "[".to_string();
    for query in valid_queries() {
        for fuzzed_query in Permutation::permute_words(query) {
            let input = fuzzed_query.join(" ");
            let start = Instant::now();

            let result = SqliteQuery::run(&input);

            let elapsed_micro = start.elapsed().as_micros();
            let outcome_str = format!(
                "{},",
                FuzzingResult {
                    input,
                    result,
                    elapsed_micro,
                }
            );
            outcome.push_str(&outcome_str);
        }
    }
    if outcome.ends_with(',') {
        outcome.pop();
    }
    outcome.push(']');

    file.write_all(outcome.as_bytes()).unwrap();
}

fn valid_queries() -> Vec<&'static str> {
    vec![
        // "EXPLAIN QUERY",
        // "EXPLAIN QUERY PLAN",
        "SELECT 1",
        "SELECT 1,2",
        "SELECT 1,2,1,3",
        "SELECT DISTINCT 1,2,1,3",
        "SELECT (1)",
        "SELECT (5+2)",
        "SELECT (5-2)",
        "SELECT (5*2)",
        "SELECT (5/2)",
        "SELECT id,name FROM users WHERE age > 18",
        "SELECT ALL id,name FROM users WHERE age > 18",
        "SELECT DISTINCT id,name FROM users WHERE age > 18",
        "UPDATE users SET name = 'John' WHERE id = 1",
        "INSERT INTO users (name, age) VALUES ('Alice', 30)",
        "DELETE FROM users WHERE id = 5",
    ]
}

fn invalid_queries() -> Vec<&'static str> {
    vec![
        "TRUNCATE TABLE users",
        "EXPLAIN PLAN",
        "EXPLAIN",
        "QUERY PLAN",
        "QUERY",
        "PLAN",
    ]
}
struct Permutation;
impl Permutation {
    /// Heap's algorithm
    fn permute_words(phrase: &'static str) -> Vec<Vec<&str>> {
        let words: Vec<&str> = phrase.split_whitespace().collect();
        let mut result = Vec::new();
        let n = words.len();
        let mut c = vec![0; n];

        fn generate<T: Clone>(
            k: usize,
            words: &mut Vec<T>,
            result: &mut Vec<Vec<T>>,
            c: &mut Vec<usize>,
        ) {
            if k == 1 {
                result.push(words.clone());
            } else {
                generate(k - 1, words, result, c);

                for i in 0..k - 1 {
                    if k % 2 == 0 {
                        words.swap(i, k - 1);
                    } else {
                        words.swap(0, k - 1);
                    }
                    generate(k - 1, words, result, c);
                }
            }
        }

        generate(n, &mut words.clone(), &mut result, &mut c);
        result
    }
}
