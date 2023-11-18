use gluesql_core::{
    ast::{ColumnUniqueOption, Expr},
    prelude::DataType,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    /// `DEFAULT <restricted-expr>`
    pub default: Option<Expr>,
    /// `{ PRIMARY KEY | UNIQUE }`
    pub unique: Option<ColumnUniqueOption>,
}

impl Display for ColumnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap_or(self.name.to_string());
        f.write_str(&serialized)
    }
}

impl rustler::Encoder for ColumnDef {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        format!("{self}").encode(env)
    }
}

impl<'a> rustler::Decoder<'a> for ColumnDef {
    fn decode(term: rustler::Term<'a>) -> rustler::NifResult<Self> {
        term.decode()
    }
}
