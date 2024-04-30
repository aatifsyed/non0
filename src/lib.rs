//! Compile-time checked non-zero integers with type inference and first-class `const` support.
//!
//! ```
//! # use nonzero::nonzero;
//! # use core::num::{NonZeroUsize, NonZeroI8};
//! const UNSIGNED: NonZeroUsize = nonzero!(1);
//! const SIGNED: NonZeroI8 = nonzero!(-1);
//!            // ^ correctly infers return type
//!
//! const MY_CONST: usize = 20;
//! const MY_NONZERO_CONST: NonZeroUsize = nonzero!(MY_CONST - 19);
//!               // refer to other constant values ^
//! ```
//!
//! ```compile_fail
//! let runtime = nonzero!(0); // eager evaluation
//! ```

#![no_std]

use core::{mem, num::*, slice};
use zerocopy::AsBytes;

use __private::is_default;

/// # Safety
/// - it must be safe to [`mem::transmute_copy`] a [`NonZero::Primitive`] to an `impl`[`NonZero`]
///   provided that it is not the same bit pattern as [`NonZero::Primitive::DEFAULT_REF`](ConstDefaultRef::DEFAULT_REF).
#[doc(hidden)]
pub unsafe trait NonZero {
    #[allow(private_bounds)]
    type Primitive: ConstDefaultRef + AsBytes;
}

macro_rules! impl_nonzero {
    ($($primitive_ty:ty: $nonzero_ty:ty);* $(;)?) => {
        $(
            /// Safety
            /// - requirements are explicitly guaranteed by the docs for nonzero types.
            unsafe impl NonZero for $nonzero_ty {
                type Primitive = $primitive_ty;
            }
            impl ConstDefaultRef for $primitive_ty {
                const DEFAULT_REF: &'static Self = &0;
            }
        )*
    };
}

impl_nonzero! {
    i8: NonZeroI8;
    i16: NonZeroI16;
    i32: NonZeroI32;
    i64: NonZeroI64;
    i128: NonZeroI128;
    isize: NonZeroIsize;
    u8: NonZeroU8;
    u16: NonZeroU16;
    u32: NonZeroU32;
    u64: NonZeroU64;
    u128: NonZeroU128;
    usize: NonZeroUsize;
}

/// Evaluate the given `expr` at compile time, asserting that it is non-zero,
/// returning a (usually inferred) type-safe non-zero item.
///
/// # Panics
/// - If `expr` evaluates to zero
#[macro_export]
macro_rules! nonzero {
    ($expr:expr) => {{
        const COMPILE_TIME_CHECK_MUST_BE_CONST_EVALUABLE: () = {
            if $crate::__private::is_default(&$expr) {
                $crate::__private::core::panic!("`src` may not be zero")
            }
        };
        $crate::nonzero(&$expr)
    }};
}

/// You probably want the [`nonzero!`] macro instead.
///
/// # Panics
/// - If `src` is zero.
#[doc(hidden)]
#[track_caller]
pub const fn nonzero<T: NonZero>(src: &T::Primitive) -> T {
    match is_default(src) {
        true => panic!("`src` may not be zero"),
        // Safety:
        // - T: NonZero
        false => unsafe { mem::transmute_copy(src) },
    }
}

/// This can't build on [`zerocopy::FromZeroes`] because we can't take the
/// address of e.g a [`mem::zeroed`] in a `const` fn because it may contain an [`UnsafeCell`](core::cell::UnsafeCell)
trait ConstDefaultRef: 'static {
    const DEFAULT_REF: &'static Self;
}

const fn as_bytes<T: AsBytes>(src: &T) -> &[u8] {
    // Safety:
    // - T: AsBytes
    unsafe { slice::from_raw_parts(src as *const T as *const u8, mem::size_of::<T>()) }
}

const fn slice_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    let mut ix = right.len();
    while let Some(nix) = ix.checked_sub(1) {
        if left[nix] != right[nix] {
            return false;
        }
        ix = nix
    }

    true
}

/// Supporting functionality for the [`nonzero!`] macro.
///
/// Not a public API.
#[doc(hidden)]
pub mod __private {
    use super::*;

    pub extern crate core;

    #[allow(private_bounds)]
    pub const fn is_default<T: AsBytes + ConstDefaultRef>(src: &T) -> bool {
        let src = as_bytes(src);
        let zero = as_bytes(T::DEFAULT_REF);
        slice_eq(src, zero)
    }
}
