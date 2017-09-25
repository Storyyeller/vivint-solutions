use std::io::{self, Read, Write};

fn realmain() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let mut floor = 0;
    for (i, c) in buffer.bytes().enumerate() {
        if c == b'+' {
            floor += 1;
        } else {
            if floor == 0 {
                writeln!(&mut handle, "{}", i+1)?;
            }
            floor -= 1;
        }
    }

    Ok(())
}

fn main() {realmain().unwrap()}
