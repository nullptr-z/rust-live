use anyhow::Result;
use sql_query_all::query;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    // 获取本地的文件
    let file_path = "owid-covid-latest.csv";

    // 使用 sql 从 URL 里获取数据
    let sql = format!(
        r#"SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM "{file_path}" where new_deaths >= 50 ORDER BY new_cases DESC"#
    );
    println!("【 sql 】==> {:?}", sql);
    let df1 = query(sql).await?;
    println!("{:?}", df1);

    Ok(())
}
