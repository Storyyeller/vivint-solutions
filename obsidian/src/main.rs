
use std::io::{self, Read};
use std::{thread, time};
// use std::cmp::{min, max};

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_secs(6));
}

fn getinput() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_ints(line: &str) -> (usize, usize) {
    let ints = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    (ints[0], ints[1])
}

#[derive(Clone)]
struct Bits {
    height: usize,
    width: usize,
    bits: Vec<u64>,
    bpos: usize,
}
impl Bits {
    fn new(height: usize, width: usize) -> Self {
        Self{height, width, bits: Vec::with_capacity((height * width)/64 + 1), bpos: 0}
    }

    fn push(&mut self, val: bool) {
        // self.bits.push(val);
        if self.bpos == 0 {
            self.bits.push(0);
        }
        if val {
            *self.bits.last_mut().unwrap() |= 1 << self.bpos;
        }
        self.bpos = (self.bpos + 1) % 64;
    }

    fn get(&self, x: usize, y: usize) -> bool {
        // self.bits[x + y * self.width]
        let ind = (x + y * self.width) / 64;
        let pos = (x + y * self.width) % 64;
        (self.bits[ind] & (1 << pos)) != 0
    }
    fn set(&mut self, x: usize, y: usize) {
        // self.bits[x + y * self.width] = true;
        let ind = (x + y * self.width) / 64;
        let pos = (x + y * self.width) % 64;
        self.bits[ind] |= 1 << pos;
    }

    fn solve(&self, wlens: &[usize]) -> bool {
        if wlens.len() == 0 {
            return true;
        }

        let (first, wlens) = wlens.split_first().unwrap();
        let wlen = *first;

        // horizontal
        if wlen <= self.width {
            for y in 0..self.height {
                for x in 0..(self.width + 1 - wlen) {
                    if (0..wlen).any(|i| self.get(x + i, y)) {
                        continue;
                    }

                    let mut bits = self.clone();
                    for i in 0..wlen {bits.set(x + i, y);}

                    if bits.solve(wlens) {
                        return true;
                    }
                }
            }
        }

        // vertical
        if wlen <= self.height {
            for x in 0..self.width {
                for y in 0..(self.height + 1 - wlen) {
                    if (0..wlen).any(|i| self.get(x, y + i)) {
                        continue;
                    }

                    let mut bits = self.clone();
                    for i in 0..wlen {bits.set(x, y + i);}

                    if bits.solve(wlens) {
                        return true;
                    }
                }
            }
        }

        false
    }
}


fn realmain() -> io::Result<()> {
    let input = getinput()?;
    let mut lines = input.lines().rev().collect::<Vec<_>>();

    let (nweps, nblocks) = parse_ints(lines.pop().unwrap());
    let mut wlens = (0..nweps).map(|_| lines.pop().unwrap().len()).collect::<Vec<_>>();
    wlens.sort();
    wlens.reverse();
    let wlens = wlens;

    for _ in 0..nblocks {
        let (cols, rows) = parse_ints(lines.pop().unwrap());
        let mut bits = Bits::new(rows, cols);

        for _ in 0..rows {
            for c in lines.pop().unwrap().bytes() {
                assert!(c == b'x' || c == b'd');
                bits.push(c == b'x');
            }
        }

        let success = bits.solve(&wlens);
        println!("{}", if success {1} else {0});
    }





    Ok(())
}

fn main() {realmain().unwrap()}

// use std::panic;
// fn main() {
//     panic::catch_unwind(|| {
//         realmain().unwrap()
//     }).map(|x| {
//         // sleep();
//         1
//     }).unwrap_or_else(|x| {
//         sleep();
//         2
//     });
// }
