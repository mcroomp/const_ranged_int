macro_rules! ranged_const {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
        pub struct $name<const MIN: $type, const MAX: $type> {
            value: $type,
        }

        impl<const MIN: $type, const MAX: $type> Default for $name<MIN, MAX> {
            fn default() -> Self {
                Self { value: MIN }
            }
        }

        impl<const MIN: $type, const MAX: $type> std::fmt::Display for $name<MIN, MAX> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.value)
            }
        }

        impl<const MIN: $type, const MAX: $type> $name<MIN, MAX> {
            /// Create a new RangedConst value. Panics if the value is out of range
            pub const fn new(value: $type) -> Self {
                assert!(MIN < MAX, "MIN must be less than MAX");
                if value < MIN || value > MAX {
                    panic!("Value out of range");
                }
                Self { value }
            }

            /// Checks the value against the range and returns None if it is out of range
            pub const fn check_range(value: $type) -> Option<Self> {
                if value < MIN || value > MAX {
                    None
                } else {
                    Some(Self { value })
                }
            }

            /// Get the value of the RangedConst value as a primitive type
            /// assuming that it is in range
            pub const fn value(&self) -> $type {
                unsafe {
                    if self.value < MIN || self.value > MAX {
                        std::hint::unreachable_unchecked();
                    }
                    self.value
                }
            }

            /// Safely increments the value and returns None if we hit
            /// the maximum value.
            pub const fn increment(&self) -> Option<$type> {
                if self.value < MAX {
                    Some(self.value + 1)
                } else {
                    None
                }
            }

            /// Safely decrements the value and returns None if we hit
            /// the maximum value.
            pub const fn decrement(&self) -> Option<$type> {
                if self.value > MIN {
                    Some(self.value - 1)
                } else {
                    None
                }
            }

            /// Convert a slice of u8 into an array of ConstRangedX. Useful for const initialization, eg
            ///  ```const CONTARRAY : [ConstRangedX<1,10>;5] = ConstRangedX::<1,10>::into_array([1,2,3,4,5]);
            /// will panic if any value is out of range
            pub const fn into_array<const N: usize>(a: [$type; N]) -> [Self; N] {
                let mut r = [Self::new(MIN); N];
                let mut i = 0;
                while i < N {
                    r[i] = Self::new(a[i]);
                    i += 1;
                }
                r
            }
        }
    };
}

ranged_const!(ConstRangedU128, u128);
ranged_const!(ConstRangedU64, u64);
ranged_const!(ConstRangedU32, u32);
ranged_const!(ConstRangedU16, u16);
ranged_const!(ConstRangedU8, u8);
ranged_const!(ConstRangedUSize, usize);

ranged_const!(ConstRangedI128, i128);
ranged_const!(ConstRangedI64, i64);
ranged_const!(RangedConstI32, i32);
ranged_const!(RangedConstI16, i16);
ranged_const!(RangedConstI8, i8);
ranged_const!(RangedConstISize, isize);

#[test]
fn test_ranged_const_u32() {
    let value = ConstRangedU8::<1, 10>::new(5);
    assert_eq!(value.value(), 5);

    const VALUE: ConstRangedU8<1, 10> = ConstRangedU8::<1, 10>::new(5);
    assert_eq!(VALUE.value(), 5);

    const CONTARRAY: [ConstRangedU8<1, 10>; 5] =
        ConstRangedU8::<1, 10>::into_array([1, 2, 3, 4, 5]);
    for i in 0..5 {
        assert_eq!(CONTARRAY[i].value, i as u8 + 1);
    }

    let result = std::panic::catch_unwind(|| ConstRangedU8::<1, 10>::into_array([1, 2, 3, 4, 11]));
    assert!(result.is_err());
}
