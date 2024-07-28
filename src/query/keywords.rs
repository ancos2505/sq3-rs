mod abort;
mod action;
mod add;
mod after;
mod all;
mod alter;
mod always;
mod analyze;
mod and;
mod as_;
mod asc;
mod attach;
mod autoincrement;
mod before;
mod begin;
mod between;
mod by;
mod cascade;
mod case;
mod cast;
mod check;
mod collate;
mod column;
mod commit;
mod conflict;
mod constraint;
mod create;
mod cross;
mod current;
mod current_date;
mod current_time;
mod current_timestamp;
mod database;
mod default;
mod deferrable;
mod deferred;
mod delete;
mod desc;
mod detach;
mod distinct;
mod do_;
mod drop;
mod each;
mod else_;
mod end;
mod escape;
mod except;
mod exclude;
mod exclusive;
mod exists;
mod explain;
mod fail;
mod filter;
mod first;
mod following;
mod for_;
mod foreign;
mod from;
mod full;
mod generated;
mod glob;
mod group;
mod groups;
mod having;
mod if_;
mod ignore;
mod immediate;
mod in_;
mod index;
mod indexed;
mod initially;
mod inner;
mod insert;
mod instead;
mod intersect;
mod into;
mod is;
mod isnull;
mod join;
mod key;
mod last;
mod left;
mod like;
mod limit;
mod match_;
mod materialized;
mod natural;
mod no;
mod not;
mod nothing;
mod notnull;
mod null;
mod nulls;
mod of;
mod offset;
mod on;
mod or;
mod order;
mod others;
mod outer;
mod over;
mod partition;
mod plan;
mod pragma;
mod preceding;
mod primary;
mod query;
mod raise;
mod range;
mod recursive;
mod references;
mod regexp;
mod reindex;
mod release;
mod rename;
mod replace;
mod restrict;
mod returning;
mod right;
mod rollback;
mod row;
mod rows;
mod savepoint;
mod select;
mod set;
mod table;
mod temp;
mod temporary;
mod then;
mod ties;
mod to;
mod transaction;
mod trigger;
mod unbounded;
mod union;
mod unique;
mod update;
mod using;
mod vacuum;
mod values;
mod view;
mod virtual_;
mod when;
mod where_;
mod window;
mod with;
mod without;

use crate::result::{SqlParserError, SqliteError};

pub(crate) use self::{
    abort::Abort, action::Action, add::Add, after::After, all::All, alter::Alter, always::Always,
    analyze::Analyze, and::And, as_::As, asc::Asc, attach::Attach, autoincrement::Autoincrement,
    before::Before, begin::Begin, between::Between, by::By, cascade::Cascade, case::Case,
    cast::Cast, check::Check, collate::Collate, column::Column, commit::Commit, conflict::Conflict,
    constraint::Constraint, create::Create, cross::Cross, current::Current,
    current_date::Current_date, current_time::Current_time, current_timestamp::Current_timestamp,
    database::Database, default::Default, deferrable::Deferrable, deferred::Deferred,
    delete::Delete, desc::Desc, detach::Detach, distinct::Distinct, do_::Do, drop::Drop,
    each::Each, else_::Else, end::End, escape::Escape, except::Except, exclude::Exclude,
    exclusive::Exclusive, exists::Exists, explain::Explain, fail::Fail, filter::Filter,
    first::First, following::Following, for_::For, foreign::Foreign, from::From, full::Full,
    generated::Generated, glob::Glob, group::Group, groups::Groups, having::Having, if_::If,
    ignore::Ignore, immediate::Immediate, in_::In, index::Index, indexed::Indexed,
    initially::Initially, inner::Inner, insert::Insert, instead::Instead, intersect::Intersect,
    into::Into, is::Is, isnull::Isnull, join::Join, key::Key, last::Last, left::Left, like::Like,
    limit::Limit, match_::Match, materialized::Materialized, natural::Natural, no::No, not::Not,
    nothing::Nothing, notnull::Notnull, null::Null, nulls::Nulls, of::Of, offset::Offset, on::On,
    or::Or, order::Order, others::Others, outer::Outer, over::Over, partition::Partition,
    plan::Plan, pragma::Pragma, preceding::Preceding, primary::Primary, query::Query, raise::Raise,
    range::Range, recursive::Recursive, references::References, regexp::Regexp, reindex::Reindex,
    release::Release, rename::Rename, replace::Replace, restrict::Restrict, returning::Returning,
    right::Right, rollback::Rollback, row::Row, rows::Rows, savepoint::Savepoint, select::Select,
    set::Set, table::Table, temp::Temp, temporary::Temporary, then::Then, ties::Ties, to::To,
    transaction::Transaction, trigger::Trigger, unbounded::Unbounded, union::Union, unique::Unique,
    update::Update, using::Using, vacuum::Vacuum, values::Values, view::View, virtual_::Virtual,
    when::When, where_::Where, window::Window, with::With, without::Without,
};

// use super::traits::SqliteKeyword;
use std::any::Any;
use std::fmt::Debug;

use std::str::FromStr;

#[derive(Debug)]
pub struct Keyword(pub Box<dyn Any>);

impl FromStr for Keyword {
    type Err = SqliteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ABORT" => Ok(Self(Box::new(s.parse::<Abort>()?) as Box<dyn Any>)),
            "ACTION" => Ok(Self(Box::new(s.parse::<Action>()?) as Box<dyn Any>)),
            "ADD" => Ok(Self(Box::new(s.parse::<Add>()?) as Box<dyn Any>)),
            "AFTER" => Ok(Self(Box::new(s.parse::<After>()?) as Box<dyn Any>)),
            "ALL" => Ok(Self(Box::new(s.parse::<All>()?) as Box<dyn Any>)),
            "ALTER" => Ok(Self(Box::new(s.parse::<Alter>()?) as Box<dyn Any>)),
            "ALWAYS" => Ok(Self(Box::new(s.parse::<Always>()?) as Box<dyn Any>)),
            "ANALYZE" => Ok(Self(Box::new(s.parse::<Analyze>()?) as Box<dyn Any>)),
            "AND" => Ok(Self(Box::new(s.parse::<And>()?) as Box<dyn Any>)),
            "AS" => Ok(Self(Box::new(s.parse::<As>()?) as Box<dyn Any>)),
            "ASC" => Ok(Self(Box::new(s.parse::<Asc>()?) as Box<dyn Any>)),
            "ATTACH" => Ok(Self(Box::new(s.parse::<Attach>()?) as Box<dyn Any>)),
            "AUTOINCREMENT" => Ok(Self(Box::new(s.parse::<Autoincrement>()?) as Box<dyn Any>)),
            "BEFORE" => Ok(Self(Box::new(s.parse::<Before>()?) as Box<dyn Any>)),
            "BEGIN" => Ok(Self(Box::new(s.parse::<Begin>()?) as Box<dyn Any>)),
            "BETWEEN" => Ok(Self(Box::new(s.parse::<Between>()?) as Box<dyn Any>)),
            "BY" => Ok(Self(Box::new(s.parse::<By>()?) as Box<dyn Any>)),
            "CASCADE" => Ok(Self(Box::new(s.parse::<Cascade>()?) as Box<dyn Any>)),
            "CASE" => Ok(Self(Box::new(s.parse::<Case>()?) as Box<dyn Any>)),
            "CAST" => Ok(Self(Box::new(s.parse::<Cast>()?) as Box<dyn Any>)),
            "CHECK" => Ok(Self(Box::new(s.parse::<Check>()?) as Box<dyn Any>)),
            "COLLATE" => Ok(Self(Box::new(s.parse::<Collate>()?) as Box<dyn Any>)),
            "COLUMN" => Ok(Self(Box::new(s.parse::<Column>()?) as Box<dyn Any>)),
            "COMMIT" => Ok(Self(Box::new(s.parse::<Commit>()?) as Box<dyn Any>)),
            "CONFLICT" => Ok(Self(Box::new(s.parse::<Conflict>()?) as Box<dyn Any>)),
            "CONSTRAINT" => Ok(Self(Box::new(s.parse::<Constraint>()?) as Box<dyn Any>)),
            "CREATE" => Ok(Self(Box::new(s.parse::<Create>()?) as Box<dyn Any>)),
            "CROSS" => Ok(Self(Box::new(s.parse::<Cross>()?) as Box<dyn Any>)),
            "CURRENT" => Ok(Self(Box::new(s.parse::<Current>()?) as Box<dyn Any>)),
            "CURRENT_DATE" => Ok(Self(Box::new(s.parse::<Current_date>()?) as Box<dyn Any>)),
            "CURRENT_TIME" => Ok(Self(Box::new(s.parse::<Current_time>()?) as Box<dyn Any>)),
            "CURRENT_TIMESTAMP" => Ok(Self(
                Box::new(s.parse::<Current_timestamp>()?) as Box<dyn Any>
            )),
            "DATABASE" => Ok(Self(Box::new(s.parse::<Database>()?) as Box<dyn Any>)),
            "DEFAULT" => Ok(Self(Box::new(s.parse::<Default>()?) as Box<dyn Any>)),
            "DEFERRABLE" => Ok(Self(Box::new(s.parse::<Deferrable>()?) as Box<dyn Any>)),
            "DEFERRED" => Ok(Self(Box::new(s.parse::<Deferred>()?) as Box<dyn Any>)),
            "DELETE" => Ok(Self(Box::new(s.parse::<Delete>()?) as Box<dyn Any>)),
            "DESC" => Ok(Self(Box::new(s.parse::<Desc>()?) as Box<dyn Any>)),
            "DETACH" => Ok(Self(Box::new(s.parse::<Detach>()?) as Box<dyn Any>)),
            "DISTINCT" => Ok(Self(Box::new(s.parse::<Distinct>()?) as Box<dyn Any>)),
            "DO" => Ok(Self(Box::new(s.parse::<Do>()?) as Box<dyn Any>)),
            "DROP" => Ok(Self(Box::new(s.parse::<Drop>()?) as Box<dyn Any>)),
            "EACH" => Ok(Self(Box::new(s.parse::<Each>()?) as Box<dyn Any>)),
            "ELSE" => Ok(Self(Box::new(s.parse::<Else>()?) as Box<dyn Any>)),
            "END" => Ok(Self(Box::new(s.parse::<End>()?) as Box<dyn Any>)),
            "ESCAPE" => Ok(Self(Box::new(s.parse::<Escape>()?) as Box<dyn Any>)),
            "EXCEPT" => Ok(Self(Box::new(s.parse::<Except>()?) as Box<dyn Any>)),
            "EXCLUDE" => Ok(Self(Box::new(s.parse::<Exclude>()?) as Box<dyn Any>)),
            "EXCLUSIVE" => Ok(Self(Box::new(s.parse::<Exclusive>()?) as Box<dyn Any>)),
            "EXISTS" => Ok(Self(Box::new(s.parse::<Exists>()?) as Box<dyn Any>)),
            "EXPLAIN" => Ok(Self(Box::new(s.parse::<Explain>()?) as Box<dyn Any>)),
            "FAIL" => Ok(Self(Box::new(s.parse::<Fail>()?) as Box<dyn Any>)),
            "FILTER" => Ok(Self(Box::new(s.parse::<Filter>()?) as Box<dyn Any>)),
            "FIRST" => Ok(Self(Box::new(s.parse::<First>()?) as Box<dyn Any>)),
            "FOLLOWING" => Ok(Self(Box::new(s.parse::<Following>()?) as Box<dyn Any>)),
            "FOR" => Ok(Self(Box::new(s.parse::<For>()?) as Box<dyn Any>)),
            "FOREIGN" => Ok(Self(Box::new(s.parse::<Foreign>()?) as Box<dyn Any>)),
            "FROM" => Ok(Self(Box::new(s.parse::<From>()?) as Box<dyn Any>)),
            "FULL" => Ok(Self(Box::new(s.parse::<Full>()?) as Box<dyn Any>)),
            "GENERATED" => Ok(Self(Box::new(s.parse::<Generated>()?) as Box<dyn Any>)),
            "GLOB" => Ok(Self(Box::new(s.parse::<Glob>()?) as Box<dyn Any>)),
            "GROUP" => Ok(Self(Box::new(s.parse::<Group>()?) as Box<dyn Any>)),
            "GROUPS" => Ok(Self(Box::new(s.parse::<Groups>()?) as Box<dyn Any>)),
            "HAVING" => Ok(Self(Box::new(s.parse::<Having>()?) as Box<dyn Any>)),
            "IF" => Ok(Self(Box::new(s.parse::<If>()?) as Box<dyn Any>)),
            "IGNORE" => Ok(Self(Box::new(s.parse::<Ignore>()?) as Box<dyn Any>)),
            "IMMEDIATE" => Ok(Self(Box::new(s.parse::<Immediate>()?) as Box<dyn Any>)),
            "IN" => Ok(Self(Box::new(s.parse::<In>()?) as Box<dyn Any>)),
            "INDEX" => Ok(Self(Box::new(s.parse::<Index>()?) as Box<dyn Any>)),
            "INDEXED" => Ok(Self(Box::new(s.parse::<Indexed>()?) as Box<dyn Any>)),
            "INITIALLY" => Ok(Self(Box::new(s.parse::<Initially>()?) as Box<dyn Any>)),
            "INNER" => Ok(Self(Box::new(s.parse::<Inner>()?) as Box<dyn Any>)),
            "INSERT" => Ok(Self(Box::new(s.parse::<Insert>()?) as Box<dyn Any>)),
            "INSTEAD" => Ok(Self(Box::new(s.parse::<Instead>()?) as Box<dyn Any>)),
            "INTERSECT" => Ok(Self(Box::new(s.parse::<Intersect>()?) as Box<dyn Any>)),
            "INTO" => Ok(Self(Box::new(s.parse::<Into>()?) as Box<dyn Any>)),
            "IS" => Ok(Self(Box::new(s.parse::<Is>()?) as Box<dyn Any>)),
            "ISNULL" => Ok(Self(Box::new(s.parse::<Isnull>()?) as Box<dyn Any>)),
            "JOIN" => Ok(Self(Box::new(s.parse::<Join>()?) as Box<dyn Any>)),
            "KEY" => Ok(Self(Box::new(s.parse::<Key>()?) as Box<dyn Any>)),
            "LAST" => Ok(Self(Box::new(s.parse::<Last>()?) as Box<dyn Any>)),
            "LEFT" => Ok(Self(Box::new(s.parse::<Left>()?) as Box<dyn Any>)),
            "LIKE" => Ok(Self(Box::new(s.parse::<Like>()?) as Box<dyn Any>)),
            "LIMIT" => Ok(Self(Box::new(s.parse::<Limit>()?) as Box<dyn Any>)),
            "MATCH" => Ok(Self(Box::new(s.parse::<Match>()?) as Box<dyn Any>)),
            "MATERIALIZED" => Ok(Self(Box::new(s.parse::<Materialized>()?) as Box<dyn Any>)),
            "NATURAL" => Ok(Self(Box::new(s.parse::<Natural>()?) as Box<dyn Any>)),
            "NO" => Ok(Self(Box::new(s.parse::<No>()?) as Box<dyn Any>)),
            "NOT" => Ok(Self(Box::new(s.parse::<Not>()?) as Box<dyn Any>)),
            "NOTHING" => Ok(Self(Box::new(s.parse::<Nothing>()?) as Box<dyn Any>)),
            "NOTNULL" => Ok(Self(Box::new(s.parse::<Notnull>()?) as Box<dyn Any>)),
            "NULL" => Ok(Self(Box::new(s.parse::<Null>()?) as Box<dyn Any>)),
            "NULLS" => Ok(Self(Box::new(s.parse::<Nulls>()?) as Box<dyn Any>)),
            "OF" => Ok(Self(Box::new(s.parse::<Of>()?) as Box<dyn Any>)),
            "OFFSET" => Ok(Self(Box::new(s.parse::<Offset>()?) as Box<dyn Any>)),
            "ON" => Ok(Self(Box::new(s.parse::<On>()?) as Box<dyn Any>)),
            "OR" => Ok(Self(Box::new(s.parse::<Or>()?) as Box<dyn Any>)),
            "ORDER" => Ok(Self(Box::new(s.parse::<Order>()?) as Box<dyn Any>)),
            "OTHERS" => Ok(Self(Box::new(s.parse::<Others>()?) as Box<dyn Any>)),
            "OUTER" => Ok(Self(Box::new(s.parse::<Outer>()?) as Box<dyn Any>)),
            "OVER" => Ok(Self(Box::new(s.parse::<Over>()?) as Box<dyn Any>)),
            "PARTITION" => Ok(Self(Box::new(s.parse::<Partition>()?) as Box<dyn Any>)),
            "PLAN" => Ok(Self(Box::new(s.parse::<Plan>()?) as Box<dyn Any>)),
            "PRAGMA" => Ok(Self(Box::new(s.parse::<Pragma>()?) as Box<dyn Any>)),
            "PRECEDING" => Ok(Self(Box::new(s.parse::<Preceding>()?) as Box<dyn Any>)),
            "PRIMARY" => Ok(Self(Box::new(s.parse::<Primary>()?) as Box<dyn Any>)),
            "QUERY" => Ok(Self(Box::new(s.parse::<Query>()?) as Box<dyn Any>)),
            "RAISE" => Ok(Self(Box::new(s.parse::<Raise>()?) as Box<dyn Any>)),
            "RANGE" => Ok(Self(Box::new(s.parse::<Range>()?) as Box<dyn Any>)),
            "RECURSIVE" => Ok(Self(Box::new(s.parse::<Recursive>()?) as Box<dyn Any>)),
            "REFERENCES" => Ok(Self(Box::new(s.parse::<References>()?) as Box<dyn Any>)),
            "REGEXP" => Ok(Self(Box::new(s.parse::<Regexp>()?) as Box<dyn Any>)),
            "REINDEX" => Ok(Self(Box::new(s.parse::<Reindex>()?) as Box<dyn Any>)),
            "RELEASE" => Ok(Self(Box::new(s.parse::<Release>()?) as Box<dyn Any>)),
            "RENAME" => Ok(Self(Box::new(s.parse::<Rename>()?) as Box<dyn Any>)),
            "REPLACE" => Ok(Self(Box::new(s.parse::<Replace>()?) as Box<dyn Any>)),
            "RESTRICT" => Ok(Self(Box::new(s.parse::<Restrict>()?) as Box<dyn Any>)),
            "RETURNING" => Ok(Self(Box::new(s.parse::<Returning>()?) as Box<dyn Any>)),
            "RIGHT" => Ok(Self(Box::new(s.parse::<Right>()?) as Box<dyn Any>)),
            "ROLLBACK" => Ok(Self(Box::new(s.parse::<Rollback>()?) as Box<dyn Any>)),
            "ROW" => Ok(Self(Box::new(s.parse::<Row>()?) as Box<dyn Any>)),
            "ROWS" => Ok(Self(Box::new(s.parse::<Rows>()?) as Box<dyn Any>)),
            "SAVEPOINT" => Ok(Self(Box::new(s.parse::<Savepoint>()?) as Box<dyn Any>)),
            "SELECT" => Ok(Self(Box::new(s.parse::<Select>()?) as Box<dyn Any>)),
            "SET" => Ok(Self(Box::new(s.parse::<Set>()?) as Box<dyn Any>)),
            "TABLE" => Ok(Self(Box::new(s.parse::<Table>()?) as Box<dyn Any>)),
            "TEMP" => Ok(Self(Box::new(s.parse::<Temp>()?) as Box<dyn Any>)),
            "TEMPORARY" => Ok(Self(Box::new(s.parse::<Temporary>()?) as Box<dyn Any>)),
            "THEN" => Ok(Self(Box::new(s.parse::<Then>()?) as Box<dyn Any>)),
            "TIES" => Ok(Self(Box::new(s.parse::<Ties>()?) as Box<dyn Any>)),
            "TO" => Ok(Self(Box::new(s.parse::<To>()?) as Box<dyn Any>)),
            "TRANSACTION" => Ok(Self(Box::new(s.parse::<Transaction>()?) as Box<dyn Any>)),
            "TRIGGER" => Ok(Self(Box::new(s.parse::<Trigger>()?) as Box<dyn Any>)),
            "UNBOUNDED" => Ok(Self(Box::new(s.parse::<Unbounded>()?) as Box<dyn Any>)),
            "UNION" => Ok(Self(Box::new(s.parse::<Union>()?) as Box<dyn Any>)),
            "UNIQUE" => Ok(Self(Box::new(s.parse::<Unique>()?) as Box<dyn Any>)),
            "UPDATE" => Ok(Self(Box::new(s.parse::<Update>()?) as Box<dyn Any>)),
            "USING" => Ok(Self(Box::new(s.parse::<Using>()?) as Box<dyn Any>)),
            "VACUUM" => Ok(Self(Box::new(s.parse::<Vacuum>()?) as Box<dyn Any>)),
            "VALUES" => Ok(Self(Box::new(s.parse::<Values>()?) as Box<dyn Any>)),
            "VIEW" => Ok(Self(Box::new(s.parse::<View>()?) as Box<dyn Any>)),
            "VIRTUAL" => Ok(Self(Box::new(s.parse::<Virtual>()?) as Box<dyn Any>)),
            "WHEN" => Ok(Self(Box::new(s.parse::<When>()?) as Box<dyn Any>)),
            "WHERE" => Ok(Self(Box::new(s.parse::<Where>()?) as Box<dyn Any>)),
            "WINDOW" => Ok(Self(Box::new(s.parse::<Window>()?) as Box<dyn Any>)),
            "WITH" => Ok(Self(Box::new(s.parse::<With>()?) as Box<dyn Any>)),
            "WITHOUT" => Ok(Self(Box::new(s.parse::<Without>()?) as Box<dyn Any>)),
            _ => Err(SqliteError::SqlParser(SqlParserError(
                "Keyword not found.".into(),
            ))),
        }
    }
}
