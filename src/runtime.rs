use std::collections::HashMap;
use common_macros::hash_map;

use crate::parser::{AstNode, Value};

struct Context {
    variables: HashMap<String, Box<dyn Fn(Vec<Value>) -> Value>>
}

fn print(args: Vec<Value>) {
    println!("{:?}", args[0]);
}

pub fn execute_ast(ast: AstNode) {

    let context = Context {
        variables: hash_map!{
            "print".to_string() => Box::new(|args: Vec<Value>| {
                println!("{:?}", args[0]);
            })
            //"print".to_string() => &print
        }
    };

    println!("t");
}
