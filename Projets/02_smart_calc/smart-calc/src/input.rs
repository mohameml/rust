use std::io::{self, Write};

pub fn wolckom_msg() {
    println!("======== Wolckom to smart-calc ⭐⭐⭐: ===========");
    println!("just write your expression and see the result🪄");
    print!("function supported : ");
    println!(
        "
\t- + , - , * , / , ( , )
\t- cos , sin , tan , 
\t- ln , exp , sqrt 
"
    );
}

pub fn read_input() -> String {
    // wolckom_msg();

    let mut input = String::new();

    print!("expr :");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("faild to read the input");

    input = input.replace("\n", "").replace(" ", "");

    input = input.trim().to_string();

    input
}
