use core::ops::Range;

use super::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(transparent)]
pub struct InstCodeI4(pub u32);
impl InstCodeI4 {
    pub fn with(self, inst: impl Iterator<Item = Range<u32>>, val: impl Into<u32>) -> Self {
        let val = val.into();
        let mut code = self.0;
        let mut len = 0;
        for r in inst {
            let mask = ((1u32 << (r.end - r.start)) - 1) << r.start;
            code = (code & !mask) | (((val >> len) << r.start) & mask);
            len += r.end - r.start;
        }
        InstCodeI4(code)
    }
    pub fn extract<T>(&self, list: impl Iterator<Item = Range<u32>>) -> T
    where
        u32: Into<T>,
    {
        let mut val = 0;
        let mut len = 0;
        for r in list {
            let mask = ((1u32 << (r.end - r.start)) - 1) << r.start;
            val |= ((self.0 & mask) >> r.start) << len;
            len += r.end - r.start;
        }
        val.into()
    }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(transparent)]
pub struct InstCodeSlice<S>(pub S);
impl<S: AsRef<[u8]>> InstCodeSlice<S> {
    pub fn extract<T: From<u64>>(&self, list: impl Iterator<Item = Range<usize>>) -> T {
        let bytes = self.0.as_ref();
        let mut val = 0u64;
        let mut len = 0;
        for r in list {
            let mut part = 0u64;
            for i in r.clone() {
                part |= (bytes[i >> 3] as u64) << (i << 3);
            }
            val |= part << len;
            len += (r.end - r.start) * 8;
        }
        val.into()
    }
}
impl<S: AsMut<[u8]>> InstCodeSlice<S> {
    pub fn with(mut self, inst: impl Iterator<Item = Range<usize>>, val: impl Into<u64>) -> Self {
        let val = val.into();
        let bytes = self.0.as_mut();
        let mut len = 0;
        for r in inst {
            let mut part = 0u64;
            for i in r.clone() {
                part |= (bytes[i >> 3] as u64) << (i << 3);
            }
            let mask = ((1u64 << ((r.end - r.start) * 8)) - 1) << (r.start * 8);
            part = (part & !mask) | (((val >> len) << (r.start * 8)) & mask);
            for i in r.clone() {
                bytes[i >> 3] = ((part >> (i << 3)) & 0xff) as u8;
            }
            len += (r.end - r.start) * 8;
        }
        self
    }
}
