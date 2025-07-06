use std::process::exit;

use smart_calc::input::{read_input, wolckom_msg};
use smart_calc::lexer::{parser, tokenizer};
use smart_calc::parser::Parser;

fn smart_calc(with_loop: bool, print_input: bool, print_tokens: bool, print_arbre: bool) {
    // =========== 1. READ the input of user : ==========

    wolckom_msg();

    loop {
        let input = read_input();

        if input == "q" || input == "Q" || input == "Quit" || input == "quit" {
            exit(0);
        }
        // println!("input : {input:?}");

        // 2. =============== parse the input to vec of String ================
        let tokens = parser(&input);

        if print_input {
            println!("input_parser = {:?}", tokens);
        }

        // 3. ================ Tokenize the vec of String to Vec of Token ==============

        let tokens = tokenizer(tokens);
        if print_tokens {
            println!("Tokens = {:#?}", tokens);
        }

        // 4.  ========== Prasing : parse vec of token to AST ===============

        let mut parser_ = Parser {
            tokens: tokens,
            pos: 0,
        };

        let expr = parser_.parser_expr().expect("Faild to parse");

        if print_arbre {
            expr.pretty_print();
        }

        // 5.  ====================== Eval : the final value :====================

        let value = expr.eval();

        println!("The value of input is : {}", value);

        if !with_loop {
            break;
        }
    }
}

fn main() {
    smart_calc(true, true, true, true);
}
