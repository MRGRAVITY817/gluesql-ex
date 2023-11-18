mod data {
    pub mod schema_index;
}

use data::schema_index::SchemaIndex;
use gluesql_core::ast::ColumnDef;
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

rustler::init!("Elixir.GlueSQL.RS", [Schema]);
