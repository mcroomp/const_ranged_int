# const_ranged_int

`const_ranged_int` is a very simple Rust crate provides ranged constant values. It allows you to define constant values within a specific (inclusive) range.
The ```new``` function will assert that the value is within range, which is optimized out by the compiler.

This crate is not very ambitious, it is designed to be used for lookup tables that have constrained values. For example, if you had a lookup table that 
contains the values 0..=7 and you used this value to lookup in another array, this crate would let you avoid a bounds check in the second lookup since
this crate assumes (using ```std::hint::unreachable_unchecked()```) that the values are within MIN..=MAX.

Important note: the range is INCLUSIVE vs the normal Rust exclusive range notation. The reason for this is that if values that are maxint are used, then it
would be impossible to express maxint+1 for specifying the range.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
ranged_const = "0.1.0"
```

Then, in your Rust code:

```rust
use const_ranged_int::RangedConstU8;

// single constant (panics at compile time if values are out of range)
const VALUE: RangedConstU8<1, 10> = RangedConstU8::<1, 10>::new(5);

// array of constants (panics at compile time if values are out of range)
const CONTARRAY: [RangedConstU8<1, 10>; 5] =
       RangedConstU8::<1, 10>::into_array([1, 2, 3, 4, 5]);


println!("{}", VALUE.value());

```