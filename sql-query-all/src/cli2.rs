use anyhow::{Context, Result};
use clap::Parser;
use sql_query_all::query;
use std::path::PathBuf;

/// Command line tool to validate a protocol
#[derive(Parser, Debug)]
struct ValidateProtocol {
    #[arg(short, long)]
    sql: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = ValidateProtocol::parse();

    let sql_path = args.sql.context("Failed to get the SQL path")?;
    let sql = sql_path.to_str()?;

    let df = query(sql).await?;
    println!("{:?}", df);

    Ok(())
}
