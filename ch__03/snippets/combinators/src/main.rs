#![allow(dead_code)]
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
};

fn vector() {
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x);
    }
    // you can't longer use v
}
fn array() {
    let a = [1, 2, 3];
    for x in a.iter() {
        println!("{}", x)
    }
    println!("{}", a[1])
}
fn for_each() {
    let v = vec!["Hello", "world"].into_iter();
    v.for_each(|word| {
        println!("{}", word);
    });
}
fn collect() {
    let x = vec![1, 2, 3, 4, 5, 6, 7].into_iter();
    let _: Vec<u64> = x.collect();
}
fn from_iter() {
    let x = vec![(1, 2), (3, 4), (5, 6)].into_iter();
    let _: HashMap<u64, u64> = HashMap::from_iter(x);
}
fn fold() {
    let values = vec![1, 2, 3, 4, 5].into_iter();
    let _sum = values.reduce(|acc, x| acc + x);
}
fn fold2() {
    let values = vec!["Hello", "World", "!"].into_iter();
    let _sentence = values.fold(String::new(), |acc, x| acc + x);
    println!("{}", _sentence);
}

fn filter() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _filtered = v.filter(|x: &i32| x.is_positive()).collect::<Vec<i32>>();
    // println!("{}", filtered);
}
struct MyVec {
    myvec: Vec<i32>,
}
fn filter2() {
    let v = vec![-1, 2, -3, 4, 5];
    let my_vec = MyVec { myvec: v };
    println!("{}", my_vec);
}

impl fmt::Display for MyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let acc = self.myvec.iter().fold(String::new(), |acc, x| {
            let x_str = x.to_string();
            match acc.as_ref() {
                "" => acc + &x_str,
                _ => acc + ", " + &x_str,
            }
        });
        write!(f, "[{}]", acc)
    }
}
fn main() {
    vector();
    array();
    for_each();
    fold2();
    filter2();
    inspect();
    filter_map();
}

fn inspect() {
    let v = vec![-1, 2, -3, 4, 5].into_iter();
    let _positive_numbers: Vec<i32> = v
        .inspect(|x| println!("Before filter: {}", x))
        .filter(|x: &i32| x.is_positive())
        .inspect(|x| println!("After filter: {}", x))
        .collect();
    let my_vec = MyVec {
        myvec: _positive_numbers,
    };
    println!("filtered with positive {}", my_vec)
}
fn map() {
    let v = vec!["Hello", "World", "!"].into_iter();
    let _w: Vec<String> = v.map(String::from).collect();
}
fn filter_map() {
    let v = vec!["Hello", "World", "!"].into_iter();
    let _w: Vec<String> = v
        .filter_map(|x| {
            if x.len() > 2 {
                Some(String::from(x))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(_w, vec!["Hello".to_string(), "World".to_string()]);
}
fn chain() {
    let x = vec![1, 2, 3, 4, 5].into_iter();
    let y = vec![6, 7, 8, 9, 10].into_iter();
    let z: Vec<u64> = x.chain(y).collect();
    assert_eq!(z.len(), 10);
}
fn flatten() {
    let x = vec![vec![1, 2, 3, 4, 5], vec![6, 7, 8, 9, 10]].into_iter();
    let z: Vec<u64> = x.flatten().collect();
    assert_eq!(z.len(), 10);
}
// composing combinators
#[test]
fn combinators() {
    let a = vec![
        "1",
        "2",
        "-1",
        "4",
        "-4",
        "100",
        "invalid",
        "Not a number",
        "",
    ];
    let _only_positive_numbers: Vec<i64> = a
        .into_iter()
        .filter_map(|x| x.parse::<i64>().ok())
        .filter(|x| x > &0)
        .collect();
}
// Result to Option with ok()
fn result_ok() {
    let _port: Option<String> = std::env::var("PORT").ok();
}
fn result_or() {
    let _port: Result<String, std::env::VarError> =
        std::env::var("PORT").or(Ok(String::from("8080")));
}
