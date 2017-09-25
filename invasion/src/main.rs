use std::io::{self, Read};
use std::{thread, time};
use std::collections::HashSet;

extern crate serde_json;
extern crate crypto;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha2::Sha256;

// extern crate rustc_serialize;
// use rustc_serialize::hex::FromHex;
// use rustc_serialize::hex::ToHex;

mod hex;
use hex::ToHex;

// mod json;


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

type Pair<'a> = &'a (Vec<u8>, String);
fn is_suc(pair: Pair, pair2: Pair) -> bool {
    let mut h = Hmac::new(Sha256::new(), &pair.0);
    h.input(pair2.1.as_ref());

    let mut buf = [0u8; 32];
    h.raw_result(&mut buf);

    pair2.0 == buf.to_hex().into_bytes()
}

fn get_prev<'a>(pairs: &mut HashSet<Pair<'a>>, pair2: Pair<'a>) -> Option<Pair<'a>> {
    let mut result = None;
    for pair in pairs.iter().cloned() {
        if is_suc(pair, pair2) {
            result = Some(pair);
            break;
        }
    }

    if let Some(pair) = result {
        pairs.remove(pair);
    }
    result
}

fn get_next<'a>(pairs: &mut HashSet<Pair<'a>>, pair: Pair<'a>) -> Option<Pair<'a>> {
    let mut result = None;
    for pair2 in pairs.iter().cloned() {
        if is_suc(pair, pair2) {
            result = Some(pair2);
            break;
        }
    }

    if let Some(pair2) = result {
        pairs.remove(pair2);
    }
    result
}

fn realmain() -> io::Result<()> {
    let input = getinput()?;

    let start = std::env::args().nth(1).unwrap().into_bytes();

    // let messages =
    //     if input.as_bytes().contains(&b'\\') {
    //         let messages: Vec<(String, String)> = serde_json::from_str(&input).unwrap();
    //         messages.into_iter().map(|(id, msg)| (id.into_bytes(), msg)).collect::<Vec<_>>()
    //     } else {
    //         json::stupid_json(&input)
    //     };
    // let messages = json::stupid_json(&input);
    let messages: Vec<(String, String)> = serde_json::from_str(&input).unwrap();
    let messages = messages.into_iter().map(|(id, msg)| (id.into_bytes(), msg)).collect::<Vec<_>>();


    let startp = messages.iter().find(|&&(ref id, _)| id == &start).unwrap();
    let mut pairs = messages.iter().collect::<HashSet<_>>();


    let mut good_message = vec![&startp.1];

    let mut pair = get_prev(&mut pairs, startp);
    while let Some(p) = pair {
        good_message.push(&p.1);
        pair = get_prev(&mut pairs, p);
    }

    good_message.reverse();
    let mut pair = get_next(&mut pairs, startp);
    while let Some(p) = pair {
        good_message.push(&p.1);
        pair = get_next(&mut pairs, p);
    }


    // println!("startp {:?}", startp);
    println!("{}", good_message.into_iter().map(|c| c.as_str()).collect::<Vec<_>>().join(" "));
    // println!("{}", good_message.join(" "));
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
