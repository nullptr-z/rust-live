use anyhow::Result;
use clap::Parser;
use sql_query_all::query;
use std::path::PathBuf;

/// Command line tool to validate a protocol
#[derive(Parser, Debug)]
struct ValidateProtocol {
    #[arg(short = 's', long)]
    sql: String,
    #[arg(long)]
    file: Option<PathBuf>,
    #[arg(long)]
    http: Option<String>,
    #[arg(long)]
    https: Option<String>,
}

static MARK: &str = "@source";

#[tokio::main]
async fn main() -> Result<()> {
    let args = ValidateProtocol::parse();

    let source_it = &args.sql.contains(MARK);

    let protocol = if *source_it {
        match (args.file, args.http, args.https) {
            (Some(file_path), None, None) => {
                let file = file_path.to_str().unwrap();
                if file.starts_with("file://") {
                    file.to_string()
                } else {
                    format!("file://{}", file)
                }
            }
            (None, Some(http), None) => {
                if http.starts_with("http://") {
                    http
                } else {
                    format!("http://{}", http)
                }
            }
            (None, None, Some(https)) => {
                if https.starts_with("https://") {
                    https
                } else {
                    format!("https://{}", https)
                }
            }
            _ => panic!(
            "Invalid input! Please provide one of the following options: --file, --http, --https"
        ),
        }
    } else {
        String::default()
    };

    let sql = args.sql.replace(MARK, &protocol);

    println!("【 sql 】==> {:?}", sql);
    let df = query(&sql).await?;
    println!("{:?}", df);

    Ok(())
}
