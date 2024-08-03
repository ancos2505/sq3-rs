#[cfg(test)]
mod tests;

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

use std::any::Any;
use std::fmt::Debug;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct Keyword(Box<dyn Any>);
impl Keyword {
    pub fn get(&self) -> &Box<dyn Any> {
        &self.0
    }
    pub fn into_inner(self) -> Box<dyn Any> {
        Box::new(self)
    }
}

impl FromStr for Keyword {
    type Err = SqliteError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if Abort == input {
            return Ok(Self(Box::new(Abort) as Box<dyn Any>));
        }
        if Action == input {
            return Ok(Self(Box::new(Action) as Box<dyn Any>));
        }
        if Add == input {
            return Ok(Self(Box::new(Add) as Box<dyn Any>));
        }
        if After == input {
            return Ok(Self(Box::new(After) as Box<dyn Any>));
        }
        if All == input {
            return Ok(Self(Box::new(All) as Box<dyn Any>));
        }
        if Alter == input {
            return Ok(Self(Box::new(Alter) as Box<dyn Any>));
        }
        if Always == input {
            return Ok(Self(Box::new(Always) as Box<dyn Any>));
        }
        if Analyze == input {
            return Ok(Self(Box::new(Analyze) as Box<dyn Any>));
        }
        if And == input {
            return Ok(Self(Box::new(And) as Box<dyn Any>));
        }
        if As == input {
            return Ok(Self(Box::new(As) as Box<dyn Any>));
        }
        if Asc == input {
            return Ok(Self(Box::new(Asc) as Box<dyn Any>));
        }
        if Attach == input {
            return Ok(Self(Box::new(Attach) as Box<dyn Any>));
        }
        if Autoincrement == input {
            return Ok(Self(Box::new(Autoincrement) as Box<dyn Any>));
        }
        if Before == input {
            return Ok(Self(Box::new(Before) as Box<dyn Any>));
        }
        if Begin == input {
            return Ok(Self(Box::new(Begin) as Box<dyn Any>));
        }
        if Between == input {
            return Ok(Self(Box::new(Between) as Box<dyn Any>));
        }
        if By == input {
            return Ok(Self(Box::new(By) as Box<dyn Any>));
        }
        if Cascade == input {
            return Ok(Self(Box::new(Cascade) as Box<dyn Any>));
        }
        if Case == input {
            return Ok(Self(Box::new(Case) as Box<dyn Any>));
        }
        if Cast == input {
            return Ok(Self(Box::new(Cast) as Box<dyn Any>));
        }
        if Check == input {
            return Ok(Self(Box::new(Check) as Box<dyn Any>));
        }
        if Collate == input {
            return Ok(Self(Box::new(Collate) as Box<dyn Any>));
        }
        if Column == input {
            return Ok(Self(Box::new(Column) as Box<dyn Any>));
        }
        if Commit == input {
            return Ok(Self(Box::new(Commit) as Box<dyn Any>));
        }
        if Conflict == input {
            return Ok(Self(Box::new(Conflict) as Box<dyn Any>));
        }
        if Constraint == input {
            return Ok(Self(Box::new(Constraint) as Box<dyn Any>));
        }
        if Create == input {
            return Ok(Self(Box::new(Create) as Box<dyn Any>));
        }
        if Cross == input {
            return Ok(Self(Box::new(Cross) as Box<dyn Any>));
        }
        if Current == input {
            return Ok(Self(Box::new(Current) as Box<dyn Any>));
        }
        if Current_date == input {
            return Ok(Self(Box::new(Current_date) as Box<dyn Any>));
        }
        if Current_time == input {
            return Ok(Self(Box::new(Current_time) as Box<dyn Any>));
        }
        if Current_timestamp == input {
            return Ok(Self(Box::new(Current_timestamp) as Box<dyn Any>));
        }
        if Database == input {
            return Ok(Self(Box::new(Database) as Box<dyn Any>));
        }
        if Default == input {
            return Ok(Self(Box::new(Default) as Box<dyn Any>));
        }
        if Deferrable == input {
            return Ok(Self(Box::new(Deferrable) as Box<dyn Any>));
        }
        if Deferred == input {
            return Ok(Self(Box::new(Deferred) as Box<dyn Any>));
        }
        if Delete == input {
            return Ok(Self(Box::new(Delete) as Box<dyn Any>));
        }
        if Desc == input {
            return Ok(Self(Box::new(Desc) as Box<dyn Any>));
        }
        if Detach == input {
            return Ok(Self(Box::new(Detach) as Box<dyn Any>));
        }
        if Distinct == input {
            return Ok(Self(Box::new(Distinct) as Box<dyn Any>));
        }
        if Do == input {
            return Ok(Self(Box::new(Do) as Box<dyn Any>));
        }
        if Drop == input {
            return Ok(Self(Box::new(Drop) as Box<dyn Any>));
        }
        if Each == input {
            return Ok(Self(Box::new(Each) as Box<dyn Any>));
        }
        if Else == input {
            return Ok(Self(Box::new(Else) as Box<dyn Any>));
        }
        if End == input {
            return Ok(Self(Box::new(End) as Box<dyn Any>));
        }
        if Escape == input {
            return Ok(Self(Box::new(Escape) as Box<dyn Any>));
        }
        if Except == input {
            return Ok(Self(Box::new(Except) as Box<dyn Any>));
        }
        if Exclude == input {
            return Ok(Self(Box::new(Exclude) as Box<dyn Any>));
        }
        if Exclusive == input {
            return Ok(Self(Box::new(Exclusive) as Box<dyn Any>));
        }
        if Exists == input {
            return Ok(Self(Box::new(Exists) as Box<dyn Any>));
        }
        if Explain == input {
            return Ok(Self(Box::new(Explain) as Box<dyn Any>));
        }
        if Fail == input {
            return Ok(Self(Box::new(Fail) as Box<dyn Any>));
        }
        if Filter == input {
            return Ok(Self(Box::new(Filter) as Box<dyn Any>));
        }
        if First == input {
            return Ok(Self(Box::new(First) as Box<dyn Any>));
        }
        if Following == input {
            return Ok(Self(Box::new(Following) as Box<dyn Any>));
        }
        if For == input {
            return Ok(Self(Box::new(For) as Box<dyn Any>));
        }
        if Foreign == input {
            return Ok(Self(Box::new(Foreign) as Box<dyn Any>));
        }
        if KeywordFrom == input {
            return Ok(Self(Box::new(KeywordFrom) as Box<dyn Any>));
        }
        if Full == input {
            return Ok(Self(Box::new(Full) as Box<dyn Any>));
        }
        if Generated == input {
            return Ok(Self(Box::new(Generated) as Box<dyn Any>));
        }
        if Glob == input {
            return Ok(Self(Box::new(Glob) as Box<dyn Any>));
        }
        if Group == input {
            return Ok(Self(Box::new(Group) as Box<dyn Any>));
        }
        if Groups == input {
            return Ok(Self(Box::new(Groups) as Box<dyn Any>));
        }
        if Having == input {
            return Ok(Self(Box::new(Having) as Box<dyn Any>));
        }
        if If == input {
            return Ok(Self(Box::new(If) as Box<dyn Any>));
        }
        if Ignore == input {
            return Ok(Self(Box::new(Ignore) as Box<dyn Any>));
        }
        if Immediate == input {
            return Ok(Self(Box::new(Immediate) as Box<dyn Any>));
        }
        if In == input {
            return Ok(Self(Box::new(In) as Box<dyn Any>));
        }
        if Index == input {
            return Ok(Self(Box::new(Index) as Box<dyn Any>));
        }
        if Indexed == input {
            return Ok(Self(Box::new(Indexed) as Box<dyn Any>));
        }
        if Initially == input {
            return Ok(Self(Box::new(Initially) as Box<dyn Any>));
        }
        if Inner == input {
            return Ok(Self(Box::new(Inner) as Box<dyn Any>));
        }
        if Insert == input {
            return Ok(Self(Box::new(Insert) as Box<dyn Any>));
        }
        if Instead == input {
            return Ok(Self(Box::new(Instead) as Box<dyn Any>));
        }
        if Intersect == input {
            return Ok(Self(Box::new(Intersect) as Box<dyn Any>));
        }
        if Into == input {
            return Ok(Self(Box::new(Into) as Box<dyn Any>));
        }
        if Is == input {
            return Ok(Self(Box::new(Is) as Box<dyn Any>));
        }
        if Isnull == input {
            return Ok(Self(Box::new(Isnull) as Box<dyn Any>));
        }
        if Join == input {
            return Ok(Self(Box::new(Join) as Box<dyn Any>));
        }
        if Key == input {
            return Ok(Self(Box::new(Key) as Box<dyn Any>));
        }
        if Last == input {
            return Ok(Self(Box::new(Last) as Box<dyn Any>));
        }
        if Left == input {
            return Ok(Self(Box::new(Left) as Box<dyn Any>));
        }
        if Like == input {
            return Ok(Self(Box::new(Like) as Box<dyn Any>));
        }
        if Limit == input {
            return Ok(Self(Box::new(Limit) as Box<dyn Any>));
        }
        if Match == input {
            return Ok(Self(Box::new(Match) as Box<dyn Any>));
        }
        if Materialized == input {
            return Ok(Self(Box::new(Materialized) as Box<dyn Any>));
        }
        if Natural == input {
            return Ok(Self(Box::new(Natural) as Box<dyn Any>));
        }
        if No == input {
            return Ok(Self(Box::new(No) as Box<dyn Any>));
        }
        if Not == input {
            return Ok(Self(Box::new(Not) as Box<dyn Any>));
        }
        if Nothing == input {
            return Ok(Self(Box::new(Nothing) as Box<dyn Any>));
        }
        if Notnull == input {
            return Ok(Self(Box::new(Notnull) as Box<dyn Any>));
        }
        if Null == input {
            return Ok(Self(Box::new(Null) as Box<dyn Any>));
        }
        if Nulls == input {
            return Ok(Self(Box::new(Nulls) as Box<dyn Any>));
        }
        if Of == input {
            return Ok(Self(Box::new(Of) as Box<dyn Any>));
        }
        if Offset == input {
            return Ok(Self(Box::new(Offset) as Box<dyn Any>));
        }
        if On == input {
            return Ok(Self(Box::new(On) as Box<dyn Any>));
        }
        if Or == input {
            return Ok(Self(Box::new(Or) as Box<dyn Any>));
        }
        if Order == input {
            return Ok(Self(Box::new(Order) as Box<dyn Any>));
        }
        if Others == input {
            return Ok(Self(Box::new(Others) as Box<dyn Any>));
        }
        if Outer == input {
            return Ok(Self(Box::new(Outer) as Box<dyn Any>));
        }
        if Over == input {
            return Ok(Self(Box::new(Over) as Box<dyn Any>));
        }
        if Partition == input {
            return Ok(Self(Box::new(Partition) as Box<dyn Any>));
        }
        if Plan == input {
            return Ok(Self(Box::new(Plan) as Box<dyn Any>));
        }
        if Pragma == input {
            return Ok(Self(Box::new(Pragma) as Box<dyn Any>));
        }
        if Preceding == input {
            return Ok(Self(Box::new(Preceding) as Box<dyn Any>));
        }
        if Primary == input {
            return Ok(Self(Box::new(Primary) as Box<dyn Any>));
        }
        if Query == input {
            return Ok(Self(Box::new(Query) as Box<dyn Any>));
        }
        if Raise == input {
            return Ok(Self(Box::new(Raise) as Box<dyn Any>));
        }
        if Range == input {
            return Ok(Self(Box::new(Range) as Box<dyn Any>));
        }
        if Recursive == input {
            return Ok(Self(Box::new(Recursive) as Box<dyn Any>));
        }
        if References == input {
            return Ok(Self(Box::new(References) as Box<dyn Any>));
        }
        if Regexp == input {
            return Ok(Self(Box::new(Regexp) as Box<dyn Any>));
        }
        if Reindex == input {
            return Ok(Self(Box::new(Reindex) as Box<dyn Any>));
        }
        if Release == input {
            return Ok(Self(Box::new(Release) as Box<dyn Any>));
        }
        if Rename == input {
            return Ok(Self(Box::new(Rename) as Box<dyn Any>));
        }
        if Replace == input {
            return Ok(Self(Box::new(Replace) as Box<dyn Any>));
        }
        if Restrict == input {
            return Ok(Self(Box::new(Restrict) as Box<dyn Any>));
        }
        if Returning == input {
            return Ok(Self(Box::new(Returning) as Box<dyn Any>));
        }
        if Right == input {
            return Ok(Self(Box::new(Right) as Box<dyn Any>));
        }
        if Rollback == input {
            return Ok(Self(Box::new(Rollback) as Box<dyn Any>));
        }
        if Row == input {
            return Ok(Self(Box::new(Row) as Box<dyn Any>));
        }
        if Rows == input {
            return Ok(Self(Box::new(Rows) as Box<dyn Any>));
        }
        if Savepoint == input {
            return Ok(Self(Box::new(Savepoint) as Box<dyn Any>));
        }
        if Select == input {
            return Ok(Self(Box::new(Select) as Box<dyn Any>));
        }
        if Set == input {
            return Ok(Self(Box::new(Set) as Box<dyn Any>));
        }
        if Table == input {
            return Ok(Self(Box::new(Table) as Box<dyn Any>));
        }
        if Temp == input {
            return Ok(Self(Box::new(Temp) as Box<dyn Any>));
        }
        if Temporary == input {
            return Ok(Self(Box::new(Temporary) as Box<dyn Any>));
        }
        if Then == input {
            return Ok(Self(Box::new(Then) as Box<dyn Any>));
        }
        if Ties == input {
            return Ok(Self(Box::new(Ties) as Box<dyn Any>));
        }
        if To == input {
            return Ok(Self(Box::new(To) as Box<dyn Any>));
        }
        if Transaction == input {
            return Ok(Self(Box::new(Transaction) as Box<dyn Any>));
        }
        if Trigger == input {
            return Ok(Self(Box::new(Trigger) as Box<dyn Any>));
        }
        if Unbounded == input {
            return Ok(Self(Box::new(Unbounded) as Box<dyn Any>));
        }
        if Union == input {
            return Ok(Self(Box::new(Union) as Box<dyn Any>));
        }
        if Unique == input {
            return Ok(Self(Box::new(Unique) as Box<dyn Any>));
        }
        if Update == input {
            return Ok(Self(Box::new(Update) as Box<dyn Any>));
        }
        if Using == input {
            return Ok(Self(Box::new(Using) as Box<dyn Any>));
        }
        if Vacuum == input {
            return Ok(Self(Box::new(Vacuum) as Box<dyn Any>));
        }
        if Values == input {
            return Ok(Self(Box::new(Values) as Box<dyn Any>));
        }
        if View == input {
            return Ok(Self(Box::new(View) as Box<dyn Any>));
        }
        if Virtual == input {
            return Ok(Self(Box::new(Virtual) as Box<dyn Any>));
        }
        if When == input {
            return Ok(Self(Box::new(When) as Box<dyn Any>));
        }
        if Where == input {
            return Ok(Self(Box::new(Where) as Box<dyn Any>));
        }
        if Window == input {
            return Ok(Self(Box::new(Window) as Box<dyn Any>));
        }
        if With == input {
            return Ok(Self(Box::new(With) as Box<dyn Any>));
        }
        if Without == input {
            return Ok(Self(Box::new(Without) as Box<dyn Any>));
        }

        Err(SqliteError::SqlParser(SqlParserError(
            "Not a valid SQLite keyword.".into(),
        )))
    }
}
