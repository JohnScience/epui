# Equisized (primitive) unsigned ints for primitive ints

According to Rust's reference, [primitive numeric integer types][primitive numeric type] in Rust are such:

# Numeric types

## Integer types

The unsigned integer types consist of:

Type   | Minimum | Maximum
-------|---------|-------------------
`u8`   | 0       | 2<sup>8</sup>-1
`u16`  | 0       | 2<sup>16</sup>-1
`u32`  | 0       | 2<sup>32</sup>-1
`u64`  | 0       | 2<sup>64</sup>-1
`u128` | 0       | 2<sup>128</sup>-1

The signed two's complement integer types consist of:

Type   | Minimum            | Maximum
-------|--------------------|-------------------
`i8`   | -(2<sup>7</sup>)   | 2<sup>7</sup>-1
`i16`  | -(2<sup>15</sup>)  | 2<sup>15</sup>-1
`i32`  | -(2<sup>31</sup>)  | 2<sup>31</sup>-1
`i64`  | -(2<sup>63</sup>)  | 2<sup>63</sup>-1
`i128` | -(2<sup>127</sup>) | 2<sup>127</sup>-1

## Machine-dependent integer types

The `usize` type is an unsigned integer type with the same number of bits as the
platform's pointer type. It can represent every memory address in the process.

The `isize` type is a signed integer type with the same number of bits as the
platform's pointer type. The theoretical upper bound on object and array size
is the maximum `isize` value. This ensures that `isize` can be used to calculate
differences between pointers into an object or array and can address every byte
within an object along with one byte past the end.

`usize` and `isize` are at least 16-bits wide.

> **Note**: Many pieces of Rust code may assume that pointers, `usize`, and
> `isize` are either 32-bit or 64-bit. As a consequence, 16-bit
> pointer support is limited and may require explicit care and acknowledgment
> from a library to support.

# Why this trait is needed

All [primitive numeric integer types][primitive numeric type], including machine-dependent types, come with known size that can be obtained via [`core::mem::size_of<T>()`][core::mem::size_of] and the corresponding signed or unsigned counterpart with the exact size. Such algorithms as C++ 20 standard midpoint relies both on equisized primitive unsigned integers and on equisized primitive signed integers. This crate offers the former.

For equisized primitive signed integers, refer to [`epsi`](https://crates.io/crates/epsi) crate

## Signed integers

Type    | Size                   | Equisized primitive unsigned integer 
--------|------------------------|-------------------------------------
`i8`    | 1 byte                 | `u8`                              
`i16`   | 2 bytes                | `u16`                             
`i32`   | 4 bytes                | `u32`                             
`i64`   | 8 bytes                | `u64`                             
`i128`  | 16 bytes               | `u128`                            
`isize` | **platform-dependent** | `usize`

## Unsigned integers

Type    | Size                   | Equisized primitive unsigned integer 
--------|------------------------|-------------------------------------
`u8`    | 1 byte                 | `u8`                              
`u16`   | 2 bytes                | `u16`                             
`u32`   | 4 bytes                | `u32`                             
`u64`   | 8 bytes                | `u64`                             
`u128`  | 16 bytes               | `u128`                            
`usize` | **platform-dependent** | `usize`

# Example

You can notice that `EquisizedPrimitiveUnsignedIntExt` is quite long to type. To make it shorter, you are advised to rename the imported trait as `EPUI`, the namesake for the crate. Because its uses are meant to be accompanied with [fully qualified syntax](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#fully-qualified-syntax-for-disambiguation-calling-methods-with-the-same-name), such shorthand is indispensible.

```rust
  use epui::EquisizedPrimitiveUnsignedIntExt as EPUI;
  let a = -1i8;
  assert_eq!(a as <i8 as EPUI>::EquisizedPrimitiveUnsignedInt, u8::MAX);
```

# Analogues

* C++: [std::make_unsigned](https://en.cppreference.com/w/cpp/types/make_unsigned)

# License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[primitive numeric type]: https://doc.rust-lang.org/reference/types/numeric.html
[core::mem::size_of]: https://doc.rust-lang.org/stable/core/mem/fn.size_of.html