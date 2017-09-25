use std::char;
use std::str;

fn find_quote(s: &[u8]) -> Option<usize> {
    s.iter().cloned().enumerate().find(|&(_, b)| b == b'"').map(|(i, _)| i)
}

fn find_quote_or_slash(s: &[u8]) -> (usize, u8) {
    s.iter().cloned().enumerate().find(|&(_, b)| b == b'"' || b == b'\\').unwrap()
}

fn get_str(mut s: &[u8]) -> (&[u8], String) {
    let mut result = String::new();

    // let pos2 = s.find(|c| c == '"' || c == '\\').unwrap();
    let (pos2, mut foundb) = find_quote_or_slash(s);
    let temp = s.split_at(pos2);
    result.push_str(str::from_utf8(temp.0).unwrap());
    s = temp.1;

    while foundb == b'\\' {
        // println!("esc[{}] s='{}'", s.len(), str::from_utf8(s).unwrap());

        match s[1] {
            b'u' => {
                let code = u32::from_str_radix(
                    str::from_utf8(&s[2..6]).unwrap(), 16).unwrap();

                result.push(char::from_u32(code).unwrap());
                s = s.split_at(6).1;
            }
            b'b' => {result.push('\x08'); s = s.split_at(2).1;}
            b'f' => {result.push('\x0C'); s = s.split_at(2).1;}
            b'n' => {result.push('\n'); s = s.split_at(2).1;}
            b'r' => {result.push('\r'); s = s.split_at(2).1;}
            b't' => {result.push('\t'); s = s.split_at(2).1;}
            x => {result.push(x as char); s = s.split_at(2).1;}
        };

        let (pos2, fb) = find_quote_or_slash(s);
        foundb = fb;

        let temp = s.split_at(pos2);
        result.push_str(str::from_utf8(temp.0).unwrap());
        s = temp.1;
    }

    s = s.split_at(1).1;
    (s, result)
}

pub fn stupid_json(s: &str) -> Vec<(Vec<u8>, String)> {
    let mut results = Vec::new();

    let mut s = s.as_bytes();
    while let Some(ind) = find_quote(s) {
        // println!("id[{}] s='{}'", s.len(), str::from_utf8(&s[ind..]).unwrap());

        let id = s[ind+1..ind+65].to_vec();
        s = s.split_at(ind+66).1;

        let qpos = find_quote(s).unwrap();
        s = s.split_at(qpos+1).1;

        let temp = get_str(s);
        s = temp.0;
        let message = temp.1;

        results.push((id, message));
    }
    results
}