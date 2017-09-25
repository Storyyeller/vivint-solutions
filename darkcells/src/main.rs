use std::io::{self, Read};
use std::{thread, time};

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_secs(6));
}

#[allow(dead_code)]
fn leak<T>(val: T) -> &'static T {
    let b = Box::new(val);
    let p = Box::into_raw(b);
    unsafe {p.as_ref()}.unwrap()
}


fn getinput() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[derive(Clone, Copy)]
struct ListNode<T: Clone + 'static>(T, List<T>);
type List<T> = Option<&'static ListNode<T>>;

// type List<T> = Option<Box<(T, List<T>)>>;
fn print_list(list: &List<usize>, maxval: usize) {
    let mut counts = vec![0; maxval];
    let mut chain = list.clone();
    while let Some(node) = chain {
        counts[node.0] += 1;
        chain = node.1;
    }
    println!("{}", counts.into_iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
}

fn realmain() -> io::Result<()> {
    let input = getinput()?;
    let lines = input.lines().collect::<Vec<_>>();

    let p: usize = lines[0].parse().unwrap();
    let costs = lines[1].split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();
    let caps = lines[2].split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    let pairs = costs.into_iter().zip(caps.into_iter()).collect::<Vec<_>>();

    // let mut subs = Vec::with_capacity(p + 1);
    let mut subs = Vec::with_capacity(p + 1);
    subs.push((0, None));

    for n in 1..(p+1) {
        assert!(n == subs.len());

        #[allow(non_snake_case)]
        let INVALID: u64 = u64::max_value();
        let mut best = INVALID;
        let mut bestchain = None;

        for (i, (cost, cap)) in pairs.iter().cloned().enumerate() {
            if cap > n {continue;}

            let (scost, schain) = subs[n - cap].clone();
            if scost == INVALID {continue;}

            let cost = cost + scost;
            if best > cost {
                best = cost;
                // bestchain = Some(Box::new(ListNode(i, schain)));
                bestchain = Some(leak(ListNode(i, schain)));
            }
        }
        subs.push((best, bestchain));
    }
    assert!(subs.len() == p+1);

    let (cost, mut chain) = subs.pop().unwrap();
    println!("{}", cost);
    print_list(&chain, pairs.len());
    // let mut counts = vec![0; pairs.len()];
    // while let Some(node) = chain {
    //     counts[node.0] += 1;
    //     chain = node.1;
    // }
    // println!("{}", counts.into_iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" "));
    Ok(())
}

// fn main() {realmain().unwrap()}

use std::panic;
fn main() {
    panic::catch_unwind(|| {
        realmain().unwrap()
    }).map(|_| {
        // sleep();
        1
    }).unwrap_or_else(|_| {
        sleep();
        2
    });
}
