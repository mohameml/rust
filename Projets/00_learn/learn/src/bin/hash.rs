use std::collections::HashMap;

fn main() {
    let mut hash = HashMap::new();

    hash.insert(String::from("blue"), 10);
    hash.insert(String::from("red"), 20);

    println!("hash = {:#?}", hash);

    for (key, v) in &hash {
        println!("the value of key {key} is {v}");
    }

    let mut new_hash = HashMap::new();

    // new_hash.insert(String::from("blue"), 10);

    println!("newHash = {:#?}", new_hash);
    let c1 = new_hash.entry(String::from("blue")).or_insert(20);
    // let c2 = new_hash.entry(String::from("blue")).or_insert(30);

    println!("c1 = {:#?}", c1);
    // println!("c2 = {:#?}", c2);
}
