use std::io::Cursor;

use anyhow::Result;
use polars::prelude::{CsvReader, SerReader};

use crate::DataSet;

pub trait Load {
    type Error;

    fn load(self) -> Result<DataSet, Self::Error>;
}

#[derive(Debug)]
pub enum Loader {
    Csv(CsvLoader),
}

#[derive(Default, Debug)]
pub struct CsvLoader(pub(crate) String);

impl Loader {
    pub fn load(self) -> Result<DataSet> {
        match self {
            Loader::Csv(csv) => csv.load(),
        }
    }
}

impl Load for CsvLoader {
    type Error = anyhow::Error;

    fn load(self) -> Result<DataSet, Self::Error> {
        let df = CsvReader::new(Cursor::new(self.0))
            // 自动推断出，各列数据应该使用哪种数据类型；使用前`max_records`行数据推断
            .infer_schema(Some(16))
            .finish()?;

        Ok(DataSet(df))
    }
}

pub fn detect_content(data: String) -> Loader {
    // todo: validate data content
    Loader::Csv(CsvLoader(data))
}
