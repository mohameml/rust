#[allow(unused_variables)]
#[allow(unused_assignments)]
// #[allow(unused_mut)]
// #[allow(unused_imports)]
// #[allow(unused_macros)]
// #[allow(unused_attributes)]
// #[allow(unused_unsafe)]

fn main() {
    // Tips: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.
    let x: u32 = 5 ;
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; // Type of z ? i32

    println!("Success!");

    //
    let v: u16 = 38u8 as u16;

    println!("Success!");

    //
    let x :u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x)); 

    println!("Success!");

    //
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");

    // =============== exemple :
    let v1 : u16 = 251_u16 + 8;
    let v2 : i16 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);

    // =============== exemple :
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);



    println!("Success!");
    
    // =============== exemple : ================
    
    // for c in 'a'..='z' {
    //     println!("{:?}",c as u8);
    // }


}


// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}