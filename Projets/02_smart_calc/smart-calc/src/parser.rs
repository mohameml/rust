/*
- convert the Vec of Token to AST with the grammar :

    Expr        → Term (( "+" | "-" ) Term)*
    Term        → Factor (( "*" | "/" ) Factor)*
    Factor      → ( "+" | "-" )? Primary
    Primary     → number
                | const
                | Function "(" Expr ")"
                | "(" Expr ")"

    number      → [0-9]+(.[0-9]+)?
    const       → "pi" | "e"
    Function    → "cos" | "sin" | "ln" | "exp" | "sqrt"


*/

use crate::lexer::{Operator, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    Term(Box<Term>),
    Add(Box<Expr>, Box<Term>),
    Sub(Box<Expr>, Box<Term>),
}

#[derive(Debug, Clone)]
pub enum Term {
    Factor(Box<Factor>),
    Mul(Box<Term>, Box<Factor>),
    Div(Box<Term>, Box<Factor>),
}

#[derive(Debug, Clone)]
pub enum Factor {
    Plus(Box<Primary>),
    Minus(Box<Primary>),
    Plain(Box<Primary>),
}

#[derive(Debug, Clone)]
pub enum Primary {
    Number(f64),
    Const(Constant),
    FunctionCall(Function, Box<Expr>),
    GroupedExpr(Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Function {
    Cos,
    Sin,
    Tan,
    Ln,
    Exp,
    Sqrt,
}

#[derive(Debug, Clone)]
pub enum Constant {
    Pi,
    E,
}

fn next_indent(current_indent: &str, is_last: bool) -> String {
    let new_indent = if is_last {
        format!("{}  ", current_indent)
    } else {
        format!("{}|  ", current_indent)
    };

    new_indent
}

impl Expr {
    fn pretty_print_internal(&self, indent: &str, is_last: bool) {
        let branch = if is_last { "└─" } else { "├─" };

        println!("{}{}<expression>", indent, branch);

        let next_i: String = next_indent(indent, is_last);

        match self {
            Expr::Term(term) => {
                term.pretty_print_internal(&next_i, true);
            }

            Expr::Add(lhs, rhs) => {
                lhs.pretty_print_internal(&next_i, false);
                println!("{}├─ '+'", next_i);
                rhs.pretty_print_internal(&next_i, true);
            }

            Expr::Sub(lhs, rhs) => {
                lhs.pretty_print_internal(&next_i, false);
                println!("{}├─ '-'", next_i);
                rhs.pretty_print_internal(&next_i, true);
            }
        }
    }

    pub fn pretty_print(&self) {
        self.pretty_print_internal("", true);
    }

    pub fn eval(&self) -> f64 {
        match self {
            Expr::Term(term) => {
                let res = term.eval();
                res
            }

            Expr::Add(rhs, lhs) => {
                let left = rhs.eval();
                let right = lhs.eval();

                let res = left + right;
                res
            }

            Expr::Sub(rhs, lhs) => {
                let left = rhs.eval();
                let right = lhs.eval();

                let res = left - right;
                res
            }
        }
    }
}

impl Term {
    fn pretty_print_internal(&self, indent: &str, is_last: bool) {
        let branch = if is_last { "└─" } else { "├─" };

        println!("{}{}<term>", indent, branch);

        let next_i = next_indent(indent, is_last);

        match self {
            Term::Factor(factor) => {
                factor.pretty_print_internal(&next_i, true);
            }

            Term::Mul(lhs, rhs) => {
                lhs.pretty_print_internal(&next_i, false);
                println!("{}├─ '*'", next_i);
                rhs.pretty_print_internal(&next_i, true);
            }

            Term::Div(lhs, rhs) => {
                lhs.pretty_print_internal(&next_i, false);
                println!("{}├─ '/'", next_i);
                rhs.pretty_print_internal(&next_i, true);
            }
        }
    }

    fn eval(&self) -> f64 {
        match self {
            Term::Factor(factor) => {
                return factor.eval();
            }

            Term::Mul(rhs, lhs) => {
                return rhs.eval() * lhs.eval();
            }

            Term::Div(rhs, lhs) => {
                let left = rhs.eval();
                let right = lhs.eval();

                if right == 0. {
                    panic!("[Error Eval] : we can not devide by zero");
                }

                return left / right;
            }
        }
    }
}

impl Factor {
    fn eval(&self) -> f64 {
        match self {
            Factor::Plain(primary) => {
                return primary.eval();
            }

            Factor::Plus(primary) => {
                return primary.eval();
            }

            Factor::Minus(primary) => {
                return -primary.eval();
            }
        }
    }

    fn pretty_print_internal(&self, indent: &str, is_last: bool) {
        let branch = if is_last { "└─" } else { "├─" };

        println!("{}{}<factor>", indent, branch);

        let next_i = next_indent(indent, is_last);

        match self {
            Factor::Plain(primary) => {
                primary.pretty_print_internal(&next_i, true);
            }

            Factor::Plus(primary) => {
                println!("{}├─ '+'", next_i);
                primary.pretty_print_internal(&next_i, true);
            }

            Factor::Minus(primary) => {
                println!("{}├─ '-'", next_i);
                primary.pretty_print_internal(&next_i, true);
            }
        }
    }
}

impl Primary {
    fn eval(&self) -> f64 {
        match self {
            Primary::Number(number) => *number,
            Primary::Const(c) => c.eval(),

            Primary::FunctionCall(func, expr) => {
                let val_expr = expr.eval();
                match func {
                    Function::Cos => val_expr.cos(),
                    Function::Sin => val_expr.sin(),
                    Function::Tan => val_expr.tan(),
                    Function::Exp => val_expr.exp(),
                    Function::Ln => val_expr.ln(),
                    Function::Sqrt => val_expr.sqrt(),
                }
            }

            Primary::GroupedExpr(expr) => expr.eval(),
        }
    }

    fn pretty_print_internal(&self, indent: &str, is_last: bool) {
        let branch = if is_last { "└─" } else { "├─" };
        let next_i = next_indent(indent, is_last);

        match self {
            Primary::Number(number) => {
                println!("{}{}<number> \"{}\"", indent, branch, number);
            }

            Primary::Const(c) => {
                println!("{}{}<constant> \"{:?}\"", indent, branch, c);
            }

            Primary::FunctionCall(func, expr) => {
                println!("{}{}<function>", indent, branch);
                println!("{}├─ '{:?}'", next_i, func);
                println!("{}├─ '('", next_i);
                expr.pretty_print_internal(&next_i, false);
                println!("{}└─ ')'", next_i);
            }

            Primary::GroupedExpr(expr) => {
                println!("{}├─ '('", next_i);
                expr.pretty_print_internal(&next_i, false);
                println!("{}└─ ')'", next_i);
            }
        }
    }
}

impl Constant {
    fn eval(&self) -> f64 {
        match self {
            Constant::Pi => std::f64::consts::PI,
            Constant::E => std::f64::consts::E,
        }
    }
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub pos: usize,
}

impl Parser {
    fn current(&self) -> Option<Token> {
        self.tokens.get(self.pos).cloned()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, token: Token) -> Option<()> {
        if let Some(token_) = self.current() {
            if token_ == token {
                return Some(());
                // panic!("[Error Parsing] : Expected {:?} found {:?}", token, token_,);
            }
        }
        None
    }
}

impl Parser {
    pub fn parser_expr(&mut self) -> Option<Expr> {
        let mut node = Expr::Term(Box::new(self.parser_term()?));

        while let Some(Token::Operator(op)) = self.current() {
            match op {
                Operator::Plus | Operator::Sub => {
                    let op_toekn = op.clone();
                    self.advance();

                    let right = self.parser_term()?;

                    node = match op_toekn {
                        Operator::Plus => Expr::Add(Box::new(node), Box::new(right)),
                        Operator::Sub => Expr::Sub(Box::new(node), Box::new(right)),
                        _ => unreachable!(),
                    }
                }

                _ => break,
            }
        }

        Some(node)
    }

    fn parser_term(&mut self) -> Option<Term> {
        let mut node = Term::Factor(Box::new(self.parser_factor()?));

        while let Some(Token::Operator(op)) = self.current() {
            match op {
                Operator::Div | Operator::Mult => {
                    let op_token = op.clone();
                    self.advance();
                    let right = self.parser_factor()?;

                    node = match op_token {
                        Operator::Mult => Term::Mul(Box::new(node), Box::new(right)),
                        Operator::Div => Term::Div(Box::new(node), Box::new(right)),
                        _ => unreachable!(),
                    }
                }

                _ => break,
            }
        }

        Some(node)
    }

    fn parser_factor(&mut self) -> Option<Factor> {
        let token = self.current()?;

        match token {
            Token::Operator(op) => match op {
                Operator::Plus => {
                    self.advance();
                    let primary = self.parser_primary()?;
                    Some(Factor::Plus(Box::new(primary)))
                }

                Operator::Sub => {
                    self.advance();
                    let primary = self.parser_primary()?;
                    Some(Factor::Minus(Box::new(primary)))
                }

                _ => None,
            },

            _ => {
                let primary = self.parser_primary()?;

                Some(Factor::Plain(Box::new(primary)))
            }
        }
    }

    fn parser_primary(&mut self) -> Option<Primary> {
        let token = self.current()?;

        match token {
            Token::Float(f) => {
                self.advance();
                Some(Primary::Number(f))
            }

            Token::Int(i) => {
                self.advance();
                Some(Primary::Number(i as f64))
            }

            Token::Constant(c) => {
                self.advance();
                let constant = match c.as_str() {
                    "pi" => Constant::Pi,
                    "e" => Constant::E,
                    _ => return None,
                };

                Some(Primary::Const(constant))
            }

            Token::Function(name) => {
                let function = match name.as_str() {
                    "cos" => Function::Cos,
                    "sin" => Function::Sin,
                    "tab" => Function::Tan,
                    "ln" => Function::Ln,
                    "sqrt" => Function::Sqrt,
                    "Exp" => Function::Exp,
                    _ => return None,
                };

                self.advance();
                self.expect(Token::LParen);
                self.advance();
                let expr = self.parser_expr()?;
                self.expect(Token::RParen);
                self.advance();
                Some(Primary::FunctionCall(function, Box::new(expr)))
            }

            Token::LParen => {
                self.advance();
                let expr = self.parser_expr()?;
                self.expect(Token::RParen)?;
                self.advance();
                Some(Primary::GroupedExpr(Box::new(expr)))
            }

            _ => None,
        }
    }
}
