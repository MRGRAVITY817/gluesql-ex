#![allow(non_snake_case)]

mod data {
    pub mod column_def;
    pub mod schema_index;
}

use data::{column_def::ColumnDef, schema_index::SchemaIndex};
use rustler::NifStruct;
use serde::{Deserialize, Serialize};

#[derive(NifStruct, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[module = "GlueSQL.RS.Schema"]
struct Schema {
    table_name: String,
    column_defs: Option<Vec<ColumnDef>>,
    indexes: Vec<SchemaIndex>,
    engine: Option<String>,
}

rustler::init!("Elixir.GlueSQL.RS", []);
