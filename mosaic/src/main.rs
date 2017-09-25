use std::io::{self, Read};
use std::{thread, time};
use std::collections::HashMap;
use std::collections::HashSet;
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

fn parse_ints(line: &str) -> (usize, usize, usize) {
    let ints = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    (ints[0], ints[1], ints[2])
}


fn realmain() -> io::Result<()> {
    let input = getinput()?;
    let mut lines = input.lines();

    let (width, height, num) = parse_ints(lines.next().unwrap());
    lines.next();

    let mosaic = (0..(height*3)).map(|_| lines.next().unwrap()).collect::<Vec<_>>();
    let mut exposed = HashMap::with_capacity(height * width);

    for y in 0..height {
        for x in 0..width {
            let face = mosaic[y*3..y*3+3].iter().map(|&row| &row[x*3..x*3+3]).collect::<Vec<_>>().join("").into_bytes();
            *exposed.entry(face).or_insert(0) += 1;
        }
    }
    let exposed = exposed;

    static FACE_COORDS: [[usize; 9]; 24]  = [[0, 1, 2, 9, 10, 11, 18, 19, 20], [0, 3, 6, 1, 4, 7, 2, 5, 8], [0, 9, 18, 3, 12, 21, 6, 15, 24], [2, 1, 0, 5, 4, 3, 8, 7, 6], [2, 5, 8, 11, 14, 17, 20, 23, 26], [2, 11, 20, 1, 10, 19, 0, 9, 18], [6, 3, 0, 15, 12, 9, 24, 21, 18], [6, 7, 8, 3, 4, 5, 0, 1, 2], [6, 15, 24, 7, 16, 25, 8, 17, 26], [8, 5, 2, 7, 4, 1, 6, 3, 0], [8, 7, 6, 17, 16, 15, 26, 25, 24], [8, 17, 26, 5, 14, 23, 2, 11, 20], [18, 9, 0, 19, 10, 1, 20, 11, 2], [18, 19, 20, 21, 22, 23, 24, 25, 26], [18, 21, 24, 9, 12, 15, 0, 3, 6], [20, 11, 2, 23, 14, 5, 26, 17, 8], [20, 19, 18, 11, 10, 9, 2, 1, 0], [20, 23, 26, 19, 22, 25, 18, 21, 24], [24, 15, 6, 21, 12, 3, 18, 9, 0], [24, 21, 18, 25, 22, 19, 26, 23, 20], [24, 25, 26, 15, 16, 17, 6, 7, 8], [26, 17, 8, 25, 16, 7, 24, 15, 6], [26, 23, 20, 17, 14, 11, 8, 5, 2], [26, 25, 24, 23, 22, 21, 20, 19, 18]];

    for _ in 0..num {
        lines.next();
        let description = (0..3).map(|_| lines.next().unwrap().replace(" ","")).collect::<Vec<_>>().join("").into_bytes();

        let mut sum = 0;
        let mut faces = HashSet::with_capacity(FACE_COORDS.len());
        for &coords in FACE_COORDS.iter() {
            // let face = coords.iter().map(|ind| description[*ind]).collect::<Vec<_>>();
            let face = coords.iter().map(
                    |ind| unsafe{*description.get_unchecked(*ind)}
                ).collect::<Vec<_>>();

            let temp = exposed.get(&face).cloned().unwrap_or(0);
            if faces.insert(face) {
                sum += temp;
            }
        }

        println!("{}", sum);
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
