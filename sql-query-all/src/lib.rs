mod convert;
mod dialect;
mod fetcher;
mod loader;

use anyhow::{anyhow, Result};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::ops::{Deref, DerefMut};
use tracing::info;

use crate::{convert::Sql, dialect::SqlDialect, fetcher::retrieve_data, loader::detect_content};

#[derive(Debug)]
pub struct DataSet(DataFrame);

impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    // DataSet convert to string format CSV
    pub fn to_csv(&mut self) -> Result<String> {
        let mut buf = Vec::new();
        let mut writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;

        Ok(String::from_utf8(buf)?)
    }
}

// filter from
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&SqlDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("ast length greater than 1"));
    }

    let sql = &ast[0];
    let Sql {
        selection,
        condition,
        source,
        order_by,
        offset,
        limit,
    } = sql.try_into()?;

    info!("retrieving data form source: {source}");

    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by.into_iter().fold(filtered, |acc, (col, desc)| {
        let mut opt = SortOptions::default();
        opt.descending = desc;
        acc.sort(&col, opt)
    });

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(u32::MAX));
    };

    Ok(DataSet(filtered.select(selection).collect()?))
}
