#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Equals,
    Semicolon,
    EOF,
    LeftParen,
    RightParen,
    LeftChevron,
    RightChevron,
    Ampersand,
    Pipe,
    Boolean(bool),
    ExclamationMark,
    DoubleQuote,
    Function,
    LeftCurlyBrace,
    RightCurlyBrace,
    Return,
    String(String),
    Unknown(String),
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    let mut is_reading_string: bool = false;
    input.chars().for_each(|character| {
        if is_reading_string {
            match character {
                '"' => {
                    tokens.push(Token::String(current_string.clone()));
                    tokens.push(Token::DoubleQuote);
                    current_string.clear();
                    is_reading_string = false;
                }
                _ => current_string.push(character),
            }
        } else {
            match character {
                ' ' | '\n' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    current_string.clear();
                }
                '=' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Equals);
                }
                '+' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Plus);
                }
                '-' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Minus);
                }
                '*' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Star);
                }
                '/' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Slash);
                }
                ';' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Semicolon);
                }
                '(' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::LeftParen);
                }
                ')' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::RightParen);
                }
                '<' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::LeftChevron);
                }
                '>' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::RightChevron);
                }
                '{' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::LeftCurlyBrace);
                }
                '}' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::RightCurlyBrace);
                }
                '&' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Ampersand);
                }
                '|' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::Pipe);
                }
                '!' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::ExclamationMark);
                }
                '"' => {
                    evaluate_current_string(&mut tokens, &mut current_string);
                    tokens.push(Token::DoubleQuote);
                    is_reading_string = true;
                }
                _ => {
                    current_string.push(character);
                }
            }
        }
    });
    if string_has_non_whitespace(&current_string) {
        evaluate_current_string(&mut tokens, &mut current_string);
    }
    tokens.push(Token::EOF);
    tokens
}

fn evaluate_current_string(tokens: &mut Vec<Token>, current_string: &mut String) {
    if string_has_non_whitespace(current_string) {
        if *current_string == "let" {
            tokens.push(Token::Let);
        } else if current_string.trim() == "function" {
            tokens.push(Token::Function);
        } else if current_string.trim() == "return" {
            tokens.push(Token::Return);
        } else if current_string.trim() == "true" || current_string.trim() == "false" {
            let bool_value = current_string.trim() == "true";
            tokens.push(Token::Boolean(bool_value));
        } else if is_string_a_number(current_string) {
            tokens.push(Token::Number(convert_string_to_f64(current_string)));
        } else {
            tokens.push(Token::Ident(current_string.clone()));
        }
    }
    current_string.clear();
}

fn is_string_a_number(current_string: &String) -> bool {
    let result = current_string.trim().parse::<f64>();
    result.is_ok()
}

fn convert_string_to_f64(current_string: &String) -> f64 {
    current_string.trim().parse::<f64>().unwrap()
}

fn string_has_non_whitespace(current_string: &String) -> bool {
    current_string.trim().len() > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    static BASIC_TEST_STRING: &str = "let x = 3 + 4;";

    #[test]
    fn it_parses_let() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[0], Token::Let);
    }

    #[test]
    fn it_parses_ident() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[1], Token::Ident("x".to_string()));
    }

    #[test]
    fn it_parses_assign() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[2], Token::Equals);
    }

    #[test]
    fn it_parses_a_number() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[3], Token::Number(3.0));
    }

    #[test]
    fn it_parses_plus() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[4], Token::Plus);
    }

    #[test]
    fn it_parses_a_second_number() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[5], Token::Number(4.0));
    }

    #[test]
    fn it_parses_semicolon() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[6], Token::Semicolon);
    }

    #[test]
    fn it_ends_with_eof() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[7], Token::EOF);
    }

    static TEST_STRING_WITH_REASSIGNMENT: &str = "
    let x = 3 + 4;
    x = 9;
    ";

    #[test]
    fn it_finds_previously_used_ident() {
        let result = tokenize(TEST_STRING_WITH_REASSIGNMENT);
        assert_eq!(result[7], Token::Ident("x".to_string()));
    }

    #[test]
    fn it_parses_minus() {
        let result = tokenize("-");
        assert_eq!(result[0], Token::Minus);
    }

    #[test]
    fn it_parses_star() {
        let result = tokenize("*");
        assert_eq!(result[0], Token::Star);
    }

    #[test]
    fn it_parses_slash() {
        let result = tokenize("/");
        assert_eq!(result[0], Token::Slash);
    }

    #[test]
    fn it_parses_without_semicolon() {
        let result = tokenize("1 + 2");
        let expected = [
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_parentheses() {
        let result = tokenize("(1 + 2)");
        let expected = [
            Token::LeftParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RightParen,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_without_spaces() {
        let result = tokenize("1+2");
        let expected = [
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_ending_in_return() {
        let result = tokenize("1+2\n");
        let expected = [
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_chevrons() {
        let result = tokenize("1 <> 2");
        let expected = [
            Token::Number(1.0),
            Token::LeftChevron,
            Token::RightChevron,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_true() {
        let result = tokenize("true");
        let expected = [Token::Boolean(true), Token::EOF];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_false() {
        let result = tokenize("false");
        let expected = [Token::Boolean(false), Token::EOF];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_ampersand() {
        let result = tokenize("1 && 2");
        let expected = [
            Token::Number(1.0),
            Token::Ampersand,
            Token::Ampersand,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_pipe() {
        let result = tokenize("1 || 2");
        let expected = [
            Token::Number(1.0),
            Token::Pipe,
            Token::Pipe,
            Token::Number(2.0),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_exclamation_mark() {
        let result = tokenize("!(1 > 2)");
        let expected = [
            Token::ExclamationMark,
            Token::LeftParen,
            Token::Number(1.0),
            Token::RightChevron,
            Token::Number(2.0),
            Token::RightParen,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_true_without_semicolon() {
        let result: Vec<Token> = tokenize("true");
        let expected = [Token::Boolean(true), Token::EOF];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_true_without_semicolon_assignment() {
        let result: Vec<Token> = tokenize("let x = true");
        let expected = [
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equals,
            Token::Boolean(true),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_true_without_semicolon_assignment_with_newline() {
        let result: Vec<Token> = tokenize("let x = true\n");
        let expected = [
            Token::Let,
            Token::Ident("x".to_string()),
            Token::Equals,
            Token::Boolean(true),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_string() {
        let result: Vec<Token> = tokenize("\"This is a String\"");
        let expected = [
            Token::DoubleQuote,
            Token::String("This is a String".to_string()),
            Token::DoubleQuote,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn it_parses_a_function_declaration() {
        let result: Vec<Token> = tokenize("function returnPi() { return 3.1415 }");
        let expected = [
            Token::Function,
            Token::Ident("returnPi".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::LeftCurlyBrace,
            Token::Return,
            Token::Number(3.1415),
            Token::RightCurlyBrace,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }
}
