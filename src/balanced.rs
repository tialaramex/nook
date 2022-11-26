#![deny(unsafe_op_in_unsafe_fn)]

macro_rules! impl_pass_thru_fmt {
    ( ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
        $(
            impl std::fmt::$Trait for $Ty {
                #[inline]
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    self.get().fmt(f)
                }
            }
        )+
    }
}

macro_rules! make_balanced {
    ($Ty: ident($Int: ty) $low: expr, $high: expr) => {
        /// A balanced signed integer, similar to the ordinary unbalanced integer of the same size except
        /// the minimum extent is shrunk by one, balancing the integer and providing a niche.
        #[doc = concat!("Thus `Option<", stringify!($Ty), ">` is guaranteed to have the same size as `", stringify!($Int), "`:")]
        ///
        /// ```rust
        /// use std::mem::size_of;
        #[doc = concat!("assert_eq!(size_of::<Option<nook::", stringify!($Ty), ">>(), size_of::<", stringify!($Int), ">());")]
        /// ```
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
        #[repr(transparent)]
        #[rustc_layout_scalar_valid_range_start($high)]
        #[rustc_layout_scalar_valid_range_end($low)]
        #[rustc_nonnull_optimization_guaranteed]
        pub struct $Ty($Int);

        impl_pass_thru_fmt! { (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty }

        impl $Ty {

            #[must_use]
            #[inline]
            #[doc = concat!("Creates a balanced integer without checking the value is not ", stringify!($Int), "::MIN.")]
            #[doc = concat!("This results in undefined behaviour if the value is ", stringify!($Int), "::MIN")]
            ///
            /// # Safety
            ///
            #[doc = concat!("The value must not be ", stringify!($Int), "::MIN")]
            
            pub const unsafe fn new_unchecked(n: $Int) -> $Ty {
                unsafe {
                    Self(n)
                }
            }

            #[must_use]
            #[inline]
            #[doc = concat!("Creates a balanced integer unless the given value is ", stringify!($Int), "::MIN")]
            pub const fn new(n: $Int) -> Option<$Ty> {
                if n > <$Int>::MIN {
                    // SAFETY: we checked this isn't the excluded minimum
                    Some(unsafe { Self(n) })
                } else {
                    None
                }
            }

            #[inline]
            /// Just the value as a primitive type.
            pub const fn get(self) -> $Int {
                self.0
            }

            /// The smallest value that can be represented in this balanced integer type
            // SAFETY: we're one higher than the excluded value
            pub const MIN: $Ty = unsafe { Self::new_unchecked(<$Int>::MIN + 1) };

            /// The largest value that can be represented in this balanced integer type
            // SAFETY: we're at the far end of the range
            pub const MAX: $Ty = unsafe { Self::new_unchecked(<$Int>::MAX) };

            /// Computes the absolute value of self. Note that because this is balanced type this
            /// operation can't overflow.
            ///
            /// # Example
            ///
            /// ```
            #[doc = concat!("assert_eq!(nook::", stringify!($Ty), "::MIN.abs(), nook::", stringify!($Ty), "::MAX);")]
            /// ```
            #[inline]
            pub const fn abs(self) -> $Ty {
                // SAFETY: the only problematic value is excluded
                unsafe {
                    $Ty::new_unchecked(self.get().abs())
                }
            }
        }

        impl From<$Ty> for $Int {
            #[inline]
            fn from(n: $Ty) -> Self {
                n.0
            }
        }

    };
}

make_balanced! { BalancedI8(i8) 0x7F, 0x81 }
make_balanced! { BalancedI16(i16) 0x7FFF, 0x8001 }
make_balanced! { BalancedI32(i32) 0x7FFFFFFF, 0x80000001 }
make_balanced! { BalancedI64(i64) 0x7FFFFFFFFFFFFFFF, 0x8000000000000001 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_abs() {
        let p = BalancedI32::new(0).unwrap();
        assert_eq!(p, p.abs());
    }

    #[test]
    fn format_hex() {
        let s = format!("{0} {0:x} {0:X} {0:x?} {0:X?}", BalancedI8::MAX);
        assert_eq!(s, "127 7f 7F 7f 7F");
    }

    #[test]
    fn format_bases() {
        let s = format!("{0} {0:o} {0:b}", BalancedI8::MAX);
        assert_eq!(s, "127 177 1111111");
    }

    #[test]
    fn format_fill() {
        let s = format!("{0:_<8} {0:\"^1$} {0:_>8}", BalancedI8::new(0).unwrap(), 5);
        assert_eq!(s, r#"0_______ ""0"" _______0"#);
    }

    #[test]
    fn format_neg() {
        let s = format!("{0} {0:X?} {0:o}", BalancedI8::MIN);
        // Yup, hexadecimal and octal formats are defined as-if this was an unsigned type
        assert_eq!(s, "-127 81 201");
    }
}
