use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum DefaultMazeBoardSizes {
    TenPerTen,
    FiftyPerFifty,
    OneHundredPerOneHundred,
    TwoHundredPerTwoHundred,
}

impl From<DefaultMazeBoardSizes> for String {
    fn from(value: DefaultMazeBoardSizes) -> Self {
        match value {
            DefaultMazeBoardSizes::TenPerTen => String::from("10x10"),
            DefaultMazeBoardSizes::FiftyPerFifty => String::from("50x50"),
            DefaultMazeBoardSizes::OneHundredPerOneHundred => String::from("100x100"),
            DefaultMazeBoardSizes::TwoHundredPerTwoHundred => String::from("200x200"),
        }
    }
}
