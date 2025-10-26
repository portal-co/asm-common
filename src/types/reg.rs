use super::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Reg(pub u8);
impl Reg {
    pub const CTX: Reg = Reg(255);
    pub const fn r32_swap_0_and_31(&self) -> Self {
        match self.0 % 32 {
            0 => Self(31),
            31 => Self(0),
            v => Self(v),
        }
    }
    pub const fn r32(&self) -> Self {
        return Self(self.0 % 32);
    }
}
