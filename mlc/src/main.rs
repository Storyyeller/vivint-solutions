
use std::io::{self, Read, Write};
use std::{thread, time};

use std::cmp::{min, max};

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

fn parse_ints(line: &str) -> (usize, usize, usize) {
    let ints = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    (ints[0], ints[1], ints[2])
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct Float(f64);
impl Eq for Float {}
use std::cmp::Ordering;
impl Ord for Float {
    fn cmp(&self, other: &Float) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

type FVec = Vec<f64>;

fn dist(v1: &FVec, v2: &FVec) -> Float {
    let mut d = 0.0;
    for (x1, x2) in v1.iter().zip(v2.iter()) {
        d += (x1 - x2) * (x1 - x2);
    }
    Float(d)
}

fn realmain() -> io::Result<()> {
    let input = getinput()?;

    let mut training = Vec::new();
    let mut unknowns = Vec::new();


    for line in input.lines() {
        let mut parts = line.split_whitespace().collect::<Vec<_>>();
        let lbl = parts.pop().unwrap();

        let fv: FVec = parts.into_iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
        if lbl == "?" {
            unknowns.push(fv);
        } else {
            training.push((fv, lbl));
        }
    }

    for fv in unknowns.into_iter() {
        let best = training.iter().min_by_key(|&&(ref fv2, lbl)| dist(&fv, fv2)).unwrap();
        println!("{}", best.1);
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
