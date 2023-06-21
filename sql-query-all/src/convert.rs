use anyhow::Result;
use sqlparser::ast::{Expr, Statement};

pub struct Sql<'a> {
    selection: Vec<Expr>,
    condition: Option<Expr>,
    source: &'a str,
    order_by: Vec<(String, bool)>,
    offset: Option<i64>,
    limit: Option<u32>,
}

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self> {
        match sql {
            Statement::Query(query) => todo!(),
            _ => todo!(),
        }
    }
}
