use core::panic;

const FUNCTIONS: [&str; 6] = ["cos", "sin", "tan", "ln", "sqrt", "exp"];
const CONSTS: [&str; 2] = ["pi", "e"];

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Sub,
    Mult,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Int(i32),
    Float(f64),
    Operator(Operator),
    LParen,
    RParen,
    Function(String),
    Constant(String),
}

fn str_to_token(ch: &str) -> Token {
    match ch {
        "+" => Token::Operator(Operator::Plus),
        "-" => Token::Operator(Operator::Sub),
        "*" => Token::Operator(Operator::Mult),
        "/" => Token::Operator(Operator::Div),
        "(" => Token::LParen,
        ")" => Token::RParen,

        _ if matches!(ch.parse::<i32>(), Ok(_)) => Token::Int(ch.parse::<i32>().unwrap()),
        _ if matches!(ch.parse::<f64>(), Ok(_)) => Token::Float(ch.parse::<f64>().unwrap()),

        _ if FUNCTIONS.contains(&ch) => Token::Function(ch.to_string()),
        _ if CONSTS.contains(&ch) => Token::Constant(ch.to_string()),

        _ => panic!("unkown char {ch}"),
    }
}

pub fn parser(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for c in input.chars() {
        match c {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }

                tokens.push(c.to_string());
            }

            _ if c.is_ascii_digit() => {
                let is_not_valid = !current_token.is_empty()
                    && !current_token
                        .chars()
                        .all(|x| x.is_ascii_digit() || x == '.');

                if is_not_valid {
                    panic!(
                        "[Error Tokenizer] : current_token is {} and char_input is {}",
                        current_token, c
                    );
                }
                current_token.push(c);
            }

            _ if c.is_ascii_alphabetic() => {
                let is_not_valide = !current_token.is_empty()
                    && !current_token.chars().all(|x| x.is_ascii_alphabetic());

                if is_not_valide {
                    panic!(
                        "[Error Tokenizer] : current_token is {} and char_input is {}",
                        current_token, c
                    );
                }

                current_token.push(c);
            }

            _ if c == '.' => {
                let is_valide =
                    !current_token.is_empty() && current_token.chars().all(|x| x.is_ascii_digit());

                if !is_valide {
                    panic!(
                        "[Error Tokenizer] : current_token is {} and char_input is {}",
                        current_token, c
                    );
                }

                current_token.push(c);
            }

            _ if c == ' ' => {}

            _ => {
                panic!("[Error Tokenizer] : invalid char_input {}", c);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

pub fn tokenizer(input: Vec<String>) -> Vec<Token> {
    let res: Vec<Token> = input.iter().map(|ch| str_to_token(&ch)).collect();

    return res;
}
