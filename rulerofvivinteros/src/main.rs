
use std::io::{self, Read};
use std::{thread, time};
use std::cmp::{min, max};

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_secs(6));
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

    let (n, c) = parse_ints(&input);
    let pn = 1usize << n; // n <= 8, but might as well use longer words for simplicity

    let mut throws0 = vec![0.0; pn];
    throws0[0] = 1.0;

    let mut throws = vec![throws0];
    while throws.len() <= n {
        let mut new = vec![0.0; pn];

        for (mask, p) in throws.last().unwrap().iter().enumerate() {
            for result in 0..n {
                new[mask | (1<<result)] += *p;
            }
        }

        for ptr in new.iter_mut() {
            *ptr /= n as f64;
        }
        throws.push(new);
    }

    let mut old = vec![0.0; n+1];
    old[n] = 1.0;

    for _ in 0..c {
        let mut new = vec![0.0; n+1];

        for count in 0..n+1 {
            let numdice = n - count;
            for (mask, p) in throws[numdice].iter().enumerate() {
                let mask = mask | ((1<<count) - 1);
                let count2 = mask.count_ones();
                new[count] += p * old[count2 as usize];
            }
        }

        let mut best = Float(0.0);
        for ptr in new.iter_mut() {
            best = max(best, Float(*ptr));
            *ptr = best.0;
        }
        old = new;
    }


    println!("{:.4}", old[0] * 100.0);
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
