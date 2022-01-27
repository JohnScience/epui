#![no_std]
#![doc = include_str!("../README.md")]

/// Extension trait offering EquisizedPrimitiveUnsignedInt type.
/// 
/// * `<u8 as EquisizedPrimitiveUnsignedIntExt>::EquisizedPrimitiveUnsignedInt == u16`;
/// * `<i16 as EquisizedPrimitiveUnsignedIntExt>::EquisizedPrimitiveUnsignedInt == i32`;
/// * ...
pub trait EquisizedPrimitiveUnsignedIntExt {
    type EquisizedPrimitiveUnsignedInt;
}

impl EquisizedPrimitiveUnsignedIntExt for u8 {
    type EquisizedPrimitiveUnsignedInt = u8;
}

impl EquisizedPrimitiveUnsignedIntExt for u16 {
    type EquisizedPrimitiveUnsignedInt = u16;
}

impl EquisizedPrimitiveUnsignedIntExt for u32 {
    type EquisizedPrimitiveUnsignedInt = u32;
}

impl EquisizedPrimitiveUnsignedIntExt for u64 {
    type EquisizedPrimitiveUnsignedInt = u64;
}

impl EquisizedPrimitiveUnsignedIntExt for u128 {
    type EquisizedPrimitiveUnsignedInt = u128;
}

impl EquisizedPrimitiveUnsignedIntExt for usize {
    type EquisizedPrimitiveUnsignedInt = usize;
}

impl EquisizedPrimitiveUnsignedIntExt for i8 {
    type EquisizedPrimitiveUnsignedInt = u8;
}

impl EquisizedPrimitiveUnsignedIntExt for i16 {
    type EquisizedPrimitiveUnsignedInt = u16;
}

impl EquisizedPrimitiveUnsignedIntExt for i32 {
    type EquisizedPrimitiveUnsignedInt = u32;
}

impl EquisizedPrimitiveUnsignedIntExt for i64 {
    type EquisizedPrimitiveUnsignedInt = u64;
}

impl EquisizedPrimitiveUnsignedIntExt for i128 {
    type EquisizedPrimitiveUnsignedInt = u128;
}

impl EquisizedPrimitiveUnsignedIntExt for isize {
    type EquisizedPrimitiveUnsignedInt = usize;
}