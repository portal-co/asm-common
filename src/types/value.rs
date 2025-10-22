use super::*;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///The bit width of a value
pub struct Bitness {
    pub log2: u8,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///A constant
pub struct Constant {
    ///Native endianness, which is usually little
    pub data: [u64; 8],
}
impl Constant {
    pub fn bytes(&self, b: Bitness) -> impl Iterator<Item = u8> {
        self.data
            .into_iter()
            .flat_map(|a| a.to_ne_bytes())
            .take(1 << (b.log2 - 3))
    }
    pub fn bits(&self, b: Bitness) -> impl Iterator<Item = bool> {
        self.bytes(b)
            .flat_map(|a| bitvec::array::BitArray::<u8, Lsb0>::new(a).into_iter())
            .take(1 << (b.log2))
    }
    pub fn from_bytes(b: Bitness, i: impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self {
            data: array_init::from_iter(
                i.chain(once(0u8).cycle().take((512 / 8) - (1 << (b.log2 - 3))))
                    .batching(|i| array_init::from_iter(i))
                    .map(|a| u64::from_ne_bytes(a)),
            )?,
        })
    }
    pub fn from_bits(b: Bitness, i: impl Iterator<Item = bool>) -> Option<Self> {
        Self::from_bytes(
            b,
            i.chain(once(false).cycle().take(
                8 - (match (1 << (b.log2)) % 8 {
                    0 => 8,
                    a => a,
                }),
            ))
            .batching(|a| {
                Some({
                    let x: [bool; 8] = array_init::from_iter(a)?;
                    x.into_iter()
                        .rev()
                        .fold(0, |a, b| (a * 2) + (if b { 1 } else { 0u8 }))
                })
            }),
        )
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value<G> {
    pub offset: G,
    pub bitness: Bitness,
}
impl<G> Value<G> {
    pub fn as_mut<'a>(&'a mut self) -> Value<&'a mut G> {
        Value {
            offset: &mut self.offset,
            bitness: self.bitness,
        }
    }
    pub fn as_ref<'a>(&'a self) -> Value<&'a G> {
        Value {
            offset: &self.offset,
            bitness: self.bitness,
        }
    }
    pub fn map<G2, E>(self, f: &mut (dyn FnMut(G) -> Result<G2, E> + '_)) -> Result<Value<G2>, E> {
        Ok(match self {
            Value { offset, bitness } => Value {
                offset: f(offset)?,
                bitness,
            },
        })
    }
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LoadStoreFrame<G> {
    Value {
        bits: Bitness,
        val: Value<G>,
        bit_offset: usize,
    },
    Constant {
        bits: Bitness,
        constant: Constant,
    },
}
impl<G> LoadStoreFrame<G> {
    pub fn map<G2, E>(
        self,
        f: &mut (dyn FnMut(G) -> Result<G2, E> + '_),
    ) -> Result<LoadStoreFrame<G2>, E> {
        Ok(match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits,
                val: val.map(f)?,
                bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => {
                LoadStoreFrame::Constant { bits, constant }
            }
        })
    }
    pub fn as_ref<'a>(&'a self) -> LoadStoreFrame<&'a G> {
        match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits: *bits,
                val: val.as_ref(),
                bit_offset: *bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => LoadStoreFrame::Constant {
                bits: *bits,
                constant: *constant,
            },
        }
    }
    pub fn as_mut<'a>(&'a mut self) -> LoadStoreFrame<&'a mut G> {
        match self {
            LoadStoreFrame::Value {
                bits,
                val,
                bit_offset,
            } => LoadStoreFrame::Value {
                bits: *bits,
                val: val.as_mut(),
                bit_offset: *bit_offset,
            },
            LoadStoreFrame::Constant { bits, constant } => LoadStoreFrame::Constant {
                bits: *bits,
                constant: *constant,
            },
        }
    }
}
pub trait Any: Iterator {}
impl<T: Iterator + ?Sized> Any for T {}
pub trait All: Iterator {}
impl<T: Iterator + ?Sized> All for T {}
///First element replaced first, until the last, which is discarded
pub trait AssignChain: Iterator {}
impl<T: Iterator + ?Sized> AssignChain for T {}
