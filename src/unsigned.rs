pub trait Unsigned:
    num::Unsigned
    + num::traits::WrappingAdd
    + num::traits::WrappingSub
    + num::traits::WrappingShl
    + num::traits::WrappingShr
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::BitXor<Output = Self>
    + std::ops::Shl<Output = Self>
    + std::ops::Shr<Output = Self>
    + From<u8>
    + Copy
{
    type Array;

    const BITS: Self;
    const BITSU32: u32;
    const BYTES: usize;
    const P: Self;
    const Q: Self;

    fn from_le_bytes(bytes: Self::Array) -> Self;
    fn to_le_bytes(a: Self) -> Vec<u8>;
}

impl Unsigned for u8 {
    type Array = [u8; 1];

    const BITS: Self = u8::BITS as Self;
    const BITSU32: u32 = u8::BITS;
    const BYTES: usize = 1;
    const P: Self = 0xb7u8;
    const Q: Self = 0x9fu8;

    fn from_le_bytes(bytes: Self::Array) -> u8 {
        u8::from_le_bytes(bytes)
    }

    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl Unsigned for u16 {
    type Array = [u8; 2];

    const BITS: Self = u16::BITS as Self;
    const BITSU32: u32 = u16::BITS;
    const BYTES: usize = 2;
    const P: Self = 0xb7e1u16;
    const Q: Self = 0x9e37u16;

    fn from_le_bytes(bytes: Self::Array) -> u16 {
        u16::from_le_bytes(bytes)
    }

    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl Unsigned for u32 {
    type Array = [u8; 4];

    const BITS: Self = u32::BITS as Self;
    const BITSU32: u32 = u32::BITS;
    const BYTES: usize = 4;
    const P: Self = 0xb7e15163u32;
    const Q: Self = 0x9e3779b9u32;

    fn from_le_bytes(bytes: Self::Array) -> u32 {
        u32::from_le_bytes(bytes)
    }

    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl Unsigned for u64 {
    type Array = [u8; 8];

    const BITS: Self = u64::BITS as Self;
    const BITSU32: u32 = u64::BITS;
    const BYTES: usize = 8;
    const P: Self = 0xb7e151628aed2a6bu64;
    const Q: Self = 0x9e3779b97f4a7c15u64;

    fn from_le_bytes(bytes: Self::Array) -> u64 {
        u64::from_le_bytes(bytes)
    }

    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}

impl Unsigned for u128 {
    type Array = [u8; 16];

    const BITS: Self = u128::BITS as Self;
    const BITSU32: u32 = u128::BITS;
    const BYTES: usize = 16;
    const P: Self = 0xb7e151628aed2a6abf7158809cf4f3c7u128;
    const Q: Self = 0x9e3779b97f4a7c15f39cc0605cedc835u128;

    fn from_le_bytes(bytes: Self::Array) -> u128 {
        u128::from_le_bytes(bytes)
    }

    fn to_le_bytes(a: Self) -> Vec<u8> {
        a.to_le_bytes().to_vec()
    }
}
