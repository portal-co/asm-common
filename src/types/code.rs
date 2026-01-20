use core::ops::Range;

use super::*;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
#[repr(transparent)]
pub struct InstCodeI4(pub u32);
impl InstCodeI4 {
    pub fn with(self, inst: impl Iterator<Item = Range<u32>>, val: u32) -> Self {
        let mut code = self.0;
        let mut len = 0;
        for r in inst {
            let mask = ((1u32 << (r.end - r.start)) - 1) << r.start;
            code = (code & !mask) | (((val >> len) << r.start) & mask);
            len += r.end - r.start;
        }
        InstCodeI4(code)
    }
    pub fn extract(&self, list: impl Iterator<Item = Range<u32>>) -> u32 {
        let mut val = 0;
        let mut len = 0;
        for r in list {
            let mask = ((1u32 << (r.end - r.start)) - 1) << r.start;
            val |= ((self.0 & mask) >> r.start) << len;
            len += r.end - r.start;
        }
        val
    }
}
