#[derive(PartialEq, Debug)]
#[cfg_attr(feature = "serialisation", derive(Serialize, Deserialize))]
pub enum GameboyButton {
    A,
    B,
    LEFT,
    RIGHT,
    UP,
    DOWN,
    START,
    SELECT,
}