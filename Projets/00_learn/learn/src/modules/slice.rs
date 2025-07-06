use std::array::from_fn;

pub fn slice() {
    // slice in string :
    let s = String::from("hello world");

    let s1 = &s[..=2];

    println!("s = {s}");

    println!("s1 = {s1}");

    // slice array :
    let arr: [i32; 10] = from_fn(|i| (i + 1) as i32);

    println!("arr = {arr:#?}");

    let arr_slice = &arr[..5];

    println!("arr_slice = {arr_slice:#?}");
}
