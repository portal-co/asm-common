use super::*;
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///An arithmetic operation
pub enum Arith {
    Add,
    Sub,
    ///Results in a [`Bitness`] doubled, or with an incremented [`Bitness::log2`]
    Mul,
    Div(Sign),
    Rem(Sign),
    And,
    Or,
    Xor,
    Shl,
    Shr(Sign),
    Rotl(Sign),
    Rotr(Sign),
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///The signedness of a number
pub enum Sign {
    Unsigned,
    Signed,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///An endianness
pub enum Endian {
    Little,
    Big,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///A method of numerical extension
pub enum Ext {
    Sign,
    Zero,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Cmp {
    Le(Sign),
    Lt(Sign),
    Eq,
    Gt(Sign),
    Ge(Sign),
    Ne,
}
