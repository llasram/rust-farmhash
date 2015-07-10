// Based on the original C++ farmhashcc.cc

// This file provides a 32-bit hash equivalent to CityHash32 (v1.1.1)
// and a 128-bit hash equivalent to CityHash128 (v1.1.1).  It also provides
// a seeded 32-bit hash function similar to CityHash32.

use platform::*;
use farmhashna_shared::*;

fn hash32_len_13_to_24(s: &[u8]) -> u32 {
    let len = s.len() as usize;
    let a = fetch32(&s[-4+(len>>1 as u64)..]);
    let b = fetch32(&s[4..]);
    let c = fetch32(&s[len-8..]);
    let d = fetch32(&s[len>>1..]);
    let e = fetch32(&s[0..]);
    let f = fetch32(&s[len-4..]);
    let h = len as u32;
    return fmix(mur(f, mur(e, mur(d, mur(c, mur(b, mur(a, h)))))));
}

fn hash32_len_0_to_4(s: &[u8]) -> u32 {
    let len = s.len() as usize;
    let mut b: u32 = 0;
    let mut c: u32 = 9;
    for i in 0..len {
        let v: u8 = s[i];
        b = b.wrapping_mul(C1) + v as u32;
        c ^= b;
    }
    return fmix(mur(b, mur(len as u32, c)));
}

fn hash32_len_5_to_12(s: &[u8]) -> u32 {
    let len = s.len() as usize;
    let mut a = len as u32;
    let mut b = len as u32 * 5;
    let mut c: u32 = 9;
    let d: u32 = b;

    a += fetch32(&s[0..]);
    b += fetch32(&s[len-4..]);
    c += fetch32(&s[((len >> 1) & 4)..]);
    return fmix(mur(c, mur(b, mur(a, d))));
}


pub fn hash32(mut s: &[u8]) ->u32 {
    let len = s.len() as usize;
    if len <= 24 {
        if len <= 12 {
            if len <= 4 {
                return hash32_len_0_to_4(s)
            }
            return hash32_len_5_to_12(s)
        }
        return hash32_len_13_to_24(s)
    }

    // len > 24
    let mut h = len as u32;
    let mut g = (len as u32).wrapping_mul(C1);
    let mut f = g;
    let a0 = rotate32(fetch32(&s[(len-4)..]).wrapping_mul(C1), 17) * C2;
    let a1 = rotate32(fetch32(&s[(len-8)..]).wrapping_mul(C1), 17) * C2;
    let a2 = rotate32(fetch32(&s[(len-16)..]).wrapping_mul(C1), 17) * C2;
    let a3 = rotate32(fetch32(&s[(len-12)..]).wrapping_mul(C1), 17) * C2;
    let a4 = rotate32(fetch32(&s[(len-20)..]).wrapping_mul(C1), 17) * C2;
    h ^= a0;
    h = rotate32(h, 19);
    h = (h*5).wrapping_add(0xe6546b64);
    h ^= a2;
    h = rotate32(h, 19);
    h = (h*5).wrapping_add(0xe6546b64);
    g ^= a1;
    g = rotate32(g, 19);
    g = (g*5).wrapping_add(0xe6546b64);
    g ^= a3;
    g = rotate32(g, 19);
    g = (g*5).wrapping_add(0xe6546b64);
    f += a4;
    f = rotate32(f, 19);
    f = (f*5).wrapping_add(0xe6546b64);

    let mut iters = ((len - 1) / 20) as u64;
    while iters > 0{
        let a0 = rotate32(fetch32(&s[..]).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a1 = fetch32(&s[4..]);
        let a2 = rotate32(fetch32(&s[8..]).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a3 = rotate32(fetch32(&s[12..]).wrapping_mul(C1), 17).wrapping_mul(C2);
        let a4 = fetch32(&s[16..]);
        h ^= a0;
        h = rotate32(h, 18);
        h = (h*5).wrapping_add(0xe6546b64);
        f += a1;
        f = rotate32(f, 19);
        f = f.wrapping_mul(C1);
        g += a2;
        g = rotate32(g, 18);
        g = (g*5).wrapping_add(0xe6546b64);
        h ^= a3 + a1;
        h = rotate32(h, 19);
        h = (h*5).wrapping_add(0xe6546b64);
        g ^= a4;
        g = bswap32(g) * 5;
        h += a4 * 5;
        h = bswap32(h);
        f += a0;
        //PERMUTE3(f, h, g) - swap(a,b);swap(b,c)
        let mut t = h;
        h = f;
        f = t;

        t = g;
        g = f;
        f = t;
        s = &s[20..];
        iters-=1;
    }
    g = rotate32(g, 11).wrapping_mul(C1);
    g = rotate32(g, 17).wrapping_mul(C1);
    f = rotate32(f, 11).wrapping_mul(C1);
    f = rotate32(f, 17).wrapping_mul(C1);
    h = rotate32(h+g, 19);
    h = h*5 + 0xe6546b64;
    h = rotate32(h, 17).wrapping_mul(C1);
    h = rotate32(h+f, 19);
    h = h*5 + 0xe6546b64;
    h = rotate32(h, 17).wrapping_mul(C1);
    return h
}

// Return a 16-byte hash for 48 bytes.  Quick and dirty.
// Callers do best to use "random-looking" values for a and b.
// Note: C++ returned pair<uint64_t, uint64_t>
pub fn weak_hash_len_32_with_seeds(w: u64, x: u64, y:u64, z:u64, mut a:u64, mut b:u64) -> Uint128 {
    a = a.wrapping_add(w);
    b = rotate64(b.wrapping_add(a).wrapping_add(z), 21);
    let c = a;
    a = a.wrapping_add(x);
    a = a.wrapping_add(y);
    b = b.wrapping_add(rotate64(a, 44));
    return Uint128{first: a.wrapping_add(z), second:b.wrapping_add(c)};
}

// Return a 16-byte hash for s[0] ... s[31], a, and b.  Quick and dirty.
// Note: original C++ returned a pair<uint64_t, uint64_t>
pub fn weak_hash_len_32_with_seeds_bytes(s: &[u8], mut a:u64, mut b:u64) -> Uint128 {
    return weak_hash_len_32_with_seeds(fetch64(s),
        fetch64(&s[8..]),
        fetch64(&s[16..]),
        fetch64(&s[24..]),
        a,
        b);
}

pub fn hash_len_16_no_mul(u: u64, v: u64) -> u64 {
    return hash128to64(Uint128{first: u, second: v});
}

// Note: avoid clashing with the same names in farmhashna by adding cc to start

pub fn cc_hash_len_16(u: u64, v: u64, mul: u64) -> u64 {
    // Murmur-inspired hashing.
    let mut a = (u ^ v) * mul;
    a ^= (a >> 47);
    let mut b = (v ^ a) * mul;
    b ^= (b >> 47);
    b *= mul;
    return b;
}

pub fn cc_hash_len_0_to_16(s: &[u8]) -> u64 {
    let len = s.len() as usize;
    if len >= 8 {
        let mul = ((len as u64) *2).wrapping_mul(K2);
        let a = fetch64(&s).wrapping_add(K2);
        let b = fetch64(&s[len-8..]);
        let c = rotate64(b, 37).wrapping_mul(mul) + a;
        let d = (rotate64(a, 25) + b).wrapping_mul(mul);
        return cc_hash_len_16(c, d, mul)
    }
    if len >= 4 {
        let mul = ((len as u64) *2).wrapping_mul(K2);
        let a = fetch32(s) as u64;
        return cc_hash_len_16((len as u64)+(a<<3), fetch32(&s[len-4..]) as u64, mul)
    }
    if len > 0 {
        let a = s[0];
        let b = s[len>>1];
        let c = s[len-1];
        let y = (a as u32) + ((b as u32) << 8);
        let z = (len as u32) + ((c as u32) << 2);
        // FIXME: Check if order is right!
        return shift_mix((y as u64).wrapping_mul(K2)^(z as u64).wrapping_mul(K0)).wrapping_mul(K2);
    }
    return K2
}

// A subroutine for CityHash128().  Returns a decent 128-bit hash for strings
// of any length representable in signed long.  Based on City and Murmur.
pub fn city_murmur(mut s: &[u8], seed: Uint128) -> Uint128 {
    let len = s.len() as usize;
    let mut a = seed.first;
    let mut b = seed.second;
    let mut c: u64 = 0;
    let mut d: u64 = 0;
    let mut l = (len as u64) - 16;
    if l <= 0 { // len <= 16
        a = shift_mix(a.wrapping_mul(K1)).wrapping_mul(K1);
        c = b.wrapping_mul(K1) + cc_hash_len_0_to_16(s);
        if len >= 8 {
            d = shift_mix(a + fetch64(s))
        } else {
            d = shift_mix(a + c)
        }
    } else { // len > 16
        c = hash_len_16_no_mul(fetch64(&s[(len-8)..]).wrapping_mul(K1), a);
        d = hash_len_16_no_mul(b+(len as u64), c+fetch64(&s[len-16..]));
        a += d;
        while l > 0 {
            a ^= shift_mix(fetch64(s).wrapping_mul(K1)).wrapping_mul(K1);
            a = a.wrapping_mul(K1);
            b ^= a;
            c ^= shift_mix(fetch64(&s[8..]).wrapping_mul(K1)).wrapping_mul(K1);
            c = c.wrapping_mul(K1);
            d ^= c;
            s = &s[16..];
            l -= 16;
        }
    }
    a = hash_len_16_no_mul(a, c);
    b = hash_len_16_no_mul(d, b);
    return Uint128{first: a ^ b, second: hash_len_16_no_mul(b, a)}
}
