use std;
use std::fmt;

#[allow(dead_code)]
fn as_addr<T: Sized>(p: Option<&T>) -> usize {p.map_or(std::ptr::null(), |x| x) as usize}

#[allow(dead_code)]
pub fn forget<T>(val: T) -> &'static T {
    let b = Box::new(val);
    let p = Box::into_raw(b);
    unsafe {p.as_ref()}.unwrap()
}


type Word = u64;
// const LIMIT: Word = 1000000000000000000;
// const NDIGITS: usize = 18; // decimal digits per word of BigInt

// For decimal -> binary
const NDIGITS: usize = 19;
const FRAC: Word = 152587890625000;
const TWO_16: Word = 65536;

fn get_16bits(dwords: &mut Vec<u64>) -> u64 {
    let mut carry = 0;
    for dword in dwords.iter_mut().rev() {
        let newcarry = *dword % TWO_16;
        *dword = (*dword / TWO_16) + carry * FRAC;
        carry = newcarry;
    }

    if dwords.last().cloned() == Some(0) {
        dwords.pop();
    }
    return carry;
}

#[derive(Clone)]
pub struct BigInt {
    words: Vec<Word>,
}
impl BigInt {
    pub fn add(&mut self, other: Self) {
        let mut carry = false;
        let mut i = 0;

        while i < other.words.len() || carry {
            let x = self.words.get(i).cloned().unwrap_or(0);
            let y = other.words.get(i).cloned().unwrap_or(0);

            let (x, c1) = x.overflowing_add(y);
            let (x, c2) = x.overflowing_add(if carry {1} else {0});
            carry = c1 || c2;

            if i == self.words.len() {
                self.words.push(x);
            } else {
                self.words[i] = x;
            }
            i += 1;
        }
    }

    pub fn cmp(&self, other: &Self) -> i32 {
        if self.words.len() < other.words.len() {
            return -1;
        } else if self.words.len() > other.words.len() {
            return 1;
        }

        for (x, y) in self.words.iter().zip(other.words.iter()).rev() {
            if x < y {
                return -1;
            } else if x > y {
                return 1;
            }
        }
        return 0;
    }

    pub fn parse(s: &str) -> Result<Self, std::num::ParseIntError> {
        let mut s = s.trim_left_matches('0');
        let mut parts = Vec::with_capacity(1 + s.len() / NDIGITS);

        while s.len() > NDIGITS {
            let (s1, s2) = s.split_at(s.len() - NDIGITS);
            s = s1;
            parts.push(s2.parse()?);
        }
        if s.len() > 0 {
            parts.push(s.parse()?);
        }

        let mut words = Vec::with_capacity(parts.len());
        while parts.len() > 0 {
            let p0 = get_16bits(&mut parts);
            let p1 = get_16bits(&mut parts);
            let p2 = get_16bits(&mut parts);
            let p3 = get_16bits(&mut parts);
            words.push(p0 | (p1 << 16) | (p2 << 32) | (p3 << 48));
        }
        if words.last().cloned() == Some(0) {
            words.pop();
        }

        Ok(BigInt{words})
    }
}


// For binary -> decimal
const TEN_14: u64 = 100000000000000;
fn shift_dec_vec(dwords: &mut Vec<u64>) {
    let mut carry = 0;
    for dword in dwords.iter_mut() {
        let temp = (*dword * TWO_16) + carry;
        carry = temp / TEN_14;
        *dword = temp % TEN_14;
    }
    if carry != 0 {
        dwords.push(carry);
    }
}

fn add_dec_vec(add: u64, dwords: &mut Vec<u64>) {
    let mut carry = add;
    for dword in dwords.iter_mut() {
        let temp = *dword + carry;
        carry = temp / TEN_14;
        *dword = temp % TEN_14;
    }
    if carry != 0 {
        dwords.push(carry);
    }
}
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.words.len() == 0 {
            return write!(f, "0");
        }

        let mut dvec = Vec::with_capacity((self.words.len() as f64 * 1.37614) as usize + 1);

        // fill dvec with array of 14 digit groups
        for word in self.words.iter().rev() {
            shift_dec_vec(&mut dvec);
            add_dec_vec((word >> 48) % TWO_16, &mut dvec);
            shift_dec_vec(&mut dvec);
            add_dec_vec((word >> 32) % TWO_16, &mut dvec);
            shift_dec_vec(&mut dvec);
            add_dec_vec((word >> 16) % TWO_16, &mut dvec);
            shift_dec_vec(&mut dvec);
            add_dec_vec((word >> 0) % TWO_16, &mut dvec);
        }

        for (i, word) in dvec.into_iter().rev().enumerate() {
            if i == 0 {
                write!(f, "{}", word)?;
            } else {
                write!(f, "{:014}", word)?;
            }
        }
        Ok(())
    }
}
