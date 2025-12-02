//! Cryptographic seed ratcheting mechanism.
//!
//! This module provides a ratchet type for generating deterministic sequences
//! of pseudo-random values using SHA3-256. The ratchet can also be used to
//! split byte slices at positions marked by ratchet-generated values.
//!
//! ## Purpose
//!
//! The ratchet is designed for marking targeted modifications to supporting code.
//! Ratchet markers can be embedded in assembly to designate sections for special
//! processing. Consumers can split on these markers to apply custom transformations,
//! while the same code would crash if executed on real hardware (preventing accidental
//! execution of marker bytes).
//!
//! Available only with the `ratchet` feature enabled.

use core::mem::{replace, transmute};

use sha3::Digest;

/// A cryptographic ratchet based on SHA3-256.
///
/// The ratchet maintains an internal seed and generates a sequence of
/// deterministic pseudo-random values by repeatedly hashing the seed.
/// Each call to [`next`](Ratchet::next) produces the current seed value
/// and advances the internal state.
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "ratchet")]
/// # {
/// use portal_pc_asm_common::ratchet::Ratchet;
///
/// let mut ratchet = Ratchet::from_seed([0u8; 32]);
/// let value1 = ratchet.next();
/// let value2 = ratchet.next();
///
/// // Each value is deterministic but different
/// assert_ne!(value1, value2);
/// # }
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Ratchet {
    seed: [u8; 32],
}
impl Ratchet {
    /// Creates a new ratchet from an initial seed.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ratchet")]
    /// # {
    /// use portal_pc_asm_common::ratchet::Ratchet;
    ///
    /// let seed = [42u8; 32];
    /// let ratchet = Ratchet::from_seed(seed);
    /// # }
    /// ```
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { seed }
    }

    /// Generates the next value in the sequence and advances the ratchet.
    ///
    /// Returns the current seed value, then updates the internal seed to
    /// SHA3-256(current_seed).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ratchet")]
    /// # {
    /// use portal_pc_asm_common::ratchet::Ratchet;
    ///
    /// let mut ratchet = Ratchet::from_seed([0u8; 32]);
    /// let first = ratchet.next();
    /// let second = ratchet.next();
    /// assert_ne!(first, second);
    /// # }
    /// ```
    pub fn next(&mut self) -> [u8; 32] {
        let s = self.seed;
        self.seed = sha3::Sha3_256::digest(s).into();
        s
    }
    /// Splits a byte slice at positions marked by ratchet-generated values.
    ///
    /// Searches for occurrences of ratchet-generated 32-byte sequences in the
    /// input slice and yields the chunks between them. The ratchet advances
    /// for each chunk, so the markers are deterministically generated.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ratchet")]
    /// # {
    /// use portal_pc_asm_common::ratchet::Ratchet;
    ///
    /// let mut ratchet = Ratchet::from_seed([0u8; 32]);
    /// let marker1 = ratchet.next();
    /// let marker2 = ratchet.next();
    ///
    /// let mut data = Vec::new();
    /// data.extend_from_slice(b"chunk1");
    /// data.extend_from_slice(&marker1);
    /// data.extend_from_slice(b"chunk2");
    /// data.extend_from_slice(&marker2);
    ///
    /// let ratchet = Ratchet::from_seed([0u8; 32]);
    /// let chunks: Vec<&[u8]> = ratchet.split(&data).collect();
    /// assert_eq!(chunks.len(), 2);
    /// assert_eq!(chunks[0], b"chunk1");
    /// assert_eq!(chunks[1], b"chunk2");
    /// # }
    /// ```
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
    /// Splits a mutable byte slice at positions marked by ratchet-generated values.
    ///
    /// Similar to [`split`](Ratchet::split), but works with mutable slices and
    /// optionally replaces each marker with a different value.
    ///
    /// # Parameters
    ///
    /// - `a`: The byte slice to split
    /// - `replacer`: If `Some`, each marker found will be replaced with this value
    ///
    /// # Safety
    ///
    /// This function uses unsafe code internally to satisfy borrow checker
    /// requirements while maintaining memory safety invariants.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "ratchet")]
    /// # {
    /// use portal_pc_asm_common::ratchet::Ratchet;
    ///
    /// let mut ratchet = Ratchet::from_seed([0u8; 32]);
    /// let marker1 = ratchet.next();
    /// let marker2 = ratchet.next();
    ///
    /// let mut data = Vec::new();
    /// data.extend_from_slice(b"chunk1");
    /// data.extend_from_slice(&marker1);
    /// data.extend_from_slice(b"chunk2");
    /// data.extend_from_slice(&marker2);
    ///
    /// let ratchet = Ratchet::from_seed([0u8; 32]);
    /// let chunks: Vec<&mut [u8]> = ratchet.split_mut(&mut data, None).collect();
    /// assert_eq!(chunks.len(), 2);
    /// assert_eq!(chunks[0], b"chunk1");
    /// assert_eq!(chunks[1], b"chunk2");
    /// # }
    /// ```
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
