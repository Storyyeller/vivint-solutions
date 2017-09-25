
use std::io::{self, Read};
use std::{thread, time};
use std::collections::HashMap;
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


fn realmain() -> io::Result<()> {
    let input = getinput()?;
    let mut lines = input.lines().rev().collect::<Vec<_>>();

    let (rows, cols) = parse_ints(lines.pop().unwrap());

    let mask = if cols < 64 {(1u64<<cols) - 1} else {u64::max_value()};
    // println!("mask {:0x}", mask);
    let mut counts = HashMap::with_capacity(rows * 2);

    for _ in 0..rows {
        let line = lines.pop().unwrap();

        let mut int = 0;
        for (i, c) in line.bytes().enumerate() {
            if c == b'1' {
                int |= 1u64 << (i / 2);
            }
        }
        // println!("int {:0x} {:0x}", int, mask & !int);

        *counts.entry(int).or_insert(0) += 1;
        *counts.entry(mask & !int).or_insert(0) += 1;
    }

    let (best, score) = counts.into_iter().max_by_key(|&(bits, count)| (count, bits.count_zeros())).unwrap();
    let colinds = (0..cols).filter(|&i| best & (1u64 << i) != 0).map(|i| i.to_string()).collect::<Vec<_>>();

    println!("{}", score);
    println!("{}", colinds.join(" "));


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
