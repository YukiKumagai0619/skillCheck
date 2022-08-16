extern crate regex;

use regex::Regex;
use std::env;
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]

enum ValueKind {
    Int(isize),
    Float(f64),
    Bool(bool),
    String(String),
}

struct KeyValue {
    key: String,
    value: ValueKind,
}

impl KeyValue {
    fn new(vec: Vec<&str>) -> Self {
        let k = vec[0].to_string();
        let v = vec[1];
        let int = Regex::new(r"^(-|)[0-9]*$").unwrap();
        let float = Regex::new(r"^(-|)[0-9]+\.[0-9]+$").unwrap();
        let boolean = Regex::new(r"(true|false)").unwrap();
        if int.is_match(v) {
            KeyValue {
                key: k,
                value: ValueKind::Int(v.parse().unwrap()),
            }
        } else if float.is_match(v) {
            KeyValue {
                key: k,
                value: ValueKind::Float(v.parse().unwrap()),
            }
        } else if boolean.is_match(v) {
            KeyValue {
                key: k,
                value: ValueKind::Bool(v.parse().unwrap()),
            }
        } else {
            KeyValue {
                key: k,
                value: ValueKind::String(v.parse().unwrap()),
            }
        }
    }

    fn print(self) {
        match self.value {
            ValueKind::Int(x) => {
                println!("key: {}, value: {:?}, type: {}", self.key, x, type_of(&x))
            }
            ValueKind::Float(x) => {
                println!("key: {}, value: {:?}, type: {}", self.key, x, type_of(&x))
            }
            ValueKind::Bool(x) => {
                println!("key: {}, value: {:?}, type: {}", self.key, x, type_of(&x))
            }
            ValueKind::String(x) => {
                println!("key: {}, value: {:?}, type: {}", self.key, x, type_of(&x))
            }
        }
    }
}

// 型取得
fn type_of<T>(_: &T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    // 引数にファイルパスを受け取る
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let f = File::open(path)?;
    let reader = BufReader::new(f);
    let mut kv_vec: Vec<KeyValue> = Vec::new();

    // 一行ずつ処理
    for line in reader.lines() {
        let l = line?;
        let vec: Vec<_> = l.split(" = ").collect();
        if vec.len() == 2 {
            kv_vec.push(KeyValue::new(vec));
        }
    }

    // 表示
    for v in kv_vec {
        v.print();
    }

    Ok(())
}
