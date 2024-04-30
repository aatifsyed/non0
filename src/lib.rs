#![no_std]

#[rustfmt::skip]
macro_rules! define {
    ($dollar:tt $name:ident $ty:ident) => {
        #[doc = concat!("Create a [`", stringify!($ty), "`](::core::num::", stringify!($ty), ")")]
        /// from a `const`-evaluable expression.
        #[macro_export]
        macro_rules! $name {
            ($dollar expr:expr) => {{
                const ARGUMENT_MUST_BE_CONST: $crate::__private::core::num::$ty =
                    match $crate::__private::core::num::$ty::new($dollar expr) {
                        $crate::__private::core::option::Option::Some(it) => it,
                        _ => $crate::__private::core::panic!("argument was zero"),
                    };
                ARGUMENT_MUST_BE_CONST
            }};
        }
    };
}

define!($ i8 NonZeroI8);
define!($ i16 NonZeroI16);
define!($ i32 NonZeroI32);
define!($ i64 NonZeroI64);
define!($ i128 NonZeroI128);
define!($ isize NonZeroIsize);
define!($ u8 NonZeroU8);
define!($ u16 NonZeroU16);
define!($ u32 NonZeroU32);
define!($ u64 NonZeroU64);
define!($ u128 NonZeroU128);
define!($ usize NonZeroUsize);

/// Implementation detail, semver exempt
#[doc(hidden)]
pub mod __private {
    pub extern crate core;
}
