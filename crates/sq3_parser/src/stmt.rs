mod alter_table;
mod analyze;
mod attach;
mod begin;
mod commit;
mod create_index;
mod create_table;
mod create_trigger;
mod create_view;
mod create_virtual_table;
mod delete;
mod delete_limited;
mod detach;
mod drop_index;
mod drop_table;
mod drop_trigger;
mod drop_view;
mod insert;
mod pragma;
mod reindex;
mod release;
mod rollback;
mod savepoint;
mod select;
mod update;
mod update_limited;
mod vacuum;

use std::fmt::Debug;

use crate::explain::ExplainStmt;

pub(crate) use self::{
    delete::{DeleteParser, DeleteStmt},
    insert::{InsertParser, InsertStmt},
    select::{SelectParser, SelectStmt},
    update::{UpdateParser, UpdateStmt},
};

pub trait SqliteStmt: Debug {}

impl SqliteStmt for DeleteStmt<'_> {}
impl SqliteStmt for InsertStmt<'_> {}
impl SqliteStmt for SelectStmt<'_> {}
impl SqliteStmt for UpdateStmt<'_> {}
impl SqliteStmt for ExplainStmt<'_> {}
