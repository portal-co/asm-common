use core::mem::{replace, transmute};

use sha3::Digest;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Ratchet {
    seed: [u8; 32],
}
impl Ratchet {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { seed }
    }
    pub fn next(&mut self) -> [u8; 32] {
        let s = self.seed;
        self.seed = sha3::Sha3_256::digest(s).into();
        s
    }
    pub fn split<'a>(mut self, mut a: &'a [u8]) -> impl Iterator<Item = &'a [u8]> {
        core::iter::from_fn(move || {
            if a.len() == 0 {
                return None;
            }
            let n = self.next();
            let mut i = 0;
            let old = a;
            a = loop {
                if a[i..][..32] == n[..] {
                    break &a[i..];
                }
                i += 1;
                if i == a.len() {
                    return Some(replace(&mut a, &[]));
                }
            };
            a = &a[32..];
            return Some(&old[..i]);
        })
    }
    pub fn split_mut<'a>(
        mut self,
        mut a: &'a mut [u8],
        replacer: Option<[u8; 32]>,
    ) -> impl Iterator<Item = &'a mut [u8]> + use<'a> {
        core::iter::from_fn(move || {
            if a.len() == 0 {
                return None;
            }
            let n = self.next();
            let mut i = 0;
            let old: &mut [u8];
            let b: &mut [u8];
            loop {
                if a[i..][..32] == n[..] {
                    (old, b) = a.split_at_mut(i);
                    if let Some(r) = replacer.as_ref() {
                        b[..32].copy_from_slice(r);
                    }
                    //SAFETY: slice contents seperate from `old` by `split_at_mut`
                    let b: &mut [u8] = unsafe { transmute(b) };
                    a = &mut b[32..];
                    //SAFETY: slice contents only outputted here
                    return Some(unsafe { transmute(old) });
                }
                i += 1;
                if i == a.len() {
                    return Some(match replace(&mut a, &mut []) {
                        a => {
                            //SAFETY: slice taken so it can only be used once
                            unsafe { transmute(a) }
                        }
                    });
                }
            }
            // a = &mut a[32..];
            // return Some(old);
        })
    }
}
