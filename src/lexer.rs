#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    Semicolon,
    EOF,
    Unknown(String)
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    input.chars().for_each(|character| {
        match character {
            ' ' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                current_string.clear();
            },
            '=' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Assign);
            },
            '+' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Plus);
            },
            '-' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Minus);
            },
            '*' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Star);
            },
            '/' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Slash);
            },
            ';' => {
                if string_has_non_whitespace(&current_string) {
                    evaluate_current_string(&mut tokens, &mut current_string);
                }
                tokens.push(Token::Semicolon);
            }
            _ => {
                current_string.push(character);
            }
        }
    });
    tokens.push(Token::EOF);
    tokens
}

fn evaluate_current_string(tokens: &mut Vec<Token>, current_string: &mut String) {
    if *current_string == "let" {
        tokens.push(Token::Let)
    } else if is_string_a_number(current_string) {
        tokens.push(Token::Number(convert_string_to_f64(current_string)));
    } else if last_token_was_let(tokens) || ident_token_exists(tokens, current_string){
        tokens.push(Token::Ident(current_string.clone()));
    } else {
        println!("current_string {} not evaluated to token!", current_string);
        tokens.push(Token::Unknown(current_string.clone()));
    }
    current_string.clear();
}

fn ident_token_exists(tokens: &mut Vec<Token>, current_string: &String) -> bool {
    tokens.contains(&Token::Ident(current_string.to_string()))
}

fn last_token_was_let(tokens: &mut Vec<Token>) -> bool {
    tokens.len() > 0 && *tokens.last().unwrap() == Token::Let
}

fn is_string_a_number(current_string: &String) -> bool {
    let result = current_string.parse::<f64>();
    result.is_ok()
}

fn convert_string_to_f64(current_string: &String) -> f64 {
    current_string.parse::<f64>().unwrap()
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
        assert_eq!(result[2], Token::Assign);
    }

    #[test]
    fn it_parses_a_number() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[3], Token::Number(3f64));
    }

    #[test]
    fn it_parses_plus() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[4], Token::Plus);
    }

    #[test]
    fn it_parses_a_second_number() {
        let result = tokenize(BASIC_TEST_STRING);
        assert_eq!(result[5], Token::Number(4f64));
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

    static TEST_STRING_WITH_UNKNOWN_IDENT: &str = "
    let x = 3 + 4;
    sum = 9;
    ";

    #[test]
    fn it_captures_unidentified_tokens() {
        let result = tokenize(TEST_STRING_WITH_UNKNOWN_IDENT);
        assert_eq!(result[7], Token::Unknown("sum".to_string()));
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
}