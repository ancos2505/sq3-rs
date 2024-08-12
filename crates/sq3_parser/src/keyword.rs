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

use crate::result::Sq3ParserError;

pub(crate) use self::{
    abort::Abort as KeywordAbort, action::Action as KeywordAction, add::Add as KeywordAdd,
    after::After as KeywordAfter, all::All as KeywordAll, alter::Alter as KeywordAlter,
    always::Always as KeywordAlways, analyze::Analyze as KeywordAnalyze, and::And as KeywordAnd,
    as_::As as KeywordAs, asc::Asc as KeywordAsc, attach::Attach as KeywordAttach,
    autoincrement::Autoincrement as KeywordAutoincrement, before::Before as KeywordBefore,
    begin::Begin as KeywordBegin, between::Between as KeywordBetween, by::By as KeywordBy,
    cascade::Cascade as KeywordCascade, case::Case as KeywordCase, cast::Cast as KeywordCast,
    check::Check as KeywordCheck, collate::Collate as KeywordCollate,
    column::Column as KeywordColumn, commit::Commit as KeywordCommit,
    conflict::Conflict as KeywordConflict, constraint::Constraint as KeywordConstraint,
    create::Create as KeywordCreate, cross::Cross as KeywordCross,
    current::Current as KeywordCurrent, current_date::Current_date as KeywordCurrent_date,
    current_time::Current_time as KeywordCurrent_time,
    current_timestamp::Current_timestamp as KeywordCurrent_timestamp,
    database::Database as KeywordDatabase, default::Default as KeywordDefault,
    deferrable::Deferrable as KeywordDeferrable, deferred::Deferred as KeywordDeferred,
    delete::Delete as KeywordDelete, desc::Desc as KeywordDesc, detach::Detach as KeywordDetach,
    distinct::Distinct as KeywordDistinct, do_::Do as KeywordDo, drop::Drop as KeywordDrop,
    each::Each as KeywordEach, else_::Else as KeywordElse, end::End as KeywordEnd,
    escape::Escape as KeywordEscape, except::Except as KeywordExcept,
    exclude::Exclude as KeywordExclude, exclusive::Exclusive as KeywordExclusive,
    exists::Exists as KeywordExists, explain::Explain as KeywordExplain, fail::Fail as KeywordFail,
    filter::Filter as KeywordFilter, first::First as KeywordFirst,
    following::Following as KeywordFollowing, for_::For as KeywordFor,
    foreign::Foreign as KeywordForeign, from::From as KeywordFrom, full::Full as KeywordFull,
    generated::Generated as KeywordGenerated, glob::Glob as KeywordGlob,
    group::Group as KeywordGroup, groups::Groups as KeywordGroups, having::Having as KeywordHaving,
    if_::If as KeywordIf, ignore::Ignore as KeywordIgnore,
    immediate::Immediate as KeywordImmediate, in_::In as KeywordIn, index::Index as KeywordIndex,
    indexed::Indexed as KeywordIndexed, initially::Initially as KeywordInitially,
    inner::Inner as KeywordInner, insert::Insert as KeywordInsert,
    instead::Instead as KeywordInstead, intersect::Intersect as KeywordIntersect,
    into::Into as KeywordInto, is::Is as KeywordIs, isnull::Isnull as KeywordIsnull,
    join::Join as KeywordJoin, key::Key as KeywordKey, last::Last as KeywordLast,
    left::Left as KeywordLeft, like::Like as KeywordLike, limit::Limit as KeywordLimit,
    match_::Match as KeywordMatch, materialized::Materialized as KeywordMaterialized,
    natural::Natural as KeywordNatural, no::No as KeywordNo, not::Not as KeywordNot,
    nothing::Nothing as KeywordNothing, notnull::Notnull as KeywordNotnull,
    null::Null as KeywordNull, nulls::Nulls as KeywordNulls, of::Of as KeywordOf,
    offset::Offset as KeywordOffset, on::On as KeywordOn, or::Or as KeywordOr,
    order::Order as KeywordOrder, others::Others as KeywordOthers, outer::Outer as KeywordOuter,
    over::Over as KeywordOver, partition::Partition as KeywordPartition, plan::Plan as KeywordPlan,
    pragma::Pragma as KeywordPragma, preceding::Preceding as KeywordPreceding,
    primary::Primary as KeywordPrimary, query::Query as KeywordQuery, raise::Raise as KeywordRaise,
    range::Range as KeywordRange, recursive::Recursive as KeywordRecursive,
    references::References as KeywordReferences, regexp::Regexp as KeywordRegexp,
    reindex::Reindex as KeywordReindex, release::Release as KeywordRelease,
    rename::Rename as KeywordRename, replace::Replace as KeywordReplace,
    restrict::Restrict as KeywordRestrict, returning::Returning as KeywordReturning,
    right::Right as KeywordRight, rollback::Rollback as KeywordRollback, row::Row as KeywordRow,
    rows::Rows as KeywordRows, savepoint::Savepoint as KeywordSavepoint,
    select::Select as KeywordSelect, set::Set as KeywordSet, table::Table as KeywordTable,
    temp::Temp as KeywordTemp, temporary::Temporary as KeywordTemporary, then::Then as KeywordThen,
    ties::Ties as KeywordTies, to::To as KeywordTo, transaction::Transaction as KeywordTransaction,
    trigger::Trigger as KeywordTrigger, unbounded::Unbounded as KeywordUnbounded,
    union::Union as KeywordUnion, unique::Unique as KeywordUnique, update::Update as KeywordUpdate,
    using::Using as KeywordUsing, vacuum::Vacuum as KeywordVacuum, values::Values as KeywordValues,
    view::View as KeywordView, virtual_::Virtual as KeywordVirtual, when::When as KeywordWhen,
    where_::Where as KeywordWhere, window::Window as KeywordWindow, with::With as KeywordWith,
    without::Without as KeywordWithout,
};

#[derive(Debug)]
pub struct Keyword(Box<dyn Any>);
impl Keyword {
    pub fn get(&self) -> &Box<dyn Any> {
        &self.0
    }
    pub fn into_inner(self) -> Box<dyn Any> {
        self.0
    }
}

impl FromStr for Keyword {
    type Err = Sq3ParserError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if KeywordAbort == input {
            return Ok(Self(Box::new(KeywordAbort) as Box<dyn Any>));
        }
        if KeywordAction == input {
            return Ok(Self(Box::new(KeywordAction) as Box<dyn Any>));
        }
        if KeywordAdd == input {
            return Ok(Self(Box::new(KeywordAdd) as Box<dyn Any>));
        }
        if KeywordAfter == input {
            return Ok(Self(Box::new(KeywordAfter) as Box<dyn Any>));
        }
        if KeywordAll == input {
            return Ok(Self(Box::new(KeywordAll) as Box<dyn Any>));
        }
        if KeywordAlter == input {
            return Ok(Self(Box::new(KeywordAlter) as Box<dyn Any>));
        }
        if KeywordAlways == input {
            return Ok(Self(Box::new(KeywordAlways) as Box<dyn Any>));
        }
        if KeywordAnalyze == input {
            return Ok(Self(Box::new(KeywordAnalyze) as Box<dyn Any>));
        }
        if KeywordAnd == input {
            return Ok(Self(Box::new(KeywordAnd) as Box<dyn Any>));
        }
        if KeywordAs == input {
            return Ok(Self(Box::new(KeywordAs) as Box<dyn Any>));
        }
        if KeywordAsc == input {
            return Ok(Self(Box::new(KeywordAsc) as Box<dyn Any>));
        }
        if KeywordAttach == input {
            return Ok(Self(Box::new(KeywordAttach) as Box<dyn Any>));
        }
        if KeywordAutoincrement == input {
            return Ok(Self(Box::new(KeywordAutoincrement) as Box<dyn Any>));
        }
        if KeywordBefore == input {
            return Ok(Self(Box::new(KeywordBefore) as Box<dyn Any>));
        }
        if KeywordBegin == input {
            return Ok(Self(Box::new(KeywordBegin) as Box<dyn Any>));
        }
        if KeywordBetween == input {
            return Ok(Self(Box::new(KeywordBetween) as Box<dyn Any>));
        }
        if KeywordBy == input {
            return Ok(Self(Box::new(KeywordBy) as Box<dyn Any>));
        }
        if KeywordCascade == input {
            return Ok(Self(Box::new(KeywordCascade) as Box<dyn Any>));
        }
        if KeywordCase == input {
            return Ok(Self(Box::new(KeywordCase) as Box<dyn Any>));
        }
        if KeywordCast == input {
            return Ok(Self(Box::new(KeywordCast) as Box<dyn Any>));
        }
        if KeywordCheck == input {
            return Ok(Self(Box::new(KeywordCheck) as Box<dyn Any>));
        }
        if KeywordCollate == input {
            return Ok(Self(Box::new(KeywordCollate) as Box<dyn Any>));
        }
        if KeywordColumn == input {
            return Ok(Self(Box::new(KeywordColumn) as Box<dyn Any>));
        }
        if KeywordCommit == input {
            return Ok(Self(Box::new(KeywordCommit) as Box<dyn Any>));
        }
        if KeywordConflict == input {
            return Ok(Self(Box::new(KeywordConflict) as Box<dyn Any>));
        }
        if KeywordConstraint == input {
            return Ok(Self(Box::new(KeywordConstraint) as Box<dyn Any>));
        }
        if KeywordCreate == input {
            return Ok(Self(Box::new(KeywordCreate) as Box<dyn Any>));
        }
        if KeywordCross == input {
            return Ok(Self(Box::new(KeywordCross) as Box<dyn Any>));
        }
        if KeywordCurrent == input {
            return Ok(Self(Box::new(KeywordCurrent) as Box<dyn Any>));
        }
        if KeywordCurrent_date == input {
            return Ok(Self(Box::new(KeywordCurrent_date) as Box<dyn Any>));
        }
        if KeywordCurrent_time == input {
            return Ok(Self(Box::new(KeywordCurrent_time) as Box<dyn Any>));
        }
        if KeywordCurrent_timestamp == input {
            return Ok(Self(Box::new(KeywordCurrent_timestamp) as Box<dyn Any>));
        }
        if KeywordDatabase == input {
            return Ok(Self(Box::new(KeywordDatabase) as Box<dyn Any>));
        }
        if KeywordDefault == input {
            return Ok(Self(Box::new(KeywordDefault) as Box<dyn Any>));
        }
        if KeywordDeferrable == input {
            return Ok(Self(Box::new(KeywordDeferrable) as Box<dyn Any>));
        }
        if KeywordDeferred == input {
            return Ok(Self(Box::new(KeywordDeferred) as Box<dyn Any>));
        }
        if KeywordDelete == input {
            return Ok(Self(Box::new(KeywordDelete) as Box<dyn Any>));
        }
        if KeywordDesc == input {
            return Ok(Self(Box::new(KeywordDesc) as Box<dyn Any>));
        }
        if KeywordDetach == input {
            return Ok(Self(Box::new(KeywordDetach) as Box<dyn Any>));
        }
        if KeywordDistinct == input {
            return Ok(Self(Box::new(KeywordDistinct) as Box<dyn Any>));
        }
        if KeywordDo == input {
            return Ok(Self(Box::new(KeywordDo) as Box<dyn Any>));
        }
        if KeywordDrop == input {
            return Ok(Self(Box::new(KeywordDrop) as Box<dyn Any>));
        }
        if KeywordEach == input {
            return Ok(Self(Box::new(KeywordEach) as Box<dyn Any>));
        }
        if KeywordElse == input {
            return Ok(Self(Box::new(KeywordElse) as Box<dyn Any>));
        }
        if KeywordEnd == input {
            return Ok(Self(Box::new(KeywordEnd) as Box<dyn Any>));
        }
        if KeywordEscape == input {
            return Ok(Self(Box::new(KeywordEscape) as Box<dyn Any>));
        }
        if KeywordExcept == input {
            return Ok(Self(Box::new(KeywordExcept) as Box<dyn Any>));
        }
        if KeywordExclude == input {
            return Ok(Self(Box::new(KeywordExclude) as Box<dyn Any>));
        }
        if KeywordExclusive == input {
            return Ok(Self(Box::new(KeywordExclusive) as Box<dyn Any>));
        }
        if KeywordExists == input {
            return Ok(Self(Box::new(KeywordExists) as Box<dyn Any>));
        }
        if KeywordExplain == input {
            return Ok(Self(Box::new(KeywordExplain) as Box<dyn Any>));
        }
        if KeywordFail == input {
            return Ok(Self(Box::new(KeywordFail) as Box<dyn Any>));
        }
        if KeywordFilter == input {
            return Ok(Self(Box::new(KeywordFilter) as Box<dyn Any>));
        }
        if KeywordFirst == input {
            return Ok(Self(Box::new(KeywordFirst) as Box<dyn Any>));
        }
        if KeywordFollowing == input {
            return Ok(Self(Box::new(KeywordFollowing) as Box<dyn Any>));
        }
        if KeywordFor == input {
            return Ok(Self(Box::new(KeywordFor) as Box<dyn Any>));
        }
        if KeywordForeign == input {
            return Ok(Self(Box::new(KeywordForeign) as Box<dyn Any>));
        }
        if KeywordFrom == input {
            return Ok(Self(Box::new(KeywordFrom) as Box<dyn Any>));
        }
        if KeywordFull == input {
            return Ok(Self(Box::new(KeywordFull) as Box<dyn Any>));
        }
        if KeywordGenerated == input {
            return Ok(Self(Box::new(KeywordGenerated) as Box<dyn Any>));
        }
        if KeywordGlob == input {
            return Ok(Self(Box::new(KeywordGlob) as Box<dyn Any>));
        }
        if KeywordGroup == input {
            return Ok(Self(Box::new(KeywordGroup) as Box<dyn Any>));
        }
        if KeywordGroups == input {
            return Ok(Self(Box::new(KeywordGroups) as Box<dyn Any>));
        }
        if KeywordHaving == input {
            return Ok(Self(Box::new(KeywordHaving) as Box<dyn Any>));
        }
        if KeywordIf == input {
            return Ok(Self(Box::new(KeywordIf) as Box<dyn Any>));
        }
        if KeywordIgnore == input {
            return Ok(Self(Box::new(KeywordIgnore) as Box<dyn Any>));
        }
        if KeywordImmediate == input {
            return Ok(Self(Box::new(KeywordImmediate) as Box<dyn Any>));
        }
        if KeywordIn == input {
            return Ok(Self(Box::new(KeywordIn) as Box<dyn Any>));
        }
        if KeywordIndex == input {
            return Ok(Self(Box::new(KeywordIndex) as Box<dyn Any>));
        }
        if KeywordIndexed == input {
            return Ok(Self(Box::new(KeywordIndexed) as Box<dyn Any>));
        }
        if KeywordInitially == input {
            return Ok(Self(Box::new(KeywordInitially) as Box<dyn Any>));
        }
        if KeywordInner == input {
            return Ok(Self(Box::new(KeywordInner) as Box<dyn Any>));
        }
        if KeywordInsert == input {
            return Ok(Self(Box::new(KeywordInsert) as Box<dyn Any>));
        }
        if KeywordInstead == input {
            return Ok(Self(Box::new(KeywordInstead) as Box<dyn Any>));
        }
        if KeywordIntersect == input {
            return Ok(Self(Box::new(KeywordIntersect) as Box<dyn Any>));
        }
        if KeywordInto == input {
            return Ok(Self(Box::new(KeywordInto) as Box<dyn Any>));
        }
        if KeywordIs == input {
            return Ok(Self(Box::new(KeywordIs) as Box<dyn Any>));
        }
        if KeywordIsnull == input {
            return Ok(Self(Box::new(KeywordIsnull) as Box<dyn Any>));
        }
        if KeywordJoin == input {
            return Ok(Self(Box::new(KeywordJoin) as Box<dyn Any>));
        }
        if KeywordKey == input {
            return Ok(Self(Box::new(KeywordKey) as Box<dyn Any>));
        }
        if KeywordLast == input {
            return Ok(Self(Box::new(KeywordLast) as Box<dyn Any>));
        }
        if KeywordLeft == input {
            return Ok(Self(Box::new(KeywordLeft) as Box<dyn Any>));
        }
        if KeywordLike == input {
            return Ok(Self(Box::new(KeywordLike) as Box<dyn Any>));
        }
        if KeywordLimit == input {
            return Ok(Self(Box::new(KeywordLimit) as Box<dyn Any>));
        }
        if KeywordMatch == input {
            return Ok(Self(Box::new(KeywordMatch) as Box<dyn Any>));
        }
        if KeywordMaterialized == input {
            return Ok(Self(Box::new(KeywordMaterialized) as Box<dyn Any>));
        }
        if KeywordNatural == input {
            return Ok(Self(Box::new(KeywordNatural) as Box<dyn Any>));
        }
        if KeywordNo == input {
            return Ok(Self(Box::new(KeywordNo) as Box<dyn Any>));
        }
        if KeywordNot == input {
            return Ok(Self(Box::new(KeywordNot) as Box<dyn Any>));
        }
        if KeywordNothing == input {
            return Ok(Self(Box::new(KeywordNothing) as Box<dyn Any>));
        }
        if KeywordNotnull == input {
            return Ok(Self(Box::new(KeywordNotnull) as Box<dyn Any>));
        }
        if KeywordNull == input {
            return Ok(Self(Box::new(KeywordNull) as Box<dyn Any>));
        }
        if KeywordNulls == input {
            return Ok(Self(Box::new(KeywordNulls) as Box<dyn Any>));
        }
        if KeywordOf == input {
            return Ok(Self(Box::new(KeywordOf) as Box<dyn Any>));
        }
        if KeywordOffset == input {
            return Ok(Self(Box::new(KeywordOffset) as Box<dyn Any>));
        }
        if KeywordOn == input {
            return Ok(Self(Box::new(KeywordOn) as Box<dyn Any>));
        }
        if KeywordOr == input {
            return Ok(Self(Box::new(KeywordOr) as Box<dyn Any>));
        }
        if KeywordOrder == input {
            return Ok(Self(Box::new(KeywordOrder) as Box<dyn Any>));
        }
        if KeywordOthers == input {
            return Ok(Self(Box::new(KeywordOthers) as Box<dyn Any>));
        }
        if KeywordOuter == input {
            return Ok(Self(Box::new(KeywordOuter) as Box<dyn Any>));
        }
        if KeywordOver == input {
            return Ok(Self(Box::new(KeywordOver) as Box<dyn Any>));
        }
        if KeywordPartition == input {
            return Ok(Self(Box::new(KeywordPartition) as Box<dyn Any>));
        }
        if KeywordPlan == input {
            return Ok(Self(Box::new(KeywordPlan) as Box<dyn Any>));
        }
        if KeywordPragma == input {
            return Ok(Self(Box::new(KeywordPragma) as Box<dyn Any>));
        }
        if KeywordPreceding == input {
            return Ok(Self(Box::new(KeywordPreceding) as Box<dyn Any>));
        }
        if KeywordPrimary == input {
            return Ok(Self(Box::new(KeywordPrimary) as Box<dyn Any>));
        }
        if KeywordQuery == input {
            return Ok(Self(Box::new(KeywordQuery) as Box<dyn Any>));
        }
        if KeywordRaise == input {
            return Ok(Self(Box::new(KeywordRaise) as Box<dyn Any>));
        }
        if KeywordRange == input {
            return Ok(Self(Box::new(KeywordRange) as Box<dyn Any>));
        }
        if KeywordRecursive == input {
            return Ok(Self(Box::new(KeywordRecursive) as Box<dyn Any>));
        }
        if KeywordReferences == input {
            return Ok(Self(Box::new(KeywordReferences) as Box<dyn Any>));
        }
        if KeywordRegexp == input {
            return Ok(Self(Box::new(KeywordRegexp) as Box<dyn Any>));
        }
        if KeywordReindex == input {
            return Ok(Self(Box::new(KeywordReindex) as Box<dyn Any>));
        }
        if KeywordRelease == input {
            return Ok(Self(Box::new(KeywordRelease) as Box<dyn Any>));
        }
        if KeywordRename == input {
            return Ok(Self(Box::new(KeywordRename) as Box<dyn Any>));
        }
        if KeywordReplace == input {
            return Ok(Self(Box::new(KeywordReplace) as Box<dyn Any>));
        }
        if KeywordRestrict == input {
            return Ok(Self(Box::new(KeywordRestrict) as Box<dyn Any>));
        }
        if KeywordReturning == input {
            return Ok(Self(Box::new(KeywordReturning) as Box<dyn Any>));
        }
        if KeywordRight == input {
            return Ok(Self(Box::new(KeywordRight) as Box<dyn Any>));
        }
        if KeywordRollback == input {
            return Ok(Self(Box::new(KeywordRollback) as Box<dyn Any>));
        }
        if KeywordRow == input {
            return Ok(Self(Box::new(KeywordRow) as Box<dyn Any>));
        }
        if KeywordRows == input {
            return Ok(Self(Box::new(KeywordRows) as Box<dyn Any>));
        }
        if KeywordSavepoint == input {
            return Ok(Self(Box::new(KeywordSavepoint) as Box<dyn Any>));
        }
        if KeywordSelect == input {
            return Ok(Self(Box::new(KeywordSelect) as Box<dyn Any>));
        }
        if KeywordSet == input {
            return Ok(Self(Box::new(KeywordSet) as Box<dyn Any>));
        }
        if KeywordTable == input {
            return Ok(Self(Box::new(KeywordTable) as Box<dyn Any>));
        }
        if KeywordTemp == input {
            return Ok(Self(Box::new(KeywordTemp) as Box<dyn Any>));
        }
        if KeywordTemporary == input {
            return Ok(Self(Box::new(KeywordTemporary) as Box<dyn Any>));
        }
        if KeywordThen == input {
            return Ok(Self(Box::new(KeywordThen) as Box<dyn Any>));
        }
        if KeywordTies == input {
            return Ok(Self(Box::new(KeywordTies) as Box<dyn Any>));
        }
        if KeywordTo == input {
            return Ok(Self(Box::new(KeywordTo) as Box<dyn Any>));
        }
        if KeywordTransaction == input {
            return Ok(Self(Box::new(KeywordTransaction) as Box<dyn Any>));
        }
        if KeywordTrigger == input {
            return Ok(Self(Box::new(KeywordTrigger) as Box<dyn Any>));
        }
        if KeywordUnbounded == input {
            return Ok(Self(Box::new(KeywordUnbounded) as Box<dyn Any>));
        }
        if KeywordUnion == input {
            return Ok(Self(Box::new(KeywordUnion) as Box<dyn Any>));
        }
        if KeywordUnique == input {
            return Ok(Self(Box::new(KeywordUnique) as Box<dyn Any>));
        }
        if KeywordUpdate == input {
            return Ok(Self(Box::new(KeywordUpdate) as Box<dyn Any>));
        }
        if KeywordUsing == input {
            return Ok(Self(Box::new(KeywordUsing) as Box<dyn Any>));
        }
        if KeywordVacuum == input {
            return Ok(Self(Box::new(KeywordVacuum) as Box<dyn Any>));
        }
        if KeywordValues == input {
            return Ok(Self(Box::new(KeywordValues) as Box<dyn Any>));
        }
        if KeywordView == input {
            return Ok(Self(Box::new(KeywordView) as Box<dyn Any>));
        }
        if KeywordVirtual == input {
            return Ok(Self(Box::new(KeywordVirtual) as Box<dyn Any>));
        }
        if KeywordWhen == input {
            return Ok(Self(Box::new(KeywordWhen) as Box<dyn Any>));
        }
        if KeywordWhere == input {
            return Ok(Self(Box::new(KeywordWhere) as Box<dyn Any>));
        }
        if KeywordWindow == input {
            return Ok(Self(Box::new(KeywordWindow) as Box<dyn Any>));
        }
        if KeywordWith == input {
            return Ok(Self(Box::new(KeywordWith) as Box<dyn Any>));
        }
        if KeywordWithout == input {
            return Ok(Self(Box::new(KeywordWithout) as Box<dyn Any>));
        }

        Err(Sq3ParserError("Not a valid SQLite keyword.".into()))
    }
}
