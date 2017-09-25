use std::io::{self, Read, Write};
use std::collections::HashMap;
use std::{thread, time};

use std::cmp::{min, max};

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

fn parse_ints(line: &str) -> (usize, usize, usize) {
    let ints = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    (ints[0], ints[1], ints[2])
}

const MISSING: usize = 0xFFFFFFFFFFFFFFFFusize;
struct Data{
    width: usize,
    parents: HashMap<usize, usize>,
    roots: HashMap<usize, (usize, usize, usize, usize)>,
}
impl Data {
    fn add_root(&mut self, y: usize, x1: usize, x2: usize) {
        let key = y * self.width + x1;
        self.roots.insert(key, (x1, y, x2, y));
        self.parents.insert(key, key);
    }

    fn get_root(&mut self, y: usize, x1: usize) -> usize {
        let mut key = y * self.width + x1;
        let mut parent = self.parents[&key];
        while parent != key {
            key = parent;
            parent = self.parents[&key];
        }
        parent
    }

    fn merge(&mut self, root1: usize, root2: usize) -> usize {
        if root1 == root2 {return root1;}

        let mut ra = self.roots[&root1];
        let rb = self.roots[&root2];

        ra.1 = min(ra.1, rb.1);
        ra.0 = min(ra.0, rb.0);
        ra.3 = max(ra.3, rb.3);
        ra.2 = max(ra.2, rb.2);

        self.roots.remove(&root2);
        self.roots.insert(root1, ra);
        self.parents.insert(root2, root1);
        root1
    }
}

fn realmain() -> io::Result<()> {
    let input = getinput()?;

    let mut lines = input.lines().collect::<Vec<_>>();
    let (width, height, tolerance) = parse_ints(lines.remove(0));

    let mut data = Data{
        width,
        parents: HashMap::new(),
        roots: HashMap::new(),
    };


    let mut ranges = Vec::with_capacity(height);
    for (rowi, line) in lines.into_iter().enumerate() {
        let mut cur_ranges = Vec::new();

        let mut start = MISSING;
        let mut last = MISSING;

        for (i, c) in line.bytes().enumerate() {
            if i % 2 == 1 {continue;}
            let i = i / 2;
            if c == b'1' {
                if last == MISSING {
                    start = i;
                    last = i;
                } else if i <= last + tolerance {
                    last = i;
                } else {
                    cur_ranges.push((start, last));
                    data.add_root(rowi, start, last);
                    start = i;
                    last = i;
                }
            }
        }


        if last != MISSING {
            cur_ranges.push((start, last));
            data.add_root(rowi, start, last);
        }
        ranges.push(cur_ranges);
    }
    assert!(ranges.len() == height);

    for yoff in 1..tolerance+1 {
        for (rowi, window) in ranges.windows(yoff + 1).enumerate() {
            let row2i = rowi + yoff;
            let base_ranges = &window[0];
            let target_ranges = &window[yoff];
            let xoff = tolerance - yoff;

            for (start, end) in base_ranges.iter().cloned() {
                let xmin = if start > xoff {start - xoff} else {0};
                let xmax = if end + xoff < width {end + xoff} else {width - 1};
                let mut root = data.get_root(rowi, start);

                // todo: binary search
                for (start2, end2) in target_ranges.iter().cloned() {
                    if end2 < xmin {continue;}
                    if start2 > xmax {break;}

                    let root2 = data.get_root(row2i, start2);
                    root = data.merge(root, root2);
                }
            }
        }
    }

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for info in data.roots.values() {
        let (x1, y1, x2, y2) = *info;
        writeln!(&mut handle, "{} {} {} {}", x1, y1, x2, y2)?;
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
