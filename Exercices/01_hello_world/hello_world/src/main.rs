mod array;
mod some ; 

use crate::array::test_array;
use crate::some::div; 

fn main() {
    test_array();


    let res = div(4.0,2.0);
    println!("res = {:?}" , res);
}


