use super::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub enum MemorySize {
    _8,
    _16,
    _32,
    #[default]
    _64,
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct MemorySized<T> {
    pub value: T,
    pub size: MemorySize,
}
