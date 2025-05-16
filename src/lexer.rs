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
                if current_string == "let" {
                    tokens.push(Token::Let)
                } else if tokens.len() > 0 && *tokens.last().unwrap() == Token::Let {
                    tokens.push(Token::Ident(current_string.clone()));
                }
                current_string.clear();
            },
            _ => {
                current_string.push(character);
            }
        }
    });
    tokens
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
}