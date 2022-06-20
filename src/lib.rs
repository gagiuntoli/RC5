use core::convert::TryInto;
use std::ops;
extern crate num;

macro_rules! rotl {
    ($a: expr, $b: expr, $w: expr, $w_minus_1: expr) => {
        ($a<<($b&($w_minus_1))) | ($a>>($w-($b&($w_minus_1))))
    }
}

macro_rules! rotr {
    ($a: expr, $b: expr, $w: expr) => {
        ($a>>($b&($w-1))) | ($a<<($w-($b&($w-1))))
    }
}

// 32/12/16
//const w: u32 = 32; // word size can be 16, 32, 64
//const r: u32 = 12; // number of rounds
//const b: u32 = 16; // key size in bytes
//const c: usize = 4;  // number of words in a key
//const t: u32 = 2*(r+1); // table size (26)
//let S[t]: Word;

    /*
     * Expands the key to t = 2(r+1) bytes
     */
//    fn expand_key(key: Vec<u8>) -> [u8; T] {
//        // P = 0xb7e1;                 Q = 0x9e37              // w = 16
//        // P = 0xb7e15163;             Q = 0x9e3779b9          // w = 32
//        // P = 0xb7e151628aed2a6b;     Q = 0x9e3779b97f4a7c15; // w = 64
//        let mut L: [W; c];
//        const u: u32 = w/4;
//        let mut ekey = [0u8; T];
//        L[c-1] = 0;
//        for i in (0..=(b-1)).rev() {
//            println!("{}",i);
//            //L[(i/u) as usize] = (L[(i/u) as usize] << 8) + key[i as usize];
//        }
//        return ekey
//    }
//}

fn encode<W, const T: usize>(key_exp: [W; T], pt: [W; 2]) -> [W; 2]
where
    W:
    ops::Add<Output = W> +
    ops::Sub<Output = W> +
    ops::BitAnd<Output = W> +
    ops::BitXor<Output = W> +
    ops::Shl<Output = W> +
    ops::Shr<Output = W> +
    ops::BitOr<Output = W> +
    Copy + 
    From<u32>
{
    let r = T/2 - 1;
    let mut a = pt[0] + key_exp[0];
    let mut b = pt[1] + key_exp[1];
    for i in 1..=r {
        a = rotl!(a^b, b, W::from(32), W::from(31)) + key_exp[2*i];
        b = rotl!(b^a, a, W::from(32), W::from(31)) + key_exp[2*i+1];
    }
    [a,b]
}

fn expand_key<W, const T: usize>(key: Vec<u8>) -> [W;T]
where
    W: From<u32> +
    std::marker::Copy
{
    let mut key_exp = [W::from(0); T];
    key_exp
}

/*
 * This function should return a plaintext for a given key and ciphertext
 *
 */
fn decode(key: Vec<u8>, ciphertext: Vec<u8>) -> Vec<u8> {
	let mut plaintext = Vec::new();
	plaintext
}

#[cfg(test)]
mod tests {
	use super::*;

    //#[test]
    //fn rotl_a() {
    //    let a = rotl!(1,1,32);
    //    println!("a = {}", a);
    //}

    #[test]
    fn encode_a() {
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let key_exp = [0x00010203, 0x04050607, 0x00010203, 0x04050607];
    	let pt  = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    	let ct  = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
    	let res = encode::<u32, 4>(key_exp, [0x0001020304, 0x05060708]);
    	assert!(&ct[..] == &res[..]);
    }

    //#[test]
    //fn encode_b() {
    //	let key = vec![0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48];
    //	let pt  = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
    //	let ct  = vec![0x11, 0xE4, 0x3B, 0x86, 0xD2, 0x31, 0xEA, 0x64];
    //	let res = encode::<u32>(key, pt);
    //	assert!(&ct[..] == &res[..]);
    //}

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
