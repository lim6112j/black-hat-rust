use std::collections::HashMap;

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
fn main() {
    vector();
    array();
    for_each();
    fold2();
}
