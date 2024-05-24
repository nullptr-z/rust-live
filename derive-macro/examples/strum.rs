use anyhow::Result;
use strum::{
    EnumCount, EnumDiscriminants, EnumIs, EnumIter, EnumMessage, EnumString, IntoEnumIterator,
    IntoStaticStr, VariantNames,
};

#[derive(
    Debug,
    EnumString,
    EnumCount,
    EnumIter,
    EnumDiscriminants,
    EnumIs,
    EnumMessage,
    VariantNames,
    IntoStaticStr,
)]
enum MyEnum {
    A,
    B,
    C,
    D,
}

fn main() -> Result<()> {
    MyEnum::VARIANTS.iter().for_each(|v| {
        println!("【 v 】==> {:?}", v);
    });
    MyEnum::iter().for_each(|v| {
        println!("【 v 】==> {:?}", v);
    });
    Ok(())
}
