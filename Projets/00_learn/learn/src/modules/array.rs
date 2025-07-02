use std::array::from_fn;

pub fn array() {
    // let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // let arr: [i32; 5] = [1; 5];
    let arr: [i32; 5] = from_fn(|i| (i + 1) as i32);

    println!("Arr = {:#?}", arr);
}
