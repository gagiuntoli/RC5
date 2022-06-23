use std::convert::TryInto;
use num;

macro_rules! rotl {
    ($a: expr, $b: expr) => {
        ($a<<($b&W::from(W::BITS-1))) | ($a>>(W::from(W::BITS)-($b&W::from(W::BITS-1))))
    }
}

macro_rules! rotr {
    ($a: expr, $b: expr) => {
        ($a>>($b&W::from(W::BITS-1))) | ($a<<(W::from(W::BITS)-($b&W::from(W::BITS-1))))
    }
}

// 32/12/16
//const w: u32 = 32; // word size can be 16, 32, 64
//const r: u32 = 12; // number of rounds
//const b: u32 = 16; // key size in bytes
//const c: usize = 4;  // number of words in a key
//const t: u32 = 2*(r+1); // table size (26)
//let S[t]: Word;

// P = 0xb7e1;                 Q = 0x9e37              // w = 16
// P = 0xb7e15163;             Q = 0x9e3779b9          // w = 32
// P = 0xb7e151628aed2a6b;     Q = 0x9e3779b97f4a7c15; // w = 64

trait Unsigned: num::Unsigned +
                num::traits::WrappingAdd +
                std::ops::BitAnd<Output = Self> +
                std::ops::BitOr<Output = Self> +
                std::ops::BitXor<Output = Self> +
                std::ops::Shl<Output = Self> +
                std::ops::Shr<Output = Self>
{
    const BITS: u32;
}

impl Unsigned for u16 {
    const BITS: u32 = u16::BITS;
}

impl Unsigned for u32 {
    const BITS: u32 = u32::BITS;
}

impl Unsigned for u64 {
    const BITS: u32 = u64::BITS;
}

fn encode<W, const T: usize>(key_exp: [W; T], pt: [W; 2]) -> [W; 2]
where
    W: Unsigned + From<u32> + Copy
{
    let r = T/2 - 1;
    let mut a = pt[0] + key_exp[0];
    let mut b = pt[1] + key_exp[1];
    for i in 1..=r {
        a = rotl!(a^b, b) + key_exp[2*i];
        b = rotl!(b^a, a) + key_exp[2*i+1];
    }
    [a,b]
}

fn decode<W, const T: usize>(key_exp: [W; T], ct: [W; 2]) -> [W; 2]
where
    W: Unsigned + From<u32> + Copy
{
    let r = T/2 - 1;
    let mut a = ct[0];
    let mut b = ct[1];
    for i in (1..=r).rev() {
        b = rotr!(b-key_exp[2*i+1], a) ^ a;
        a = rotr!(a-key_exp[2*i]  , b) ^ b;
    }
    [a-key_exp[0], b-key_exp[1]]
}

/*
 * Expands the key to t = 2(r+1) bytes
 */
#[allow(arithmetic_overflow)]
fn expand_key<W, const T: usize>(key: Vec<u8>, p: W, q: W) -> [W;T]
where
    W: Unsigned + From<u8> + From<u32> + std::marker::Copy + std::fmt::Debug
{
    let mut key_s = [W::from(0u8); T];
    let b = key.len();
    let r = T/2 - 1;

    // c = max(1, ceil(8*b/w))
    let c = (std::cmp::max(
            1, (8*key.len() + u32::BITS as usize - 1) as u32 / u32::BITS)
            ) as usize;
    //println!("c = {}\n", c);

    // converting the secrey key from bytes to words
    let mut key_l = [W::from(0u8); 100];
    let u = (W::BITS / 8) as usize;
    for i in (0..=(b-1)).rev() {
        let ix = (i/u) as usize;
        key_l[ix] = (key_l[ix]<<W::from(8u8)).wrapping_add(&W::from(key[i]));
    }
    //println!("L = {:2x?}\n", key_l);
    //println!("key_s = {:?}\n", key_s);
    
    // initializing array S
    key_s[0] = p;
    for i in 1..=(T-1) {
        key_s[i] = key_s[i-1] + q;
    }
    //println!("key_s = {:2x?}\n", key_s);

    // Mixing in the secret key
    let mut i = 0;
    let mut j = 0;
    let mut a = W::from(0u8);
    let mut b = W::from(0u8);
    for k in 0..std::cmp::max(c, 3*T) {
        key_s[i] = rotl!((key_s[i] + (a + b)), W::from(3u8));
        a = key_s[i];
        key_l[j] = rotl!((key_l[j] + (a + b)), (a + b));
        b = key_l[j];
        i = (i+1)%T;
        j = (j+1)%c;
    }

    //println!("key_s = {:2x?}\n", key_s);

    key_s
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn encode_a() {
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                       0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let pt  = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    	let ct  = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
        let key_s = expand_key::<u32,26>(key, 0xb7e15163, 0x9e3779b9);
    	//let res = encode::<u32, 26>(key_s, [0x00112233, 0x44556677]);
    	let res = encode::<u32,26>(key_s, [0x33221100, 0x77665544]);
        println!("{:2x?} {:2x?}", res[0], res[1]);
        println!("{:2x?}", res[0].to_be_bytes().to_vec().into_iter().rev().collect::<Vec<u8>>());
    	//assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_b() {
    	let key = vec![0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00,
                       0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48];
    	let pt: Vec<u8> = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
    	let ct  = vec![0x11, 0xE4, 0x3B, 0x86, 0xD2, 0x31, 0xEA, 0x64];
        let key_s = expand_key::<u32,26>(key, 0xb7e15163, 0x9e3779b9);
        println!("e = {:2x?}", (&pt[0..4]).to_vec().into_iter().rev().collect::<Vec<u8>>());
        //println!("e = {:2x?}", pt[0..4].cloned().into_iter().collect::<Vec<u8>>());
        //println!("e = {:2x}",u32::from_be_bytes(pt[0..4].to_vec().into_iter().rev().try_into().unwrap()));
    	let res = encode::<u32,26>(key_s,
            [u32::from_be_bytes((&pt[0..4]).to_vec().into_iter().rev().collect::<Vec<u8>>().try_into().unwrap()),
             u32::from_be_bytes((&pt[4..8]).to_vec().into_iter().rev().collect::<Vec<u8>>().try_into().unwrap())
            ]);
        println!("encode b {:2x?}", res[0].to_be_bytes().to_vec().into_iter().rev().collect::<Vec<u8>>());
        println!("encode b {:2x?}", res[1].to_be_bytes().to_vec().into_iter().rev().collect::<Vec<u8>>());
    	//assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_c() {
    	let key = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	//let key_exp = [0x00010203, 0x04050607, 0x00010203, 0x04050607];
    	let pt  = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	let ct  = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
        // P = 0xb7e15163;             Q = 0x9e3779b9          // w = 32
        let key_s = expand_key::<u32, 26>(key, 0xb7e15163, 0x9e3779b9);
    	let res = encode::<u32, 26>(key_s, [0x00000000, 0x00000000]);
        println!("{:2x?} {:2x?}", res[0].to_be_bytes(), res[1].to_be_bytes());
    	//assert!(&ct[..] == &res[..]);
    }

    //#[test]
    //fn decode_a() {
    //	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    //	let pt  = vec![0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
    //	let ct  = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    //	let res = decode(key, ct);
    //	assert!(&pt[..] == &res[..]);
    //}

    //#[test]
    //fn decode_b() {
    //	let key = vec![0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48];
    //	let pt  = vec![0x63, 0x8B, 0x3A, 0x5E, 0xF7, 0x2B, 0x66, 0x3F];
    //	let ct  = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
    //	let res = decode(key, ct);
    //	assert!(&pt[..] == &res[..]);
    //}
}

/*
fn encode_32(key: Vec<u8>, pt: [u32; 2]) -> [u32; 2] {
	let mut ciphertext = [0; 2];
    let b = key.len();
    let r = 16;
	type word = u32;
    let mut a = pt[0] + word::from_le_bytes(key[0..4].try_into().unwrap());
    let mut b = pt[1] + word::from_le_bytes(key[4..8].try_into().unwrap());
    for i in 1..=r {
        let ix: usize = 2 * 8 * i;
        a = rotl!(a^b, b, 32) + word::from_le_bytes(key[(ix)..(ix+4)].try_into().unwrap());
        b = rotl!(b^a, a, 32) + word::from_le_bytes(key[(ix+4)..(ix+8)].try_into().unwrap());
    }
    ciphertext[0] = a;
    ciphertext[1] = b;
	ciphertext
}
*/
