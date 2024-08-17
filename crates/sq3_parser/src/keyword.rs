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
            let binding = Box::new(KeywordAbort);
            return Ok(Self(binding));
        }
        if KeywordAction == input {
            let binding = Box::new(KeywordAction);
            return Ok(Self(binding));
        }
        if KeywordAdd == input {
            let binding = Box::new(KeywordAdd);
            return Ok(Self(binding));
        }
        if KeywordAfter == input {
            let binding = Box::new(KeywordAfter);
            return Ok(Self(binding));
        }
        if KeywordAll == input {
            let binding = Box::new(KeywordAll);
            return Ok(Self(binding));
        }
        if KeywordAlter == input {
            let binding = Box::new(KeywordAlter);
            return Ok(Self(binding));
        }
        if KeywordAlways == input {
            let binding = Box::new(KeywordAlways);
            return Ok(Self(binding));
        }
        if KeywordAnalyze == input {
            let binding = Box::new(KeywordAnalyze);
            return Ok(Self(binding));
        }
        if KeywordAnd == input {
            let binding = Box::new(KeywordAnd);
            return Ok(Self(binding));
        }
        if KeywordAs == input {
            let binding = Box::new(KeywordAs);
            return Ok(Self(binding));
        }
        if KeywordAsc == input {
            let binding = Box::new(KeywordAsc);
            return Ok(Self(binding));
        }
        if KeywordAttach == input {
            let binding = Box::new(KeywordAttach);
            return Ok(Self(binding));
        }
        if KeywordAutoincrement == input {
            let binding = Box::new(KeywordAutoincrement);
            return Ok(Self(binding));
        }
        if KeywordBefore == input {
            let binding = Box::new(KeywordBefore);
            return Ok(Self(binding));
        }
        if KeywordBegin == input {
            let binding = Box::new(KeywordBegin);
            return Ok(Self(binding));
        }
        if KeywordBetween == input {
            let binding = Box::new(KeywordBetween);
            return Ok(Self(binding));
        }
        if KeywordBy == input {
            let binding = Box::new(KeywordBy);
            return Ok(Self(binding));
        }
        if KeywordCascade == input {
            let binding = Box::new(KeywordCascade);
            return Ok(Self(binding));
        }
        if KeywordCase == input {
            let binding = Box::new(KeywordCase);
            return Ok(Self(binding));
        }
        if KeywordCast == input {
            let binding = Box::new(KeywordCast);
            return Ok(Self(binding));
        }
        if KeywordCheck == input {
            let binding = Box::new(KeywordCheck);
            return Ok(Self(binding));
        }
        if KeywordCollate == input {
            let binding = Box::new(KeywordCollate);
            return Ok(Self(binding));
        }
        if KeywordColumn == input {
            let binding = Box::new(KeywordColumn);
            return Ok(Self(binding));
        }
        if KeywordCommit == input {
            let binding = Box::new(KeywordCommit);
            return Ok(Self(binding));
        }
        if KeywordConflict == input {
            let binding = Box::new(KeywordConflict);
            return Ok(Self(binding));
        }
        if KeywordConstraint == input {
            let binding = Box::new(KeywordConstraint);
            return Ok(Self(binding));
        }
        if KeywordCreate == input {
            let binding = Box::new(KeywordCreate);
            return Ok(Self(binding));
        }
        if KeywordCross == input {
            let binding = Box::new(KeywordCross);
            return Ok(Self(binding));
        }
        if KeywordCurrent == input {
            let binding = Box::new(KeywordCurrent);
            return Ok(Self(binding));
        }
        if KeywordCurrent_date == input {
            let binding = Box::new(KeywordCurrent_date);
            return Ok(Self(binding));
        }
        if KeywordCurrent_time == input {
            let binding = Box::new(KeywordCurrent_time);
            return Ok(Self(binding));
        }
        if KeywordCurrent_timestamp == input {
            let binding = Box::new(KeywordCurrent_timestamp);
            return Ok(Self(binding));
        }
        if KeywordDatabase == input {
            let binding = Box::new(KeywordDatabase);
            return Ok(Self(binding));
        }
        if KeywordDefault == input {
            let binding = Box::new(KeywordDefault);
            return Ok(Self(binding));
        }
        if KeywordDeferrable == input {
            let binding = Box::new(KeywordDeferrable);
            return Ok(Self(binding));
        }
        if KeywordDeferred == input {
            let binding = Box::new(KeywordDeferred);
            return Ok(Self(binding));
        }
        if KeywordDelete == input {
            let binding = Box::new(KeywordDelete);
            return Ok(Self(binding));
        }
        if KeywordDesc == input {
            let binding = Box::new(KeywordDesc);
            return Ok(Self(binding));
        }
        if KeywordDetach == input {
            let binding = Box::new(KeywordDetach);
            return Ok(Self(binding));
        }
        if KeywordDistinct == input {
            let binding = Box::new(KeywordDistinct);
            return Ok(Self(binding));
        }
        if KeywordDo == input {
            let binding = Box::new(KeywordDo);
            return Ok(Self(binding));
        }
        if KeywordDrop == input {
            let binding = Box::new(KeywordDrop);
            return Ok(Self(binding));
        }
        if KeywordEach == input {
            let binding = Box::new(KeywordEach);
            return Ok(Self(binding));
        }
        if KeywordElse == input {
            let binding = Box::new(KeywordElse);
            return Ok(Self(binding));
        }
        if KeywordEnd == input {
            let binding = Box::new(KeywordEnd);
            return Ok(Self(binding));
        }
        if KeywordEscape == input {
            let binding = Box::new(KeywordEscape);
            return Ok(Self(binding));
        }
        if KeywordExcept == input {
            let binding = Box::new(KeywordExcept);
            return Ok(Self(binding));
        }
        if KeywordExclude == input {
            let binding = Box::new(KeywordExclude);
            return Ok(Self(binding));
        }
        if KeywordExclusive == input {
            let binding = Box::new(KeywordExclusive);
            return Ok(Self(binding));
        }
        if KeywordExists == input {
            let binding = Box::new(KeywordExists);
            return Ok(Self(binding));
        }
        if KeywordExplain == input {
            let binding = Box::new(KeywordExplain);
            return Ok(Self(binding));
        }
        if KeywordFail == input {
            let binding = Box::new(KeywordFail);
            return Ok(Self(binding));
        }
        if KeywordFilter == input {
            let binding = Box::new(KeywordFilter);
            return Ok(Self(binding));
        }
        if KeywordFirst == input {
            let binding = Box::new(KeywordFirst);
            return Ok(Self(binding));
        }
        if KeywordFollowing == input {
            let binding = Box::new(KeywordFollowing);
            return Ok(Self(binding));
        }
        if KeywordFor == input {
            let binding = Box::new(KeywordFor);
            return Ok(Self(binding));
        }
        if KeywordForeign == input {
            let binding = Box::new(KeywordForeign);
            return Ok(Self(binding));
        }
        if KeywordFrom == input {
            let binding = Box::new(KeywordFrom);
            return Ok(Self(binding));
        }
        if KeywordFull == input {
            let binding = Box::new(KeywordFull);
            return Ok(Self(binding));
        }
        if KeywordGenerated == input {
            let binding = Box::new(KeywordGenerated);
            return Ok(Self(binding));
        }
        if KeywordGlob == input {
            let binding = Box::new(KeywordGlob);
            return Ok(Self(binding));
        }
        if KeywordGroup == input {
            let binding = Box::new(KeywordGroup);
            return Ok(Self(binding));
        }
        if KeywordGroups == input {
            let binding = Box::new(KeywordGroups);
            return Ok(Self(binding));
        }
        if KeywordHaving == input {
            let binding = Box::new(KeywordHaving);
            return Ok(Self(binding));
        }
        if KeywordIf == input {
            let binding = Box::new(KeywordIf);
            return Ok(Self(binding));
        }
        if KeywordIgnore == input {
            let binding = Box::new(KeywordIgnore);
            return Ok(Self(binding));
        }
        if KeywordImmediate == input {
            let binding = Box::new(KeywordImmediate);
            return Ok(Self(binding));
        }
        if KeywordIn == input {
            let binding = Box::new(KeywordIn);
            return Ok(Self(binding));
        }
        if KeywordIndex == input {
            let binding = Box::new(KeywordIndex);
            return Ok(Self(binding));
        }
        if KeywordIndexed == input {
            let binding = Box::new(KeywordIndexed);
            return Ok(Self(binding));
        }
        if KeywordInitially == input {
            let binding = Box::new(KeywordInitially);
            return Ok(Self(binding));
        }
        if KeywordInner == input {
            let binding = Box::new(KeywordInner);
            return Ok(Self(binding));
        }
        if KeywordInsert == input {
            let binding = Box::new(KeywordInsert);
            return Ok(Self(binding));
        }
        if KeywordInstead == input {
            let binding = Box::new(KeywordInstead);
            return Ok(Self(binding));
        }
        if KeywordIntersect == input {
            let binding = Box::new(KeywordIntersect);
            return Ok(Self(binding));
        }
        if KeywordInto == input {
            let binding = Box::new(KeywordInto);
            return Ok(Self(binding));
        }
        if KeywordIs == input {
            let binding = Box::new(KeywordIs);
            return Ok(Self(binding));
        }
        if KeywordIsnull == input {
            let binding = Box::new(KeywordIsnull);
            return Ok(Self(binding));
        }
        if KeywordJoin == input {
            let binding = Box::new(KeywordJoin);
            return Ok(Self(binding));
        }
        if KeywordKey == input {
            let binding = Box::new(KeywordKey);
            return Ok(Self(binding));
        }
        if KeywordLast == input {
            let binding = Box::new(KeywordLast);
            return Ok(Self(binding));
        }
        if KeywordLeft == input {
            let binding = Box::new(KeywordLeft);
            return Ok(Self(binding));
        }
        if KeywordLike == input {
            let binding = Box::new(KeywordLike);
            return Ok(Self(binding));
        }
        if KeywordLimit == input {
            let binding = Box::new(KeywordLimit);
            return Ok(Self(binding));
        }
        if KeywordMatch == input {
            let binding = Box::new(KeywordMatch);
            return Ok(Self(binding));
        }
        if KeywordMaterialized == input {
            let binding = Box::new(KeywordMaterialized);
            return Ok(Self(binding));
        }
        if KeywordNatural == input {
            let binding = Box::new(KeywordNatural);
            return Ok(Self(binding));
        }
        if KeywordNo == input {
            let binding = Box::new(KeywordNo);
            return Ok(Self(binding));
        }
        if KeywordNot == input {
            let binding = Box::new(KeywordNot);
            return Ok(Self(binding));
        }
        if KeywordNothing == input {
            let binding = Box::new(KeywordNothing);
            return Ok(Self(binding));
        }
        if KeywordNotnull == input {
            let binding = Box::new(KeywordNotnull);
            return Ok(Self(binding));
        }
        if KeywordNull == input {
            let binding = Box::new(KeywordNull);
            return Ok(Self(binding));
        }
        if KeywordNulls == input {
            let binding = Box::new(KeywordNulls);
            return Ok(Self(binding));
        }
        if KeywordOf == input {
            let binding = Box::new(KeywordOf);
            return Ok(Self(binding));
        }
        if KeywordOffset == input {
            let binding = Box::new(KeywordOffset);
            return Ok(Self(binding));
        }
        if KeywordOn == input {
            let binding = Box::new(KeywordOn);
            return Ok(Self(binding));
        }
        if KeywordOr == input {
            let binding = Box::new(KeywordOr);
            return Ok(Self(binding));
        }
        if KeywordOrder == input {
            let binding = Box::new(KeywordOrder);
            return Ok(Self(binding));
        }
        if KeywordOthers == input {
            let binding = Box::new(KeywordOthers);
            return Ok(Self(binding));
        }
        if KeywordOuter == input {
            let binding = Box::new(KeywordOuter);
            return Ok(Self(binding));
        }
        if KeywordOver == input {
            let binding = Box::new(KeywordOver);
            return Ok(Self(binding));
        }
        if KeywordPartition == input {
            let binding = Box::new(KeywordPartition);
            return Ok(Self(binding));
        }
        if KeywordPlan == input {
            let binding = Box::new(KeywordPlan);
            return Ok(Self(binding));
        }
        if KeywordPragma == input {
            let binding = Box::new(KeywordPragma);
            return Ok(Self(binding));
        }
        if KeywordPreceding == input {
            let binding = Box::new(KeywordPreceding);
            return Ok(Self(binding));
        }
        if KeywordPrimary == input {
            let binding = Box::new(KeywordPrimary);
            return Ok(Self(binding));
        }
        if KeywordQuery == input {
            let binding = Box::new(KeywordQuery);
            return Ok(Self(binding));
        }
        if KeywordRaise == input {
            let binding = Box::new(KeywordRaise);
            return Ok(Self(binding));
        }
        if KeywordRange == input {
            let binding = Box::new(KeywordRange);
            return Ok(Self(binding));
        }
        if KeywordRecursive == input {
            let binding = Box::new(KeywordRecursive);
            return Ok(Self(binding));
        }
        if KeywordReferences == input {
            let binding = Box::new(KeywordReferences);
            return Ok(Self(binding));
        }
        if KeywordRegexp == input {
            let binding = Box::new(KeywordRegexp);
            return Ok(Self(binding));
        }
        if KeywordReindex == input {
            let binding = Box::new(KeywordReindex);
            return Ok(Self(binding));
        }
        if KeywordRelease == input {
            let binding = Box::new(KeywordRelease);
            return Ok(Self(binding));
        }
        if KeywordRename == input {
            let binding = Box::new(KeywordRename);
            return Ok(Self(binding));
        }
        if KeywordReplace == input {
            let binding = Box::new(KeywordReplace);
            return Ok(Self(binding));
        }
        if KeywordRestrict == input {
            let binding = Box::new(KeywordRestrict);
            return Ok(Self(binding));
        }
        if KeywordReturning == input {
            let binding = Box::new(KeywordReturning);
            return Ok(Self(binding));
        }
        if KeywordRight == input {
            let binding = Box::new(KeywordRight);
            return Ok(Self(binding));
        }
        if KeywordRollback == input {
            let binding = Box::new(KeywordRollback);
            return Ok(Self(binding));
        }
        if KeywordRow == input {
            let binding = Box::new(KeywordRow);
            return Ok(Self(binding));
        }
        if KeywordRows == input {
            let binding = Box::new(KeywordRows);
            return Ok(Self(binding));
        }
        if KeywordSavepoint == input {
            let binding = Box::new(KeywordSavepoint);
            return Ok(Self(binding));
        }
        if KeywordSelect == input {
            let binding = Box::new(KeywordSelect);
            return Ok(Self(binding));
        }
        if KeywordSet == input {
            let binding = Box::new(KeywordSet);
            return Ok(Self(binding));
        }
        if KeywordTable == input {
            let binding = Box::new(KeywordTable);
            return Ok(Self(binding));
        }
        if KeywordTemp == input {
            let binding = Box::new(KeywordTemp);
            return Ok(Self(binding));
        }
        if KeywordTemporary == input {
            let binding = Box::new(KeywordTemporary);
            return Ok(Self(binding));
        }
        if KeywordThen == input {
            let binding = Box::new(KeywordThen);
            return Ok(Self(binding));
        }
        if KeywordTies == input {
            let binding = Box::new(KeywordTies);
            return Ok(Self(binding));
        }
        if KeywordTo == input {
            let binding = Box::new(KeywordTo);
            return Ok(Self(binding));
        }
        if KeywordTransaction == input {
            let binding = Box::new(KeywordTransaction);
            return Ok(Self(binding));
        }
        if KeywordTrigger == input {
            let binding = Box::new(KeywordTrigger);
            return Ok(Self(binding));
        }
        if KeywordUnbounded == input {
            let binding = Box::new(KeywordUnbounded);
            return Ok(Self(binding));
        }
        if KeywordUnion == input {
            let binding = Box::new(KeywordUnion);
            return Ok(Self(binding));
        }
        if KeywordUnique == input {
            let binding = Box::new(KeywordUnique);
            return Ok(Self(binding));
        }
        if KeywordUpdate == input {
            let binding = Box::new(KeywordUpdate);
            return Ok(Self(binding));
        }
        if KeywordUsing == input {
            let binding = Box::new(KeywordUsing);
            return Ok(Self(binding));
        }
        if KeywordVacuum == input {
            let binding = Box::new(KeywordVacuum);
            return Ok(Self(binding));
        }
        if KeywordValues == input {
            let binding = Box::new(KeywordValues);
            return Ok(Self(binding));
        }
        if KeywordView == input {
            let binding = Box::new(KeywordView);
            return Ok(Self(binding));
        }
        if KeywordVirtual == input {
            let binding = Box::new(KeywordVirtual);
            return Ok(Self(binding));
        }
        if KeywordWhen == input {
            let binding = Box::new(KeywordWhen);
            return Ok(Self(binding));
        }
        if KeywordWhere == input {
            let binding = Box::new(KeywordWhere);
            return Ok(Self(binding));
        }
        if KeywordWindow == input {
            let binding = Box::new(KeywordWindow);
            return Ok(Self(binding));
        }
        if KeywordWith == input {
            let binding = Box::new(KeywordWith);
            return Ok(Self(binding));
        }
        if KeywordWithout == input {
            let binding = Box::new(KeywordWithout);
            return Ok(Self(binding));
        }

        Err(Sq3ParserError("Not a valid SQLite keyword.".into()))
    }
}
