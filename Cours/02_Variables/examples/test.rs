fn main() {
    let mut x: i32 = 5;
    x = 10 ; // ERROR: cannot assign twice to immutable variable `x`
    println!("Success!");
}