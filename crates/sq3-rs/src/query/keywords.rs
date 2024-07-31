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
    first::First, following::Following, for_::For, foreign::Foreign, from::From as KeywordFrom,
    full::Full, generated::Generated, glob::Glob, group::Group, groups::Groups, having::Having,
    if_::If, ignore::Ignore, immediate::Immediate, in_::In, index::Index, indexed::Indexed,
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

// #[derive(Debug)]
// pub struct SqliteKeywords;

// // impl SqliteKeywords {
// //     pub const fn all_keywords() -> [&'static str; 1] {
// //         [Abort::as_str()]
// //     }
// // }

#[derive(Debug)]
pub struct Keyword(pub Box<dyn Any>);

impl FromStr for Keyword {
    type Err = SqliteError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if Abort::as_str() == input {
            return Ok(Self(Box::new(Abort) as Box<dyn Any>));
        }
        if Action::as_str() == input {
            return Ok(Self(Box::new(Action) as Box<dyn Any>));
        }
        if Add::as_str() == input {
            return Ok(Self(Box::new(Add) as Box<dyn Any>));
        }
        if After::as_str() == input {
            return Ok(Self(Box::new(After) as Box<dyn Any>));
        }
        if All::as_str() == input {
            return Ok(Self(Box::new(All) as Box<dyn Any>));
        }
        if Alter::as_str() == input {
            return Ok(Self(Box::new(Alter) as Box<dyn Any>));
        }
        if Always::as_str() == input {
            return Ok(Self(Box::new(Always) as Box<dyn Any>));
        }
        if Analyze::as_str() == input {
            return Ok(Self(Box::new(Analyze) as Box<dyn Any>));
        }
        if And::as_str() == input {
            return Ok(Self(Box::new(And) as Box<dyn Any>));
        }
        if As::as_str() == input {
            return Ok(Self(Box::new(As) as Box<dyn Any>));
        }
        if Asc::as_str() == input {
            return Ok(Self(Box::new(Asc) as Box<dyn Any>));
        }
        if Attach::as_str() == input {
            return Ok(Self(Box::new(Attach) as Box<dyn Any>));
        }
        if Autoincrement::as_str() == input {
            return Ok(Self(Box::new(Autoincrement) as Box<dyn Any>));
        }
        if Before::as_str() == input {
            return Ok(Self(Box::new(Before) as Box<dyn Any>));
        }
        if Begin::as_str() == input {
            return Ok(Self(Box::new(Begin) as Box<dyn Any>));
        }
        if Between::as_str() == input {
            return Ok(Self(Box::new(Between) as Box<dyn Any>));
        }
        if By::as_str() == input {
            return Ok(Self(Box::new(By) as Box<dyn Any>));
        }
        if Cascade::as_str() == input {
            return Ok(Self(Box::new(Cascade) as Box<dyn Any>));
        }
        if Case::as_str() == input {
            return Ok(Self(Box::new(Case) as Box<dyn Any>));
        }
        if Cast::as_str() == input {
            return Ok(Self(Box::new(Cast) as Box<dyn Any>));
        }
        if Check::as_str() == input {
            return Ok(Self(Box::new(Check) as Box<dyn Any>));
        }
        if Collate::as_str() == input {
            return Ok(Self(Box::new(Collate) as Box<dyn Any>));
        }
        if Column::as_str() == input {
            return Ok(Self(Box::new(Column) as Box<dyn Any>));
        }
        if Commit::as_str() == input {
            return Ok(Self(Box::new(Commit) as Box<dyn Any>));
        }
        if Conflict::as_str() == input {
            return Ok(Self(Box::new(Conflict) as Box<dyn Any>));
        }
        if Constraint::as_str() == input {
            return Ok(Self(Box::new(Constraint) as Box<dyn Any>));
        }
        if Create::as_str() == input {
            return Ok(Self(Box::new(Create) as Box<dyn Any>));
        }
        if Cross::as_str() == input {
            return Ok(Self(Box::new(Cross) as Box<dyn Any>));
        }
        if Current::as_str() == input {
            return Ok(Self(Box::new(Current) as Box<dyn Any>));
        }
        if Current_date::as_str() == input {
            return Ok(Self(Box::new(Current_date) as Box<dyn Any>));
        }
        if Current_time::as_str() == input {
            return Ok(Self(Box::new(Current_time) as Box<dyn Any>));
        }
        if Current_timestamp::as_str() == input {
            return Ok(Self(Box::new(Current_timestamp) as Box<dyn Any>));
        }
        if Database::as_str() == input {
            return Ok(Self(Box::new(Database) as Box<dyn Any>));
        }
        if Default::as_str() == input {
            return Ok(Self(Box::new(Default) as Box<dyn Any>));
        }
        if Deferrable::as_str() == input {
            return Ok(Self(Box::new(Deferrable) as Box<dyn Any>));
        }
        if Deferred::as_str() == input {
            return Ok(Self(Box::new(Deferred) as Box<dyn Any>));
        }
        if Delete::as_str() == input {
            return Ok(Self(Box::new(Delete) as Box<dyn Any>));
        }
        if Desc::as_str() == input {
            return Ok(Self(Box::new(Desc) as Box<dyn Any>));
        }
        if Detach::as_str() == input {
            return Ok(Self(Box::new(Detach) as Box<dyn Any>));
        }
        if Distinct::as_str() == input {
            return Ok(Self(Box::new(Distinct) as Box<dyn Any>));
        }
        if Do::as_str() == input {
            return Ok(Self(Box::new(Do) as Box<dyn Any>));
        }
        if Drop::as_str() == input {
            return Ok(Self(Box::new(Drop) as Box<dyn Any>));
        }
        if Each::as_str() == input {
            return Ok(Self(Box::new(Each) as Box<dyn Any>));
        }
        if Else::as_str() == input {
            return Ok(Self(Box::new(Else) as Box<dyn Any>));
        }
        if End::as_str() == input {
            return Ok(Self(Box::new(End) as Box<dyn Any>));
        }
        if Escape::as_str() == input {
            return Ok(Self(Box::new(Escape) as Box<dyn Any>));
        }
        if Except::as_str() == input {
            return Ok(Self(Box::new(Except) as Box<dyn Any>));
        }
        if Exclude::as_str() == input {
            return Ok(Self(Box::new(Exclude) as Box<dyn Any>));
        }
        if Exclusive::as_str() == input {
            return Ok(Self(Box::new(Exclusive) as Box<dyn Any>));
        }
        if Exists::as_str() == input {
            return Ok(Self(Box::new(Exists) as Box<dyn Any>));
        }
        if Explain::as_str() == input {
            return Ok(Self(Box::new(Explain) as Box<dyn Any>));
        }
        if Fail::as_str() == input {
            return Ok(Self(Box::new(Fail) as Box<dyn Any>));
        }
        if Filter::as_str() == input {
            return Ok(Self(Box::new(Filter) as Box<dyn Any>));
        }
        if First::as_str() == input {
            return Ok(Self(Box::new(First) as Box<dyn Any>));
        }
        if Following::as_str() == input {
            return Ok(Self(Box::new(Following) as Box<dyn Any>));
        }
        if For::as_str() == input {
            return Ok(Self(Box::new(For) as Box<dyn Any>));
        }
        if Foreign::as_str() == input {
            return Ok(Self(Box::new(Foreign) as Box<dyn Any>));
        }
        if KeywordFrom::as_str() == input {
            return Ok(Self(Box::new(KeywordFrom) as Box<dyn Any>));
        }
        if Full::as_str() == input {
            return Ok(Self(Box::new(Full) as Box<dyn Any>));
        }
        if Generated::as_str() == input {
            return Ok(Self(Box::new(Generated) as Box<dyn Any>));
        }
        if Glob::as_str() == input {
            return Ok(Self(Box::new(Glob) as Box<dyn Any>));
        }
        if Group::as_str() == input {
            return Ok(Self(Box::new(Group) as Box<dyn Any>));
        }
        if Groups::as_str() == input {
            return Ok(Self(Box::new(Groups) as Box<dyn Any>));
        }
        if Having::as_str() == input {
            return Ok(Self(Box::new(Having) as Box<dyn Any>));
        }
        if If::as_str() == input {
            return Ok(Self(Box::new(If) as Box<dyn Any>));
        }
        if Ignore::as_str() == input {
            return Ok(Self(Box::new(Ignore) as Box<dyn Any>));
        }
        if Immediate::as_str() == input {
            return Ok(Self(Box::new(Immediate) as Box<dyn Any>));
        }
        if In::as_str() == input {
            return Ok(Self(Box::new(In) as Box<dyn Any>));
        }
        if Index::as_str() == input {
            return Ok(Self(Box::new(Index) as Box<dyn Any>));
        }
        if Indexed::as_str() == input {
            return Ok(Self(Box::new(Indexed) as Box<dyn Any>));
        }
        if Initially::as_str() == input {
            return Ok(Self(Box::new(Initially) as Box<dyn Any>));
        }
        if Inner::as_str() == input {
            return Ok(Self(Box::new(Inner) as Box<dyn Any>));
        }
        if Insert::as_str() == input {
            return Ok(Self(Box::new(Insert) as Box<dyn Any>));
        }
        if Instead::as_str() == input {
            return Ok(Self(Box::new(Instead) as Box<dyn Any>));
        }
        if Intersect::as_str() == input {
            return Ok(Self(Box::new(Intersect) as Box<dyn Any>));
        }
        if Into::as_str() == input {
            return Ok(Self(Box::new(Into) as Box<dyn Any>));
        }
        if Is::as_str() == input {
            return Ok(Self(Box::new(Is) as Box<dyn Any>));
        }
        if Isnull::as_str() == input {
            return Ok(Self(Box::new(Isnull) as Box<dyn Any>));
        }
        if Join::as_str() == input {
            return Ok(Self(Box::new(Join) as Box<dyn Any>));
        }
        if Key::as_str() == input {
            return Ok(Self(Box::new(Key) as Box<dyn Any>));
        }
        if Last::as_str() == input {
            return Ok(Self(Box::new(Last) as Box<dyn Any>));
        }
        if Left::as_str() == input {
            return Ok(Self(Box::new(Left) as Box<dyn Any>));
        }
        if Like::as_str() == input {
            return Ok(Self(Box::new(Like) as Box<dyn Any>));
        }
        if Limit::as_str() == input {
            return Ok(Self(Box::new(Limit) as Box<dyn Any>));
        }
        if Match::as_str() == input {
            return Ok(Self(Box::new(Match) as Box<dyn Any>));
        }
        if Materialized::as_str() == input {
            return Ok(Self(Box::new(Materialized) as Box<dyn Any>));
        }
        if Natural::as_str() == input {
            return Ok(Self(Box::new(Natural) as Box<dyn Any>));
        }
        if No::as_str() == input {
            return Ok(Self(Box::new(No) as Box<dyn Any>));
        }
        if Not::as_str() == input {
            return Ok(Self(Box::new(Not) as Box<dyn Any>));
        }
        if Nothing::as_str() == input {
            return Ok(Self(Box::new(Nothing) as Box<dyn Any>));
        }
        if Notnull::as_str() == input {
            return Ok(Self(Box::new(Notnull) as Box<dyn Any>));
        }
        if Null::as_str() == input {
            return Ok(Self(Box::new(Null) as Box<dyn Any>));
        }
        if Nulls::as_str() == input {
            return Ok(Self(Box::new(Nulls) as Box<dyn Any>));
        }
        if Of::as_str() == input {
            return Ok(Self(Box::new(Of) as Box<dyn Any>));
        }
        if Offset::as_str() == input {
            return Ok(Self(Box::new(Offset) as Box<dyn Any>));
        }
        if On::as_str() == input {
            return Ok(Self(Box::new(On) as Box<dyn Any>));
        }
        if Or::as_str() == input {
            return Ok(Self(Box::new(Or) as Box<dyn Any>));
        }
        if Order::as_str() == input {
            return Ok(Self(Box::new(Order) as Box<dyn Any>));
        }
        if Others::as_str() == input {
            return Ok(Self(Box::new(Others) as Box<dyn Any>));
        }
        if Outer::as_str() == input {
            return Ok(Self(Box::new(Outer) as Box<dyn Any>));
        }
        if Over::as_str() == input {
            return Ok(Self(Box::new(Over) as Box<dyn Any>));
        }
        if Partition::as_str() == input {
            return Ok(Self(Box::new(Partition) as Box<dyn Any>));
        }
        if Plan::as_str() == input {
            return Ok(Self(Box::new(Plan) as Box<dyn Any>));
        }
        if Pragma::as_str() == input {
            return Ok(Self(Box::new(Pragma) as Box<dyn Any>));
        }
        if Preceding::as_str() == input {
            return Ok(Self(Box::new(Preceding) as Box<dyn Any>));
        }
        if Primary::as_str() == input {
            return Ok(Self(Box::new(Primary) as Box<dyn Any>));
        }
        if Query::as_str() == input {
            return Ok(Self(Box::new(Query) as Box<dyn Any>));
        }
        if Raise::as_str() == input {
            return Ok(Self(Box::new(Raise) as Box<dyn Any>));
        }
        if Range::as_str() == input {
            return Ok(Self(Box::new(Range) as Box<dyn Any>));
        }
        if Recursive::as_str() == input {
            return Ok(Self(Box::new(Recursive) as Box<dyn Any>));
        }
        if References::as_str() == input {
            return Ok(Self(Box::new(References) as Box<dyn Any>));
        }
        if Regexp::as_str() == input {
            return Ok(Self(Box::new(Regexp) as Box<dyn Any>));
        }
        if Reindex::as_str() == input {
            return Ok(Self(Box::new(Reindex) as Box<dyn Any>));
        }
        if Release::as_str() == input {
            return Ok(Self(Box::new(Release) as Box<dyn Any>));
        }
        if Rename::as_str() == input {
            return Ok(Self(Box::new(Rename) as Box<dyn Any>));
        }
        if Replace::as_str() == input {
            return Ok(Self(Box::new(Replace) as Box<dyn Any>));
        }
        if Restrict::as_str() == input {
            return Ok(Self(Box::new(Restrict) as Box<dyn Any>));
        }
        if Returning::as_str() == input {
            return Ok(Self(Box::new(Returning) as Box<dyn Any>));
        }
        if Right::as_str() == input {
            return Ok(Self(Box::new(Right) as Box<dyn Any>));
        }
        if Rollback::as_str() == input {
            return Ok(Self(Box::new(Rollback) as Box<dyn Any>));
        }
        if Row::as_str() == input {
            return Ok(Self(Box::new(Row) as Box<dyn Any>));
        }
        if Rows::as_str() == input {
            return Ok(Self(Box::new(Rows) as Box<dyn Any>));
        }
        if Savepoint::as_str() == input {
            return Ok(Self(Box::new(Savepoint) as Box<dyn Any>));
        }
        if Select::as_str() == input {
            return Ok(Self(Box::new(Select) as Box<dyn Any>));
        }
        if Set::as_str() == input {
            return Ok(Self(Box::new(Set) as Box<dyn Any>));
        }
        if Table::as_str() == input {
            return Ok(Self(Box::new(Table) as Box<dyn Any>));
        }
        if Temp::as_str() == input {
            return Ok(Self(Box::new(Temp) as Box<dyn Any>));
        }
        if Temporary::as_str() == input {
            return Ok(Self(Box::new(Temporary) as Box<dyn Any>));
        }
        if Then::as_str() == input {
            return Ok(Self(Box::new(Then) as Box<dyn Any>));
        }
        if Ties::as_str() == input {
            return Ok(Self(Box::new(Ties) as Box<dyn Any>));
        }
        if To::as_str() == input {
            return Ok(Self(Box::new(To) as Box<dyn Any>));
        }
        if Transaction::as_str() == input {
            return Ok(Self(Box::new(Transaction) as Box<dyn Any>));
        }
        if Trigger::as_str() == input {
            return Ok(Self(Box::new(Trigger) as Box<dyn Any>));
        }
        if Unbounded::as_str() == input {
            return Ok(Self(Box::new(Unbounded) as Box<dyn Any>));
        }
        if Union::as_str() == input {
            return Ok(Self(Box::new(Union) as Box<dyn Any>));
        }
        if Unique::as_str() == input {
            return Ok(Self(Box::new(Unique) as Box<dyn Any>));
        }
        if Update::as_str() == input {
            return Ok(Self(Box::new(Update) as Box<dyn Any>));
        }
        if Using::as_str() == input {
            return Ok(Self(Box::new(Using) as Box<dyn Any>));
        }
        if Vacuum::as_str() == input {
            return Ok(Self(Box::new(Vacuum) as Box<dyn Any>));
        }
        if Values::as_str() == input {
            return Ok(Self(Box::new(Values) as Box<dyn Any>));
        }
        if View::as_str() == input {
            return Ok(Self(Box::new(View) as Box<dyn Any>));
        }
        if Virtual::as_str() == input {
            return Ok(Self(Box::new(Virtual) as Box<dyn Any>));
        }
        if When::as_str() == input {
            return Ok(Self(Box::new(When) as Box<dyn Any>));
        }
        if Where::as_str() == input {
            return Ok(Self(Box::new(Where) as Box<dyn Any>));
        }
        if Window::as_str() == input {
            return Ok(Self(Box::new(Window) as Box<dyn Any>));
        }
        if With::as_str() == input {
            return Ok(Self(Box::new(With) as Box<dyn Any>));
        }
        if Without::as_str() == input {
            return Ok(Self(Box::new(Without) as Box<dyn Any>));
        }

        Err(SqliteError::SqlParser(SqlParserError(
            "Keyword not found.".into(),
        )))
    }
}
