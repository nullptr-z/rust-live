use std::sync::Arc;

use anyhow::{anyhow, Result};
use polars::{
    lazy::dsl::{self, Expr},
    prelude::LiteralValue,
};
use sqlparser::ast::{
    self, Ident, Offset as SqlOffset, OrderByExpr, Select, SelectItem, SetExpr, Statement,
    TableWithJoins,
};

pub struct Offset(pub(crate) SqlOffset);
pub struct Source<'a>(pub(crate) &'a [TableWithJoins]);
pub struct SqlExpr(pub(crate) ast::Expr);
pub struct SelectOption<'a>(pub(crate) &'a [SelectItem]);
pub struct Order<'a>(pub(crate) &'a [OrderByExpr]);
pub struct Limit(pub(crate) ast::Expr);

pub struct SqlValue(pub ast::Value);
pub struct LogicOperator(pub ast::BinaryOperator);
pub struct IdentValue<'a>(pub(crate) &'a Ident);

#[derive(Debug, Default)]
pub struct Sql {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: String,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<u32>,
}

impl TryFrom<&Statement> for Sql {
    type Error = anyhow::Error;

    fn try_from(sql: &Statement) -> Result<Self> {
        match sql {
            Statement::Query(q) => {
                let offset = q.offset.as_ref();
                let limit = q.limit.as_ref();
                let orders = &q.order_by;
                let Select {
                    from,
                    projection,
                    selection,
                    group_by: _,
                    ..
                } = match &q.body.as_ref() {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("Only support Select Query at the moment")),
                };

                let source = Source(from).try_into()?;
                println!("【 source 】==> {:?}", source);

                println!("【 selection 】==> {:?}", selection);
                let condition: Option<Expr> = selection.as_ref().map_or(None, |expr| {
                    Some(SqlExpr(expr.to_owned()).try_into().unwrap())
                });
                println!("【 condition 】==> {:?}", condition);

                let select_option: Vec<Expr> = SelectOption(projection).try_into()?;
                println!("【 select_option 】==> {:?}", select_option);

                let order_by = Order(orders).try_into()?;
                println!("【 order_by 】==> {:?}", order_by);

                let offset = offset.map(|ofs| Offset(ofs.clone()).into());
                println!("【 offset 】==> {:?}", offset);

                let limit = limit.map(|lmt| Limit(lmt.clone()).into());
                println!("【 limit 】==> {:?}", limit);

                let sql = Sql {
                    source,
                    condition,
                    selection: select_option,
                    offset,
                    limit,
                    order_by,
                };

                Ok(sql)
            }
            _ => return Err(anyhow!("Only support Query at the moment")),
        }
    }
}

impl TryFrom<SqlExpr> for Expr {
    type Error = anyhow::Error;

    fn try_from(from_val: SqlExpr) -> std::result::Result<Self, Self::Error> {
        match from_val.0 {
            // ast::Expr::Identifier(ident) => Ok(Self::Column(Arc::from(ident.value))),
            ast::Expr::Identifier(ident) => Ok(dsl::col(&ident.to_string())),
            ast::Expr::BinaryOp { left, op, right } => Ok(Expr::BinaryExpr {
                left: Box::new(SqlExpr(left.as_ref().clone()).try_into()?),
                op: LogicOperator(op).try_into()?,
                right: Box::new(SqlExpr(right.as_ref().clone()).try_into()?),
            }),
            ast::Expr::Value(val) => Ok(Expr::Literal(SqlValue(val).try_into()?)),
            expr => Err(anyhow!("ast::Expr Not supported yet: {expr:?}")),
        }
    }
}

// ---

impl TryFrom<Source<'_>> for String {
    type Error = anyhow::Error;

    fn try_from(from_val: Source) -> Result<Self, Self::Error> {
        if from_val.0.len() != 1 {
            return Err(anyhow!("We only support single data source at the moment"));
        }

        match &from_val.0.first().unwrap().relation {
            sqlparser::ast::TableFactor::Table { name, .. } => {
                if name.0.len() != 1 {
                    return Err(anyhow!("only support single source name at the moment"));
                }

                Ok(name.0.first().unwrap().value.clone())
            }
            _ => Err(anyhow!("only support ast::TableFactor::Table")),
        }
    }
}

impl TryFrom<SelectOption<'_>> for Vec<Expr> {
    type Error = anyhow::Error;

    fn try_from(from_val: SelectOption) -> std::result::Result<Self, Self::Error> {
        let mut option: Vec<Expr> = Vec::with_capacity(5);
        for item in from_val.0 {
            let opt = match item {
                SelectItem::UnnamedExpr(expr) => SqlExpr(expr.clone()).try_into()?,
                SelectItem::ExprWithAlias { expr, alias } => Expr::Alias(
                    Box::new(SqlExpr(expr.clone()).try_into()?),
                    Arc::from(String::from(IdentValue(alias))),
                ),
                SelectItem::QualifiedWildcard(name, _option) => {
                    dsl::col(&name.to_string()).try_into()?
                }
                SelectItem::Wildcard(_option) => dsl::col("*"),
            };

            option.push(opt);
        }

        Ok(option)
    }
}

impl TryFrom<Order<'_>> for Vec<(String, bool)> {
    type Error = anyhow::Error;

    fn try_from(from_val: Order) -> std::result::Result<Self, Self::Error> {
        let mut orders: Vec<(String, bool)> = vec![];
        for item in from_val.0 {
            let column = match &item.expr {
                ast::Expr::Identifier(ident) => ident.to_string(),
                expr => {
                    return Err(anyhow!(
                        "We only support identifier for order by, got {}",
                        expr
                    ));
                }
            };
            let asc = item.asc.map_or(false, |asc| asc);

            orders.push((column, asc))
        }

        Ok(orders)
    }
}

impl From<Offset> for i64 {
    fn from(from_val: Offset) -> Self {
        match &from_val.0.value {
            ast::Expr::Value(v_type) => match v_type {
                sqlparser::ast::Value::Number(num, _b) => num.parse::<i64>().unwrap(),
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl From<Limit> for u32 {
    fn from(from_val: Limit) -> Self {
        match &from_val.0 {
            ast::Expr::Value(v_type) => match v_type {
                sqlparser::ast::Value::Number(num, _b) => num.parse::<u32>().unwrap(),
                _ => 0,
            },
            _ => 0,
        }
    }
}
/// --------------一些额外的--------------
impl TryFrom<SqlValue> for LiteralValue {
    type Error = anyhow::Error;

    fn try_from(sql_value: SqlValue) -> std::result::Result<Self, Self::Error> {
        match sql_value.0 {
            ast::Value::Number(v, _) => Ok(LiteralValue::Float64(v.parse().unwrap())),
            ast::Value::Boolean(v) => Ok(LiteralValue::Boolean(v)),
            ast::Value::Null => Ok(LiteralValue::Null),
            value => Err(anyhow!("SqlValue not support yet LiteralValue: {value:?}")),
        }
    }
}

/// 把 SqlParser 的 BinaryOperator 转换成 DataFrame 的 Operator
impl TryFrom<LogicOperator> for dsl::Operator {
    type Error = anyhow::Error;

    fn try_from(op: LogicOperator) -> Result<Self, Self::Error> {
        match op.0 {
            ast::BinaryOperator::Plus => Ok(Self::Plus),
            ast::BinaryOperator::Minus => Ok(Self::Minus),
            ast::BinaryOperator::Multiply => Ok(Self::Multiply),
            ast::BinaryOperator::Divide => Ok(Self::Divide),
            ast::BinaryOperator::Modulo => Ok(Self::Modulus),
            ast::BinaryOperator::Gt => Ok(Self::Gt),
            ast::BinaryOperator::Lt => Ok(Self::Lt),
            ast::BinaryOperator::GtEq => Ok(Self::GtEq),
            ast::BinaryOperator::LtEq => Ok(Self::LtEq),
            ast::BinaryOperator::Eq => Ok(Self::Eq),
            ast::BinaryOperator::NotEq => Ok(Self::NotEq),
            ast::BinaryOperator::And => Ok(Self::And),
            ast::BinaryOperator::Or => Ok(Self::Or),
            v => Err(anyhow!("Operator not supported: {}", v)),
        }
    }
}

impl From<IdentValue<'_>> for String {
    fn from(ident: IdentValue) -> Self {
        ident.0.value.to_owned()
    }
}
