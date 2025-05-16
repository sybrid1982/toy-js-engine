#[derive(Debug, PartialEq)]
enum Token {
    Let,
    Ident(String),
    Number(f64),
    Plus,
    Assign,
    Semicolon,
    EOF
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_string: String = String::new();
    input.chars().for_each(|character| {
        match character {
            ' ' => {
                if current_string.len() > 0 {
                    evaluate_current_string(&mut tokens, &current_string);
                    current_string.clear();
                }
                current_string.clear();
            },
            '=' => {
                if current_string.len() > 0 {
                    evaluate_current_string(&mut tokens, &current_string);
                    current_string.clear();
                }
                tokens.push(Token::Assign);
            },
            '+' => {
                if current_string.len() > 0 {
                    evaluate_current_string(&mut tokens, &current_string);
                    current_string.clear();
                }
                tokens.push(Token::Plus);
            },
            ';' => {
                if current_string.len() > 0 {
                    evaluate_current_string(&mut tokens, &current_string);
                    current_string.clear();
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

fn evaluate_current_string(tokens: &mut Vec<Token>, current_string: &String) {
    if *current_string == "let" {
        tokens.push(Token::Let)
    } else if is_string_a_number(current_string) {
        tokens.push(Token::Number(convert_string_to_f64(current_string)));
    } else if tokens.len() > 0 && *tokens.last().unwrap() == Token::Let {
        tokens.push(Token::Ident(current_string.clone()));
    }
}

fn is_string_a_number(current_string: &String) -> bool {
    let result = current_string.parse::<f64>();
    result.is_ok()
}

fn convert_string_to_f64(current_string: &String) -> f64 {
    current_string.parse::<f64>().unwrap()
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
}