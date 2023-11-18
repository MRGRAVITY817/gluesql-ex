use gluesql_core::{ast::Expr, chrono::NaiveDateTime, data::SchemaIndexOrd};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaIndex {
    pub name: String,
    pub expr: Expr,
    pub order: SchemaIndexOrd,
    pub created: NaiveDateTime,
}

impl Display for SchemaIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = serde_json::to_string(&self).unwrap_or(self.name);
        f.write_str(&serialized)
    }
}

impl rustler::Encoder for SchemaIndex {
    fn encode<'a>(&self, env: rustler::Env<'a>) -> rustler::Term<'a> {
        format!("{self}").encode(env)
    }
}
