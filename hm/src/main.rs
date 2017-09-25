use std::fmt;
use std::io::{self, Read};
use std::rc::Rc;
use std::collections::HashMap;
use std::{thread, time};

#[allow(dead_code)]
fn sleep() {
    thread::sleep(time::Duration::from_secs(10));
}

#[allow(dead_code)]
fn forget<T>(val: T) -> &'static T {
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

fn index(buf: &str, start: usize, end: usize) -> &str {
    // &buf[start..end]
    unsafe{buf.get_unchecked(start..end)}
}

fn tokenize(buf: &str) -> Vec<&str> {
    let mut results = Vec::new();
    let mut start = 0;
    let mut pos = 0;

    while pos < buf.len() {
        match buf.as_bytes()[pos] {
            b'a'...b'z' | b'A'...b'Z' | b'0'...b'9' => {
                pos += 1;
            }
            b'(' | b')' => {
                if start < pos {
                    results.push(index(buf, start, pos));
                }
                results.push(index(buf, pos, pos+1));

                pos += 1;
                start = pos;
            }
            _ => {
                if start < pos {
                    results.push(index(buf, start, pos));
                }
                pos += 1;
                start = pos;
            }
        }
    }
    if start < pos {
        results.push(index(buf, start, pos));
    }
    results
}

// abstract syntax tree
type RcType = &'static Type;

#[derive(Clone, PartialEq, Eq)]
enum Type {
    Bool,
    Int,
    Var{id: u32},
    Func{arg: RcType, ret: RcType},
}
fn new_func<'a>(arg: Type, ret: Type) -> Type {
    Type::Func{arg: forget(arg), ret: forget(ret)}
}

type Substitution<'a> = (u32, Type);
impl<'a> Type {
    fn sub_ref(r: &mut RcType, substitution: &Substitution<'a>) -> bool {
        let mut temp = r.clone();
        let changed = temp.sub(substitution);
        if changed {
            *r = forget(temp);
        }
        changed
    }

    fn sub(&mut self, substitution: &Substitution<'a>) -> bool {
        match *self {
            Type::Func{ref mut arg, ref mut ret} => {
                let changed1 = Self::sub_ref(arg, substitution);
                let changed2 = Self::sub_ref(ret, substitution);
                return changed1 || changed2;
            }
            Type::Var{id, ..} => {
                if id != substitution.0 {
                    return false;
                }
                // fall outside match where repl will be done
            }
            Type::Bool | Type::Int => {return false;}
        }
        *self = substitution.1.clone();
        true
    }
}
impl<'a> fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Bool => write!(f, "bool"),
            Type::Int => write!(f, "int"),
            Type::Var{id, ..} => write!(f, "v{}", id),
            Type::Func{ref arg, ref ret} => {
                write!(f, "(lam {} {})", arg, ret)
            }
        }
    }
}






type Env<'a> = HashMap<&'a str, Type>;

struct State {
    var_counter: u32,
}
impl State {
    fn new() -> Self {State{var_counter: 1}}

    fn new_var(&mut self) -> Type {
        let id = self.var_counter;
        self.var_counter += 1;
        Type::Var{id}
    }

    fn parse_and_get_type<'a>(&mut self, it: &mut std::vec::IntoIter<&'a str>, env: Env<'a>, outcons: &mut Vec<(Type, Type)>) -> Type {
        let tok = it.next().expect("expected token");
        match tok {
            "(" => {
                let t = self.parse_and_get_type(it, env, outcons);
                it.next().expect("expected closing paren");
                t
            }
            "lam" => {
                let name = it.next().expect("expected lambda var name");
                let var = self.new_var();
                let mut env = env;
                env.insert(name, var.clone());
                // Rc::make_mut(&mut env).insert(name, var.clone());

                let t = self.parse_and_get_type(it, env, outcons);
                new_func(var, t)
            }
            "app" => {
                let t1 = self.parse_and_get_type(it, env.clone(), outcons);
                let t2 = self.parse_and_get_type(it, env, outcons);
                let var = self.new_var();

                outcons.push((t1, new_func(t2, var.clone())));
                var
            }
            _ => {
                env.get(tok).cloned().unwrap_or(Type::Int)
            }
        }
    }

    fn globals(&mut self) -> Env<'static> {
        let ifv = self.new_var();
        let fixv = self.new_var();

        let mut env = HashMap::with_capacity(5);
        env.insert("add", new_func(Type::Int, new_func(Type::Int, Type::Int)));
        env.insert("gt", new_func(Type::Int, new_func(Type::Int, Type::Bool)));
        env.insert("if", new_func(Type::Bool, new_func(ifv.clone(), new_func(ifv.clone(), ifv))));
        env.insert("fix", new_func(new_func(fixv.clone(), fixv.clone()), fixv));
        env.insert("true", Type::Bool);
        env.insert("false", Type::Bool);
        env
    }
}




fn unify<'a>(t1: Type, t2: Type, outsubs: &mut Vec<Substitution<'a>>) {
    if t1 == t2 {return;}

    if let Type::Var{id} = t1 {
        outsubs.push((id, t2));
    } else if let Type::Var{id} = t2 {
        outsubs.push((id, t1));
    } else if let Type::Func{arg, ret} = t1 {
        let (arg1, ret1) = (arg, ret);
        if let Type::Func{arg, ret} = t2 {
            let (arg2, ret2) = (arg, ret);

            let oldlen = outsubs.len();
            unify((*arg1).clone(), (*arg2).clone(), outsubs);


            let (mut r1, mut r2) = ((*ret1).clone(), (*ret2).clone());
            for substitution in &outsubs[oldlen..] {
                r1.sub(substitution);
                r2.sub(substitution);
            }

            unify(r1, r2, outsubs);
        }
    }
}


fn realmain() -> io::Result<()> {
    let input = getinput()?;

    let mut alloc = State::new();
    let env = alloc.globals();

    for line in input.lines() {
        let line = line.trim();
        let tokens = tokenize(line);
        let mut cons = Vec::new();
        let mut t = alloc.parse_and_get_type(&mut tokens.into_iter(), env.clone(), &mut cons);

        while let Some((lhs, rhs)) = cons.pop() {
            let mut subs = Vec::new();
            unify(lhs, rhs, &mut subs);

            for substitution in subs.into_iter() {
                for con in cons.iter_mut() {
                    con.0.sub(&substitution);
                    con.1.sub(&substitution);
                }
                t.sub(&substitution);
            }
        }

        println!("{}", t);
    }
    Ok(())
}

fn main() {realmain().unwrap()}

// use std::panic;
// fn main() {
//     panic::catch_unwind(|| {
//         realmain().unwrap()
//     }).map(|_| {
//         // sleep();
//         1
//     }).unwrap_or_else(|_| {
//         sleep();
//         2
//     });
// }
