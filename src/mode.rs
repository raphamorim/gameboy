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

crate::state::enum_state!(GbMode { GbMode::Classic, GbMode::Color, GbMode::ColorAsClassic });
crate::state::enum_state!(GbSpeed { GbSpeed::Single, GbSpeed::Double });
