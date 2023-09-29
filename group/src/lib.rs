// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;
use std::ops::{Add, AddAssign};

#[cfg(feature = "byte")]
pub mod byte;
#[cfg(feature = "int")]
pub mod int;

/// Group (mathematics) that can be converted from a byte array
pub trait Group<const LAMBDA: usize>
where
    Self: Add<Output = Self> + AddAssign + PartialEq + Eq + Debug + Sized + Clone + Sync + Send,
{
    /// Convert from a byte array to a group element
    fn convert(y: [u8; LAMBDA]) -> Self;
    /// Helper to convert from a byte array to a group element by cloning the byte array
    fn clone_convert(y: &[u8; LAMBDA]) -> Self {
        Self::convert(y.clone())
    }

    /// Zero in the group
    fn zero() -> Self;

    /// Additive inverse in the group, e.g., `-x` for `x` in the integer group
    fn add_inverse(self) -> Self;
    /// Helper to get the additive inverse if true.
    /// Used for expressions like `$(-1)^n x$`, in which `t` can be computed from `n`.
    fn add_inverse_if(self, t: bool) -> Self {
        if t {
            self.add_inverse()
        } else {
            self
        }
    }
}