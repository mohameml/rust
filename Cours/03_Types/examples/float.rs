
// Fill the blank to make it work
// fn main() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64

//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }


fn main() {
    let x1 = 0.1 ;
    let x2 = 0.2 ;
    let x3 = 0.3 ;
    let x4 = x1 + x2 ;
    let dif : f64 = x4 - x3 ;

    assert!(dif.abs() < 0.0001);

    println!("Success! {}" , dif.abs());
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}