use anyhow::{anyhow, Result};
use polars::lazy::dsl::Expr;
use sqlparser::ast::{Offset as SqlOffset, Select, SetExpr, Statement};

use crate::convert;

pub struct Sql {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: String,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<u32>,
}

pub struct Offset(pub(crate) SqlOffset);

impl From<Offset> for i64 {
    fn from(ofs: Offset) -> Self {
        match &ofs.0.value {
            sqlparser::ast::Expr::Value(VType) => match VType {
                sqlparser::ast::Value::Number(num, b) => num.parse::<i64>().unwrap(),
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl TryFrom<&Statement> for Sql {
    type Error = anyhow::Error;

    fn try_from(sql: &Statement) -> Result<Self> {
        match sql {
            Statement::Query(q) => {
                let Select {
                    distinct,
                    top,
                    projection,
                    into,
                    from,
                    lateral_views,
                    selection,
                    group_by: _,
                    cluster_by,
                    distribute_by,
                    sort_by,
                    having,
                    named_window,
                    qualify,
                } = match q.body.as_ref() {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("Only support Select Query at the moment")),
                };

                println!("【 from 】==> {:?}", from);
                let offset: i64 = convert::Offset(q.offset.clone().unwrap()).into();

                todo!()
            }
            _ => return Err(anyhow!("Only support Query at the moment")),
        }
    }
}
