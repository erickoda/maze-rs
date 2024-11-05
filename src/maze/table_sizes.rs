use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum DefaultMazeTableSizes {
    TenPerTen,
    FiftyPerFifty,
    OneHundredPerOneHundred,
    TwoHundredPerTwoHundred,
}

impl From<DefaultMazeTableSizes> for String {
    fn from(value: DefaultMazeTableSizes) -> Self {
        match value {
            DefaultMazeTableSizes::TenPerTen => String::from("10x10"),
            DefaultMazeTableSizes::FiftyPerFifty => String::from("50x50"),
            DefaultMazeTableSizes::OneHundredPerOneHundred => String::from("100x100"),
            DefaultMazeTableSizes::TwoHundredPerTwoHundred => String::from("200x200"),
        }
    }
}
