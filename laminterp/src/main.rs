mod bigint;

use std::io::{self, Read, Write};
use std::rc::Rc;
use std::collections::HashMap;
use std::{thread, time};
use std::marker::PhantomData;

use bigint::BigInt;
use bigint::forget;


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

fn is_digits(s: &str) -> bool {
    for c in s.bytes() {
        if c < b'0' || c > b'9' {return false;}
    }
    return true;
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
                    results.push(&buf[start..pos]);
                }
                results.push(&buf[pos..pos+1]);

                pos += 1;
                start = pos;
            }
            _ => {
                if start < pos {
                    results.push(&buf[start..pos]);
                }
                pos += 1;
                start = pos;
            }
        }
    }
    if start < pos {
        results.push(&buf[start..pos]);
    }
    results
}


// abstract syntax tree
type RcAst = &'static Ast;
#[derive(Clone)]
enum Ast {
    Abs{var: usize, body: RcAst},
    App{lhs: RcAst, rhs: RcAst},
    Var{id: usize},
    Const{val: Value<'static>},
}

type VarMap<'a> = Rc<HashMap<&'a str, usize>>;
struct AstManager {
    count: usize,
}
impl<'a> AstManager {
    fn new() -> Self {Self{
        count: 0,
    }}

    fn alloc(&self, ast: Ast) -> RcAst {forget(ast)}

    fn new_var(&mut self, name: &'a str, varmap: &mut VarMap<'a>) -> usize {
        let id = self.count;
        self.count += 1;
        Rc::make_mut(varmap).insert(name, id);
        id
    }

    fn parse(&mut self, it: &mut std::vec::IntoIter<&'a str>, mut varmap: VarMap<'a>) -> RcAst {
        let tok = it.next().expect("expected token");

        let ast = match tok {
            "(" => {
                let e = self.parse(it, varmap);
                let close = it.next().expect("expected closing paren");
                assert!(close == ")");
                return e;
            }
            "lam" => {
                let name = it.next().expect("expected lambda var name");
                (Ast::Abs{
                    var: self.new_var(name, &mut varmap),
                    body: self.parse(it, varmap),
                })
            }
            "app" => {
                (Ast::App{
                    lhs: self.parse(it, varmap.clone()),
                    rhs: self.parse(it, varmap),
                })
            }
            _ => {
                if is_digits(tok) {
                    (Ast::Const{val: Value::Int(Int::parse(tok).unwrap())})
                } else {
                    let id = *varmap.get(tok).unwrap();
                    match id {
                        0 => Ast::Const{val: Value::Add},
                        1 => Ast::Const{val: Value::Gt},
                        2 => Ast::Const{val: Value::If},
                        3 => Ast::Const{val: Value::Bool(true)},
                        4 => Ast::Const{val: Value::Bool(false)},
                        _ => Ast::Var{id},
                    }
                }
            }
        };
        self.alloc(ast)
    }
}



// evaluation
type Int = BigInt;
type Env<'a> = Rc<HashMap<usize, Value<'a>>>;

#[derive(Clone)]
enum Value<'a> {
    Bool(bool),
    Int(Int),

    Add,
    Add2(Int),
    Gt,
    Gt2(Int),
    If,
    If2(bool),
    If3(bool, Box<Value<'a>>),

    Lambda{var: usize, body: RcAst, env: Env<'a>, p: PhantomData<&'a ()>},
}
impl<'a> Value<'a> {
    fn call(self, arg: Value<'a>) -> Value<'a> {
        use Value::*;

        match self {
            Add => if let Int(x) = arg {Add2(x)} else {unreachable!()},
            Add2(mut x) => if let Int(y) = arg {
                x.add(y);
                Int(x)
            } else {unreachable!()},

            Gt => if let Int(x) = arg {Gt2(x)} else {unreachable!()},
            Gt2(x) => if let Int(y) = arg {
                Bool(x.cmp(&y) > 0)
            } else {unreachable!()},

            If => if let Bool(x) = arg {If2(x)} else {unreachable!()},
            If2(x) => {
                assert!(x);
                If3(x, Box::new(arg))
            },
            If3(x, a1) => if x {*a1} else {arg},

            // lambda or primitive
            _ => unreachable!()
        }
    }
}

enum EvalState<'a> {
    Eval{ast: RcAst, env: Env<'a>},
    Eval2{rhs: RcAst, env: Env<'a>},
    Eval3(Value<'a>),
    Return(Value<'a>),
}
use EvalState::*;


fn eval<'a>(ast: RcAst, env: Env<'a>) -> Value<'a> {
    let mut stack = vec![Eval{ast, env}];

    while let Some(command) = stack.pop() {
        match command {
            Eval{ast, env} => {
                match *ast {
                    Ast::Abs{var, ref body} => {
                        stack.push(Return(Value::Lambda{
                            var, body, env, p: PhantomData,
                        }));
                    }
                    Ast::App{ref lhs, ref rhs} => {
                        stack.push(Eval2{rhs: rhs.clone(), env: env.clone()});
                        stack.push(Eval{ast: lhs.clone(), env});
                    }
                    Ast::Var{id, ..} => {
                        stack.push(Return(env.get(&id).unwrap().clone()));
                    }
                    Ast::Const{ref val} => {
                        stack.push(Return(val.clone()));
                    }
                }
            }
            Return(returned_val) => {
                if let Some(command) = stack.pop() {
                    match command {
                        Eval2{rhs, env} => {
                            if let Value::If2(false) = returned_val {
                                // skip evaluation of RHS - replace it with a dummy val - Add
                                stack.push(Return(Value::If3(false, Box::new(Value::Add))));
                            }
                            else if let Value::If3(true, val) = returned_val {
                                // skip evaluation of RHS and returned stored true value
                                stack.push(Return(*val));
                            }
                            else {
                                stack.push(Eval3(returned_val));
                                stack.push(Eval{ast: rhs, env});
                            }
                        }
                        Eval3(e1) => {
                            let e2 = returned_val;

                            if let Value::Lambda{var, body, mut env, ..} = e1 {
                                Rc::make_mut(&mut env).insert(var, e2);
                                stack.push(Eval{ast: body, env});
                            } else {
                                let result = e1.call(e2);
                                stack.push(Return(result));
                            }
                        }
                        _ => unreachable!()
                    }
                } else {
                    return returned_val;
                }
            }
            _ => unreachable!()
        }
    }
    unreachable!()
}


fn realmain() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let input = getinput()?;

    for line in input.lines() {
        let line = line.trim();
        let tokens = tokenize(line);

        let mut astman = AstManager::new();
        let mut varmap = Rc::new(HashMap::new());
        let env = HashMap::new();
        astman.new_var("add", &mut varmap);
        astman.new_var("gt", &mut varmap);
        astman.new_var("if", &mut varmap);
        astman.new_var("true", &mut varmap);
        astman.new_var("false", &mut varmap);
        let env = Rc::new(env);


        let ast = astman.parse(&mut tokens.into_iter(), varmap);
        let val = eval(ast, env);

        if let Value::Int(x) = val {
            writeln!(&mut handle, "{}", x)?;
        }
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
