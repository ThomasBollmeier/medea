#[derive(Debug)]
pub enum TokenValue {
    Number(f64, String), // value and original lexeme
    Boolean(bool),
    StringLiteral(String),
    Null,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
}

#[derive(Debug)]
pub struct Token {
    pub value: TokenValue,
    line: usize,
    column: usize,
}

impl Token {
    fn new(value: TokenValue, line: usize, column: usize) -> Self {
        Token { value, line, column }
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let (line, column) = (self.line, self.column);
        let next_char = self.advance()?;

        match next_char {
            '[' => Some(Token::new(TokenValue::LBracket, line, column)),
            ']' => Some(Token::new(TokenValue::RBracket, line, column)),
            '{' => Some(Token::new(TokenValue::LBrace, line, column)),
            '}' => Some(Token::new(TokenValue::RBrace, line, column)),
            ',' => Some(Token::new(TokenValue::Comma, line, column)),
            ':' => Some(Token::new(TokenValue::Colon, line, column)),
            '"' => self.scan_string(line, column),
            _ => self.scan_chars(next_char, line, column),
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.get(self.position).cloned()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(char) = self.input.get(self.position) {
            self.position += 1;
            if *char == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(*char)
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek_char() {
            if char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_string(&mut self, line: usize, column: usize) -> Option<Token> {
        let mut lexeme = String::new();
        let mut prev_char = '\0';

        while let Some(char) = self.advance() {
            if char == '"' {
                if prev_char == '\\' {
                    lexeme.pop(); // Remove the escape character
                    lexeme.push(char); // Add the quote
                    prev_char = char;
                    continue;
                }
                return Some(Token::new(TokenValue::StringLiteral(lexeme), line, column));
            } else {
                lexeme.push(char);
            }
            prev_char = char;
        }
        // If we reach here, the string was not closed
        None
    }

    fn scan_chars(&mut self, first: char, line: usize, column: usize) -> Option<Token> {
        let mut lexeme = String::new();
        lexeme.push(first);
        while let Some(char) = self.peek_char() {
            if char.is_alphanumeric() || char == '.' || char == '-' {
                lexeme.push(char);
                self.advance();
            } else {
                break;
            }
        }

        if lexeme == "true" {
            return Some(Token::new(TokenValue::Boolean(true), line, column));
        } else if lexeme == "false" {
            return Some(Token::new(TokenValue::Boolean(false), line, column));
        } else if lexeme == "null" {
            return Some(Token::new(TokenValue::Null, line, column));
        }

        if let Ok(number) = lexeme.parse::<f64>() {
            return Some(Token::new(TokenValue::Number(number, lexeme), line, column));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_json_null() {
        let mut lexer = Lexer::new("null");
        let token = lexer.next_token().unwrap();
        match token.value {
            TokenValue::Null => (),
            _ => panic!("Expected Null token"),
        }
    }

    #[test]
    fn test_lexer_json_boolean() {
        let mut lexer = Lexer::new("true false");
        let token1 = lexer.next_token().unwrap();
        match token1.value {
            TokenValue::Boolean(true) => (),
            _ => panic!("Expected Boolean true token"),
        }
        let token2 = lexer.next_token().unwrap();
        match token2.value {
            TokenValue::Boolean(false) => (),
            _ => panic!("Expected Boolean false token"),
        }
    }

    #[test]
    fn test_lexer_json_number() {
        let mut lexer = Lexer::new("123.45");
        let token = lexer.next_token().unwrap();
        match token.value {
            TokenValue::Number(num, _) => assert_eq!(num, 123.45),
            _ => panic!("Expected Number token"),
        }
    }

    #[test]
    fn test_lexer_json_string() {
        let mut lexer = Lexer::new(r#""Hello, World!""#);
        let token = lexer.next_token().unwrap();
        match token.value {
            TokenValue::StringLiteral(s) => assert_eq!(s, "Hello, World!"),
            _ => panic!("Expected StringLiteral token"),
        }
    }

    #[test]
    fn test_lexer_json_object() {
        let json_input = r#"{
            "firstName": "Theodore",
            "lastName": "Ballmiller",
            "age": 59,
            "languages": ["German", "English", "Rust", "JavaScript"],
            "isEmployed": true
        }"#;

        let mut lexer = Lexer::new(json_input);
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            println!("{:?}", token);
            tokens.push(token);
        }

        assert_eq!(tokens.len(), 29); // Just check that we got some tokens
    }

}