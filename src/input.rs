#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub enum Input {
    A,
    B,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    START,
    SELECT,
}
