use super::*;
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "enum-map", derive(enum_map::Enum))]
#[cfg_attr(feature = "exhaust", derive(exhaust::Exhaust))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
///A permission
pub enum Perm {
    Read,
    Write,
    Exec,
    ///NOT a jump target
    NoJump,
}
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Perms<T> {
    pub r: T,
    pub w: T,
    pub x: T,
    pub nj: T,
}
impl<T> Perms<T> {
    pub fn map<U>(self, mut f: impl FnMut(T) -> U) -> Perms<U> {
        let Perms { r, w, x, nj } = self;
        Perms {
            r: f(r),
            w: f(w),
            x: f(x),
            nj: f(nj),
        }
    }
}
#[cfg(feature = "enum-map")]
const _: () = {
    use core::mem::{replace, MaybeUninit};

    impl<T, U: Into<T>> From<Perms<U>> for enum_map::EnumMap<Perm, T> {
        fn from(value: Perms<U>) -> Self {
            let value: Perms<T> = Perms {
                r: value.r.into(),
                w: value.w.into(),
                x: value.x.into(),
                nj: value.nj.into(),
            };
            let mut value: Perms<MaybeUninit<T>> = Perms {
                r: MaybeUninit::new(value.r),
                w: MaybeUninit::new(value.w),
                x: MaybeUninit::new(value.x),
                nj: MaybeUninit::new(value.nj),
            };
            return unsafe {
                enum_map::enum_map! {
                    Perm::Read => replace(&mut value.r,MaybeUninit::uninit()).assume_init(),
                    Perm::Write => replace(&mut value.w,MaybeUninit::uninit()).assume_init(),
                    Perm::Exec => replace(&mut value.x,MaybeUninit::uninit()).assume_init(),
                    Perm::NoJump => replace(&mut value.nj,MaybeUninit::uninit()).assume_init()
                }
            };
        }
    }
    impl<T, U: Into<T>> From<enum_map::EnumMap<Perm, U>> for Perms<T> {
        fn from(value: enum_map::EnumMap<Perm, U>) -> Self {
            let mut value: enum_map::EnumMap<Perm, MaybeUninit<T>> =
                value.map(|_, a| MaybeUninit::new(a.into()));
            return unsafe {
                Self {
                    r: replace(&mut value[Perm::Read], MaybeUninit::uninit()).assume_init(),
                    w: replace(&mut value[Perm::Write], MaybeUninit::uninit()).assume_init(),
                    x: replace(&mut value[Perm::Exec], MaybeUninit::uninit()).assume_init(),
                    nj: replace(&mut value[Perm::NoJump], MaybeUninit::uninit()).assume_init(),
                }
            };
        }
    }
};
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct InputRef<'a> {
    pub code: &'a [u8],
    pub r: &'a BitSlice,
    pub w: &'a BitSlice,
    pub x: &'a BitSlice,
    pub nj: &'a BitSlice,
    attest_same_size: (),
}
pub trait InputStream: ErrorType {
    fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error>;
    fn write_all(&mut self, mut i: InputRef<'_>) -> Result<(), Self::Error> {
        while i.code.len() > 0 {
            let x = self.write(i.nest())?;
            i = i.subref(x..);
        }
        Ok(())
    }
}
impl<'a, T: InputStream> InputStream for &'a mut T {
    fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error> {
        (&mut **self).write(i)
    }
    fn write_all(&mut self, mut i: InputRef<'_>) -> Result<(), Self::Error> {
        (&mut **self).write_all(i)
    }
}
impl<'a> Index<Perm> for InputRef<'a> {
    type Output = &'a BitSlice;

    fn index(&self, index: Perm) -> &Self::Output {
        match index {
            Perm::Read => &self.r,
            Perm::Write => &self.w,
            Perm::Exec => &self.x,
            Perm::NoJump => &self.nj,
        }
    }
}
impl<'a> InputRef<'a> {
    pub fn iter(&self) -> impl Iterator<Item = (u8, Perms<bool>)> + use<'a> {
        return self.code.iter().cloned().zip(
            self.r
                .iter()
                .by_vals()
                .zip(self.w.iter().by_vals())
                .zip(self.x.iter().by_vals())
                .zip(self.nj.iter().by_vals())
                .map(|x| Perms {
                    r: x.0 .0 .0,
                    w: x.0 .0 .1,
                    x: x.0 .1,
                    nj: x.1,
                }),
        );
    }
    pub fn len(&self) -> usize {
        return self.code.len();
    }
    pub fn subref<T: Clone>(self, r: T) -> Self
    where
        [u8]: Index<T, Output = [u8]>,
        BitSlice: Index<T, Output = BitSlice>,
    {
        // let r = start..end;
        Self {
            code: &self.code[r.clone()],
            r: &self.r[r.clone()],
            w: &self.w[r.clone()],
            x: &self.x[r.clone()],
            nj: &self.nj[r],
            attest_same_size: (),
        }
    }
    pub fn nest<'b>(&'b self) -> InputRef<'b> {
        InputRef {
            code: &self.code,
            r: &self.r,
            w: &self.w,
            x: &self.x,
            nj: &self.nj,
            attest_same_size: (),
        }
    }
    pub fn new(code: &'a [u8], perms: Perms<&'a BitSlice>) -> Option<Self> {
        let r = perms.r;
        let w = perms.w;
        let x = perms.x;
        let nj = perms.nj;
        Some(Self {
            code: code,
            r: r,
            w: w,
            x: x,
            nj: nj,
            attest_same_size: if [r, w, x, nj].iter().all(|a| a.len() == code.len()) {
            } else {
                return None;
            },
        })
    }
    #[cfg(feature = "enum-map")]
    pub fn new_mapped(
        code: &'a [u8],
        perms: enum_map::EnumMap<Perm, &'a BitSlice>,
    ) -> Option<Self> {
        Self::new(code, perms.into())
    }
}
#[cfg(feature = "alloc")]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Input {
    code: alloc::vec::Vec<u8>,
    r: bitvec::vec::BitVec,
    w: bitvec::vec::BitVec,
    x: bitvec::vec::BitVec,
    nj: bitvec::vec::BitVec,
    attest_same_size: (),
}
#[cfg(feature = "alloc")]
const _: () = {
    use alloc::{borrow::ToOwned, vec::Vec};
    use bitvec::vec::BitVec;
    use no_error_type::NoError;

    impl<'a> InputRef<'a> {
        pub fn to_owned(&self) -> Input {
            Input {
                code: self.code.to_owned(),
                r: self.r.to_owned(),
                w: self.w.to_owned(),
                x: self.x.to_owned(),
                nj: self.nj.to_owned(),
                attest_same_size: (),
            }
        }
    }
    impl Input {
        pub fn as_ref<'a>(&'a self) -> InputRef<'a> {
            InputRef {
                code: &self.code,
                r: self.r.as_bitslice(),
                w: self.w.as_bitslice(),
                x: self.x.as_bitslice(),
                nj: self.nj.as_bitslice(),
                attest_same_size: (),
            }
        }
        pub fn new(code: Vec<u8>, perms: Perms<BitVec>) -> Option<Self> {
            let r = perms.r;
            let w = perms.w;
            let x = perms.x;
            let nj = perms.nj;
            let attest_same_size = if [&r, &w, &x, &nj].iter().all(|a| a.len() == code.len()) {
            } else {
                return None;
            };
            Some(Self {
                code,
                r,
                w,
                x,
                nj,
                attest_same_size,
            })
        }
        pub fn len(&self) -> usize {
            return self.code.len();
        }
        pub fn into_parts(self) -> (Vec<u8>, BitVec, BitVec, BitVec, BitVec) {
            (self.code, self.r, self.w, self.x, self.nj)
        }
        #[cfg(feature = "enum-map")]
        pub fn into_mapped_parts(self) -> (Vec<u8>, enum_map::EnumMap<Perm, BitVec>) {
            (
                self.code,
                enum_map::enum_map! {Perm::Read => self.r.clone(),Perm::Write => self.w.clone(),Perm::Exec => self.x.clone(),Perm::NoJump => self.nj.clone()},
            )
        }
        #[cfg(feature = "enum-map")]
        pub fn new_mapped(code: Vec<u8>, perms: enum_map::EnumMap<Perm, BitVec>) -> Option<Self> {
            Self::new(code, perms.into())
        }
        pub fn extend(&mut self, i: impl Iterator<Item = (u8, Perms<bool>)>) {
            for (c, Perms { r, w, x, nj }) in i {
                self.code.push(c);
                self.r.push(r);
                self.w.push(w);
                self.x.push(x);
                self.nj.push(nj);
            }
        }
        #[cfg(feature = "enum-map")]
        pub fn extend_mapped(
            &mut self,
            i: impl Iterator<Item = (u8, enum_map::EnumMap<Perm, bool>)>,
        ) {
            self.extend(i.map(|(c, p)| (c, p.into())));
        }
    }
    impl ErrorType for Input {
        type Error = NoError;
    }
    impl InputStream for Input {
        fn write(&mut self, i: InputRef<'_>) -> Result<usize, Self::Error> {
            self.code.extend_from_slice(i.code);
            for (a, b) in [
                (i.r, &mut self.r),
                (i.w, &mut self.w),
                (i.x, &mut self.x),
                (i.nj, &mut self.nj),
            ] {
                b.extend_from_bitslice(a);
            }
            Ok(i.code.len())
        }
    }
};
