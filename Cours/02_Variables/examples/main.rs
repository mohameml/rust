#[allow(unused_variables)]
fn main() {
    
    // ================== 1. Uninitialized variable ==================
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32 = 6 ; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");


    // ================== 2. Mutable variable ==================
    
    let mut z = 1;
    z+= 2; 
    assert_eq!(z, 3);
    println!("Success!");

    // ================== 3. Scope ==================

    // exemple 1 : 
    let x2: i32 = 10;
    let y2: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x2 , y2);
    }
    println!("The value of x is {} and value of y is {}", x2, y2); 

    // exemple 2 : with fn : define_x
    define_x();
    
    // ================= 4. Shadowing ==================
    // You can declare a new variable with the same name as a previous variable, 
    // here we can say the first one is shadowed by the second one

    // exemple 1 :
    let x4: i32 = 5;
    {
        let x4 = 12;
        assert_eq!(x4, 12);
    }

    assert_eq!(x4, 5);

    let x4 = 42;
    println!("{}", x4); // Prints "42".

    // exemple 2 : 
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    // x += 3;


    let y : i32 = 4;
    // Shadowing
    let y : &str = "I can also be bound to text!"; 

    println!("Success!");


    // ================= 5. Unused variables ==================
    let _x7 : i32 = 5;

    // =================== 6. Destructuring ==================
    let  (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");


    // ================= 7. Destructuring assignments ==================
    let (x, y); // let x ; let y ;
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");

}


// function : 
fn define_x() {
    let x3 : &str = "hello";
    println!("{}, world", x3); 
}
