use smart_calc::lexer;

#[test]
fn test_parser_simple() {
    let expected = vec!["1", "+", "2"];

    let input_test = "1+2";

    let res = lexer::parser(input_test);

    assert_eq!(res, expected);
}

#[test]
fn test_parser_one_token() {
    let expected = vec!["cos"];

    let input_test = "cos";

    let res = lexer::parser(input_test);

    assert_eq!(res, expected);
}

#[test]
fn test_parser() {
    let expected = vec![
        "(", "1", "+", "2", ")", "*", "3", "+", "44", "*", "5.12", "*", "cos", "(", "12", ")",
    ];

    let input_test = "(1+2)*3+44*5.12*cos(12)";

    let res = lexer::parser(input_test);

    assert_eq!(res, expected);
}

#[test]
#[should_panic(expected = "current_token is 22 and char_input is c")]
fn test_parser_panic() {
    let input_test = "1+22cos";

    lexer::parser(input_test);
}

#[test]
#[should_panic(expected = "current_token is cos and char_input is 2")]
fn test_parser_panic_2() {
    let input_test = "cos22";

    lexer::parser(input_test);
}

#[test]
#[should_panic(expected = "current_token is 2. and char_input is .")]
fn test_parser_panic_3() {
    let input_test = "1+2..2";

    lexer::parser(input_test);
}
