use std::convert::TryInto;

mod unsigned;
use unsigned::Unsigned;

macro_rules! rotl {
    ($a: expr, $b: expr) => {
        ($a<<($b&(W::BITS-W::ONE))) | ($a>>((W::BITS)-($b&(W::BITS-W::ONE))))
    }
}

macro_rules! rotr {
    ($a: expr, $b: expr) => {
        ($a>>($b&(W::BITS-W::ONE))) | ($a<<((W::BITS)-($b&(W::BITS-W::ONE))))
    }
}

pub fn encode<W, const T: usize>(key: Vec<u8>, pt: Vec<u8>) -> Vec<u8>
    where W: Unsigned
{
    let key_exp = expand_key::<W,T>(key);
    let r = T/2-1;
    let mut a = W::from_le_bytes(pt[0..W::BYTES].try_into().unwrap()).unwrap() + key_exp[0];
    let mut b = W::from_le_bytes(pt[W::BYTES..2*W::BYTES].try_into().unwrap()).unwrap() + key_exp[1];
    for i in 1..=r {
        a = rotl!(a^b, b) + key_exp[2*i];
        b = rotl!(b^a, a) + key_exp[2*i+1];
    }
    [W::to_le_bytes(a).as_slice(), W::to_le_bytes(b).as_slice()].concat()
}

pub fn decode<W, const T: usize>(key: Vec<u8>, ct: Vec<u8>) -> Vec<u8>
    where W: Unsigned
{
    let key_exp = expand_key::<W,T>(key);
    let r = T/2 - 1;
    let mut a = W::from_le_bytes(ct[0..W::BYTES].try_into().unwrap()).unwrap();
    let mut b = W::from_le_bytes(ct[W::BYTES..2*W::BYTES].try_into().unwrap()).unwrap();
    for i in (1..=r).rev() {
        b = rotr!(b-key_exp[2*i+1], a) ^ a;
        a = rotr!(a-key_exp[2*i]  , b) ^ b;
    }
    [W::to_le_bytes(a-key_exp[0]).as_slice(), W::to_le_bytes(b-key_exp[1]).as_slice()].concat()
}

/*
 * Expands the key to t = 2(r+1) bytes
 */
pub fn expand_key<W, const T: usize>(key: Vec<u8>) -> [W;T]
    where W: Unsigned
{
    let mut key_s = [W::ZERO; T];
    let b = key.len();

    // c = max(1, ceil(8*b/w))
    let c = (std::cmp::max(
            1, (8*key.len() + (W::BITSU32 - 1) as usize) as u32 / W::BITSU32
            )) as usize;

    // converting the secrey key from bytes to words
    let mut key_l = vec![W::ZERO; c];
    let u = W::BYTES as usize;
    for i in (0..=(b-1)).rev() {
        let ix = (i/u) as usize;
        key_l[ix] = (key_l[ix]<<W::EIGHT) + W::from(key[i]);
    }
    
    // initializing array S
    key_s[0] = W::P;
    for i in 1..=(T-1) {
        key_s[i] = key_s[i-1] + W::Q;
    }

    // Mixing in the secret key
    let mut i = 0;
    let mut j = 0;
    let mut a = W::ZERO;
    let mut b = W::ZERO;
    for _k in 0..3*std::cmp::max(c, T) {
        key_s[i] = rotl!((key_s[i] + (a + b)), W::THREE);
        a = key_s[i];
        key_l[j] = rotl!((key_l[j] + (a + b)), (a + b));
        b = key_l[j];
        i = (i+1)%T;
        j = (j+1)%c;
    }

    key_s
}


#[cfg(test)]
mod tests {
	use super::*;

    /* Mintlayer tests */

    #[test]
    fn encode_a() {
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let pt  = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    	let ct  = vec![0x2D, 0xDC, 0x14, 0x9B, 0xCF, 0x08, 0x8B, 0x9E];
    	let res = encode::<u32,26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_b() {
    	let key = vec![0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48];
    	let pt = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
    	let ct = vec![0x11, 0xE4, 0x3B, 0x86, 0xD2, 0x31, 0xEA, 0x64];
    	let res = encode::<u32,26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_c() {
    	let key = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	let pt  = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	let ct  = vec![0x21, 0xA5, 0xDB, 0xEE, 0x15, 0x4B, 0x8F, 0x6D];
    	let res = encode::<u32, 26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_d() {
    	let key = vec![0x91, 0x5F, 0x46, 0x19, 0xBE, 0x41, 0xB2, 0x51, 0x63, 0x55, 0xA5, 0x01, 0x10, 0xA9, 0xCE, 0x91];
    	let pt  = vec![0x21, 0xA5, 0xDB, 0xEE, 0x15, 0x4B, 0x8F, 0x6D];
    	let ct  = vec![0xF7, 0xC0, 0x13, 0xAC, 0x5B, 0x2B, 0x89, 0x52];
    	let res = encode::<u32, 26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_e() {
    	let key = vec![0x78, 0x33, 0x48, 0xE7, 0x5A, 0xEB, 0x0F, 0x2F, 0xD7, 0xB1, 0x69, 0xBB, 0x8D, 0xC1, 0x67, 0x87];
    	let pt  = vec![0xF7, 0xC0, 0x13, 0xAC, 0x5B, 0x2B, 0x89, 0x52];
    	let ct  = vec![0x2F, 0x42, 0xB3, 0xB7, 0x03, 0x69, 0xFC, 0x92];
    	let res = encode::<u32, 26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_f() {
    	let key = vec![0xDC, 0x49, 0xDB, 0x13, 0x75, 0xA5, 0x58, 0x4F, 0x64, 0x85, 0xB4, 0x13, 0xB5, 0xF1, 0x2B, 0xAF];
    	let pt  = vec![0x2F, 0x42, 0xB3, 0xB7, 0x03, 0x69, 0xFC, 0x92];
    	let ct  = vec![0x65, 0xC1, 0x78, 0xB2, 0x84, 0xD1, 0x97, 0xCC];
    	let res = encode::<u32, 26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_a() {
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let pt  = vec![0x96, 0x95, 0x0D, 0xDA, 0x65, 0x4A, 0x3D, 0x62];
    	let ct  = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_b() {
    	let key = vec![0x2B, 0xD6, 0x45, 0x9F, 0x82, 0xC5, 0xB3, 0x00, 0x95, 0x2C, 0x49, 0x10, 0x48, 0x81, 0xFF, 0x48];
    	let pt  = vec![0x63, 0x8B, 0x3A, 0x5E, 0xF7, 0x2B, 0x66, 0x3F];
    	let ct  = vec![0xEA, 0x02, 0x47, 0x14, 0xAD, 0x5C, 0x4D, 0x84];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_c() {
    	let key = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	let pt  = vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    	let ct  = vec![0x21, 0xA5, 0xDB, 0xEE, 0x15, 0x4B, 0x8F, 0x6D];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_d() {
    	let key = vec![0x91, 0x5F, 0x46, 0x19, 0xBE, 0x41, 0xB2, 0x51, 0x63, 0x55, 0xA5, 0x01, 0x10, 0xA9, 0xCE, 0x91];
    	let pt  = vec![0x21, 0xA5, 0xDB, 0xEE, 0x15, 0x4B, 0x8F, 0x6D];
    	let ct  = vec![0xF7, 0xC0, 0x13, 0xAC, 0x5B, 0x2B, 0x89, 0x52];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_e() {
    	let key = vec![0x78, 0x33, 0x48, 0xE7, 0x5A, 0xEB, 0x0F, 0x2F, 0xD7, 0xB1, 0x69, 0xBB, 0x8D, 0xC1, 0x67, 0x87];
    	let pt  = vec![0xF7, 0xC0, 0x13, 0xAC, 0x5B, 0x2B, 0x89, 0x52];
    	let ct  = vec![0x2F, 0x42, 0xB3, 0xB7, 0x03, 0x69, 0xFC, 0x92];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_f() {
    	let key = vec![0xDC, 0x49, 0xDB, 0x13, 0x75, 0xA5, 0x58, 0x4F, 0x64, 0x85, 0xB4, 0x13, 0xB5, 0xF1, 0x2B, 0xAF];
    	let pt  = vec![0x2F, 0x42, 0xB3, 0xB7, 0x03, 0x69, 0xFC, 0x92];
    	let ct  = vec![0x65, 0xC1, 0x78, 0xB2, 0x84, 0xD1, 0x97, 0xCC];
    	let res = decode::<u32, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    /* Test cases from
       https://tools.ietf.org/id/draft-krovetz-rc6-rc5-vectors-00.html#rfc.section.4
       */

    #[test]
    fn encode_8_12_4() {
        // RC5-8/12/4
        // Key:          00010203
        // Block input:  0001
        // Block output: 212A
    	let key = vec![0x00, 0x01, 0x02, 0x03];
    	let pt  = vec![0x00, 0x01];
    	let ct  = vec![0x21, 0x2A];
    	let res = encode::<u8, 26>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_16_16_8() {
        // RC5-16/16/8
        // Key:          0001020304050607
        // Block input:  00010203
        // Block output: 23A8D72E
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03];
    	let ct  = vec![0x23, 0xA8, 0xD7, 0x2E];
    	let res = encode::<u16, 34>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_32_20_16() {
        // RC5-32/20/16
        // Key:          000102030405060708090A0B0C0D0E0F
        // Block input:  0001020304050607
        // Block output: 2A0EDC0E9431FF73
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    	let ct  = vec![0x2A, 0x0E, 0xDC, 0x0E, 0x94, 0x31, 0xFF, 0x73
];
    	let res = encode::<u32, 42>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_64_24_24() {
        // RC5-64/24/24
        // Key:          000102030405060708090A0B0C0D0E0F1011121314151617
        // Block input:  000102030405060708090A0B0C0D0E0F
        // Block output: A46772820EDBCE0235ABEA32AE7178DA
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let ct  = vec![0xA4, 0x67, 0x72, 0x82, 0x0E, 0xDB, 0xCE, 0x02, 0x35, 0xAB, 0xEA, 0x32, 0xAE, 0x71, 0x78, 0xDA];
    	let res = encode::<u64, 50>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn encode_128_28_32() {
        // RC5-128/28/32
        // Key:          000102030405060708090A0B0C0D0E0F
        //               101112131415161718191A1B1C1D1E1F
        // Block input:  000102030405060708090A0B0C0D0E0F
        //               101112131415161718191A1B1C1D1E1F
        // Block output: ECA5910921A4F4CFDD7AD7AD20A1FCBA
        //               068EC7A7CD752D68FE914B7FE180B440
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];
    	let ct  = vec![0xEC, 0xA5, 0x91, 0x09, 0x21, 0xA4, 0xF4, 0xCF, 0xDD, 0x7A, 0xD7, 0xAD, 0x20, 0xA1, 0xFC, 0xBA,
                       0x06, 0x8E, 0xC7, 0xA7, 0xCD, 0x75, 0x2D, 0x68, 0xFE, 0x91, 0x4B, 0x7F, 0xE1, 0x80, 0xB4, 0x40];
    	let res = encode::<u128, 58>(key, pt);
    	assert!(&ct[..] == &res[..]);
    }

    #[test]
    fn decode_16_16_8() {
        // RC5-16/16/8
        // Key:          0001020304050607
        // Block input:  00010203
        // Block output: 23A8D72E
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03];
    	let ct  = vec![0x23, 0xA8, 0xD7, 0x2E];
    	let res = decode::<u16, 34>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_8_12_4() {
        // Key:          00010203
        // Block input:  0001
        // Block output: 212A
    	let key = vec![0x00, 0x01, 0x02, 0x03];
    	let pt  = vec![0x00, 0x01];
    	let ct  = vec![0x21, 0x2A];
    	let res = decode::<u8, 26>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_32_20_16() {
        // RC5-32/20/16
        // Key:          000102030405060708090A0B0C0D0E0F
        // Block input:  0001020304050607
        // Block output: 2A0EDC0E9431FF73
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    	let ct  = vec![0x2A, 0x0E, 0xDC, 0x0E, 0x94, 0x31, 0xFF, 0x73
];
    	let res = decode::<u32, 42>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_64_24_24() {
        // RC5-64/24/24
        // Key:          000102030405060708090A0B0C0D0E0F1011121314151617
        // Block input:  000102030405060708090A0B0C0D0E0F
        // Block output: A46772820EDBCE0235ABEA32AE7178DA
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F];
    	let ct  = vec![0xA4, 0x67, 0x72, 0x82, 0x0E, 0xDB, 0xCE, 0x02, 0x35, 0xAB, 0xEA, 0x32, 0xAE, 0x71, 0x78, 0xDA];
    	let res = decode::<u64, 50>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }

    #[test]
    fn decode_128_28_32() {
        // RC5-128/28/32
        // Key:          000102030405060708090A0B0C0D0E0F
        //               101112131415161718191A1B1C1D1E1F
        // Block input:  000102030405060708090A0B0C0D0E0F
        //               101112131415161718191A1B1C1D1E1F
        // Block output: ECA5910921A4F4CFDD7AD7AD20A1FCBA
        //               068EC7A7CD752D68FE914B7FE180B440
    	let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];
    	let pt  = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
                       0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F];
    	let ct  = vec![0xEC, 0xA5, 0x91, 0x09, 0x21, 0xA4, 0xF4, 0xCF, 0xDD, 0x7A, 0xD7, 0xAD, 0x20, 0xA1, 0xFC, 0xBA,
                       0x06, 0x8E, 0xC7, 0xA7, 0xCD, 0x75, 0x2D, 0x68, 0xFE, 0x91, 0x4B, 0x7F, 0xE1, 0x80, 0xB4, 0x40];
    	let res = decode::<u128, 58>(key, ct);
    	assert!(&pt[..] == &res[..]);
    }
}

