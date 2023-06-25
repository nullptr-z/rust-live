use clap::Parser;
use std::path::PathBuf;

/// Command line tool to validate a protocol
#[derive(Parser, Debug)]
struct ValidateProtocol {
    #[arg(long)]
    file: Option<PathBuf>,
    #[arg(long)]
    http: Option<String>,
    #[arg(long)]
    https: Option<String>,
}

fn main() {
    let args = ValidateProtocol::parse();

    let source = match (args.file, args.http, args.https) {
        (Some(file), None, None) => {
            let file = file.to_string_lossy().to_string();
            if file.starts_with("file://") {
                file
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
        _ => {
            panic!("Invalid input! Please provide one of the following options: --file, --http, --https");
        }
    };
}
