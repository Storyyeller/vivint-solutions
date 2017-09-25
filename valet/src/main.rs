use std::io::{self, Read};
use std::collections::HashMap;
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

fn parse_ints(line: &str) -> Vec<u64> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

fn parse_ints2(line: &str) -> (u64, u64) {
    let ints = line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();
    (ints[0], ints[1])
}




#[derive(Clone, Copy)]
struct ListNode<T: Clone + 'static>(T, List<T>);
type List<T> = Option<&'static ListNode<T>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State(Vec<(u64, u64)>);
impl State {
    fn advance(mut self, time: u64) -> Self {
        self.0.retain(|&(dur, _)| dur > time);
        for &mut (ref mut dur, _) in self.0.iter_mut() {
            *dur -= time;
        }
        self
    }

    fn insert(&self, dur: u64, lvl: u64) -> Self {
        let mut new = self.clone();
        new.0.push((dur, lvl));
        new.0.sort();
        new
    }
}

struct StateBests{
    d: HashMap<State, (u64, List<(u64, u64)>)>
}
impl StateBests {
    fn new() -> Self {Self{d: HashMap::new()}}

    fn update_best(&mut self, key: State, val: (u64, List<(u64, u64)>)) {
        // todo - optimize lookup?
        if !self.d.contains_key(&key) || self.d[&key].0 > val.0 {
            self.d.insert(key, val);
        }
    }

    fn advance_all(self, time: u64) -> Self {
        let mut new = Self{d: HashMap::with_capacity(self.d.len())};
        for (state, val) in self.d.into_iter() {
            new.update_best(state.advance(time), val);
        }
        new
    }
}



fn realmain() -> io::Result<()> {
    let input = getinput()?;
    let mut lines = input.lines();

    let caps = parse_ints(lines.next().unwrap());
    let horses = lines.map(parse_ints2).collect::<Vec<_>>();
    let numhorse = horses.len();

    let mut bests = StateBests::new();
    bests.d.insert(State(vec![]), (0, None));
    let mut time = 0;

    let mut horses = horses.into_iter().enumerate().collect::<Vec<_>>();
    horses.sort_by_key(|&(_, p)| p);

    for (i, (arr, dur)) in horses.into_iter() {
        let i = i as u64;
        if arr > time {
            bests = bests.advance_all(arr - time);
            time = arr;
        }
        assert!(time == arr);

        let mut oldbests = StateBests::new();
        std::mem::swap(&mut bests, &mut oldbests);

        for (state, (score, solution)) in oldbests.d.into_iter() {
            let mut curcaps = caps.clone();
            for (_, lvl) in state.0.iter().cloned() {
                curcaps[lvl as usize] -= 1;
            }

            for (lvl, cap) in curcaps.into_iter().enumerate() {
                if cap == 0 {continue;}
                let lvl = lvl as u64;

                let state2 = state.insert(dur, lvl);
                let score2 = score + dur * lvl;
                let solution2 = Some(leak(ListNode((i, lvl), solution)));
                bests.update_best(state2, (score2, solution2));
            }
        }
    }

    bests = bests.advance_all(u64::max_value());
    assert!(bests.d.len() == 1);

    let (_, mut solution) = bests.d[&State(vec![])];
    let mut pairs = Vec::with_capacity(numhorse);
    while let Some(p) = solution {
        let ListNode((i, lvl), sol2) = *p;
        solution = sol2;
        pairs.push((i+1, lvl+1));
    }

    pairs.sort();
    for (i, lvl) in pairs.into_iter() {
        println!("{} {}", i, lvl);
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
