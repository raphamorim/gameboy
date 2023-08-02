#[derive(PartialEq, Debug, Copy, Clone)]
pub enum GbMode {
    Classic,
    Color,
    ColorAsClassic,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Single,
    Double,
}
