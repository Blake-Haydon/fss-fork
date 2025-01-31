// Copyright (C) myl7
// SPDX-License-Identifier: Apache-2.0

//! Many variable names together with the LaTeX math expressions in the doc comment are from the paper _Function Secret Sharing for Mixed-Mode and Fixed-Point Secure Computation_

#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]

use group::Group;

pub mod dcf;
pub mod dpf;
pub mod group;
pub mod owcf;
#[cfg(feature = "prg")]
pub mod prg;
pub mod utils;

/// Point function.
/// Despite the name, it only ships an element of the domain and an element of the range.
/// The actual meaning of the 2 elements is determined by the context.
///
/// - `N` is the **byte** size of the domain.
/// - `LAMBDA` here is used as the **byte** size of the range, unlike the one in the paper.
pub struct PointFn<const N: usize, const LAMBDA: usize, G>
where
    G: Group<LAMBDA>,
{
    /// `$\alpha$`, or say `x` in `y = f(x)`
    pub alpha: [u8; N],
    /// `$\beta$`, or say `y` in `y = f(x)`
    pub beta: G,
}

macro_rules! decl_prg_trait {
    ($ret_elem:ty) => {
        /// Pseudorandom generator
        ///
        /// Requires `Sync` for multi-threading, which should be still easy for even single-threaded
        pub trait Prg<const LAMBDA: usize>: Sync {
            fn gen(&self, seed: &[u8; LAMBDA]) -> [$ret_elem; 2];
        }
    };
}
pub(crate) use decl_prg_trait;

/// Pseudorandom generator to generate bytes
///
/// The output and input sizes are related with the sizes set by users.
/// They should be checked by the implementation.
///
/// Requires `Sync` for multi-threading, which should be still easy for even single-threaded
///
/// NOTICE: The trait with the impls still has performance issues,
/// Use [`crate::dpf::prg`] or [`crate::dcf::prg`] instead.
pub trait PrgBytes: Sync {
    fn gen(&self, buf: &mut [u8], src: &[u8]);
}

/// `Cw`. Correclation word.
#[derive(Clone)]
pub struct Cw<const LAMBDA: usize, G>
where
    G: Group<LAMBDA>,
{
    pub s: [u8; LAMBDA],
    pub v: G,
    pub tl: bool,
    pub tr: bool,
}

/// `k`.
///
/// `cws` and `cw_np1` is shared by the 2 parties.
/// Only `s0s[0]` is different.
#[derive(Clone)]
pub struct Share<const LAMBDA: usize, G>
where
    G: Group<LAMBDA>,
{
    /// For the output of `gen`, its length is 2.
    /// For the input of `eval`, the first one is used.
    pub s0s: Vec<[u8; LAMBDA]>,
    /// The length of `cws` must be `n = 8 * N`
    pub cws: Vec<Cw<LAMBDA, G>>,
    /// `$CW^{(n + 1)}$`
    pub cw_np1: G,
}
