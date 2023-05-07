#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Signed(pub Sign);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Unsigned;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
    #[default]
    Pos,
    Neg,
}
