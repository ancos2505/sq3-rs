# sq3-rs

SQLite3 library written in Idiomatic Rust and no dependencies (Under Development) 

## Getting Started

Run the task runner built on top o **xtask** pattern

```sh
alias cx="cargo xtask"
cx 
cx build
cx fuzzer
```

## SQL

### Syntax

- https://www.sqlite.org/syntaxdiagrams.html

#### Keywords

- https://www.sqlite.org/lang_keywords.html


### Initial approach in design decisions

#### Valid characters for column names and table names
- Case insensitive letters (a-z or A-Z)
- Numbers (0-9). After the first char.
- Underscore (_). After the first char.
