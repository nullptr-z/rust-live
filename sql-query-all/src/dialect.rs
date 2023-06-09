use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct SqlDialect;

impl Dialect for SqlDialect {
    /// 重载 SQL 解析器判断标识符的方法
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || ch == '_'
    }

    /// 重载 SQL 解析器判断标识符的方法
    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            || ('0'..='9').contains(&ch)
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    sql
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;
    #[test]
    fn it_works() {
        let parses = Parser::parse_sql(&SqlDialect::default(), &example_sql());
        assert!(parses.is_ok());
    }
}
