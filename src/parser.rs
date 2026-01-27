use std::collections::VecDeque;
use anyhow::Result;
use crate::json_value::JsonValue;
use crate::lexer::{Lexer, Token, TokenValue};

pub fn parse_json(input: &str) -> Result<JsonValue> {
    let mut parser = Parser::new(input);
    parser.parse()
}

struct Parser {
    lexer: Lexer,
    tokens: VecDeque<Token>,
}

impl Parser {
    fn new(input: &str) -> Self {
        Parser {
            lexer: Lexer::new(input),
            tokens: VecDeque::new(),
        }
    }

    fn parse(&mut self) -> Result<JsonValue> {
        let next_token = self.next_token()?;
        match next_token.value {
            TokenValue::Null => Ok(JsonValue::Null),
            TokenValue::Boolean(b) => Ok(JsonValue::Bool(b)),
            TokenValue::Number(num) => Ok(JsonValue::Number(num)),
            TokenValue::StringLiteral(s) => Ok(JsonValue::String(s)),
            TokenValue::LBracket => self.parse_array(),
            TokenValue::LBrace => self.parse_object(),
            _ => Err(anyhow::anyhow!("Unexpected token: {:?}", next_token)),
        }
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        let mut member_names = Vec::new();
        let mut members  = std::collections::HashMap::new();
        let mut member_expected = false;

        loop {
            let next_token = self.next_token()?;
            match next_token.value {
                TokenValue::RBrace => {
                    if !member_expected {
                        break
                    }
                    return Err(anyhow::anyhow!("Trailing comma in object is not allowed"));
                },
                TokenValue::StringLiteral(key) => {
                    if members.contains_key(&key) {
                        return Err(anyhow::anyhow!("Duplicate key in object: {}", key));
                    }
                    member_names.push(key.clone());
                    let colon_token = self.next_token()?;
                    if let TokenValue::Colon = colon_token.value {
                        let value = self.parse()?;
                        members.insert(key, value);
                    } else {
                        return Err(anyhow::anyhow!("Expected ':', found {:?}", colon_token));
                    };
                    let next_token = self.next_token()?;
                    match next_token.value {
                        TokenValue::Comma => {
                            member_expected = true;
                            continue
                        },
                        TokenValue::RBrace => break,
                        _ => {
                            return Err(anyhow::anyhow!("Expected ',' or '}}', found {:?}", next_token));
                        }
                    }
                }
                _ => {
                    return Err(anyhow::anyhow!("Expected string key or '}}', found {:?}", next_token));
                }
            }
        }

        Ok(JsonValue::Object(member_names, members))
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        let mut elements = Vec::new();
        let mut element_expected = false;

        loop {
            let next_token = self.peek_token()?;
            match next_token.value {
                TokenValue::RBracket => {
                    if !element_expected {
                        self.next_token()?; // consume RBracket
                        break;
                    } else {
                        return Err(anyhow::anyhow!("Trailing comma in array is not allowed"));
                    }
                }
                _ => {
                    let element = self.parse()?;
                    elements.push(element);
                    element_expected = false;

                    let next_token = self.peek_token()?;
                    match next_token.value {
                        TokenValue::Comma => {
                            self.next_token()?; // consume Comma
                            element_expected = true;
                        }
                        TokenValue::RBracket => continue,
                        _ => return Err(anyhow::anyhow!("Expected ',' or ']', found {:?}", next_token)),
                    }
                }
            }
        }

        Ok(JsonValue::Array(elements))
    }

    fn next_token(&mut self) -> Result<Token> {
        if let Some(token) = self.tokens.pop_front() {
            Ok(token)
        } else {
            self.lexer.next_token().ok_or_else(|| anyhow::anyhow!("Unexpected end of input"))
        }
    }

    fn peek_token(&mut self) -> Result<&Token> {
        if self.tokens.is_empty() {
            if let Some(token) = self.lexer.next_token() {
                self.tokens.push_back(token);
            } else {
                return Err(anyhow::anyhow!("Unexpected end of input"));
            }
        }
        Ok(self.tokens.front().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_parse_json_null() {
        let json = "null";
        let result = parse_json(json).unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_json_boolean() {
        let json_true = "true";
        let result_true = parse_json(json_true).unwrap();
        assert_eq!(result_true, JsonValue::Bool(true));
        let json_false = "false";
        let result_false = parse_json(json_false).unwrap();
        assert_eq!(result_false, JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_json_number() {
        let json = "123.45";
        let result = parse_json(json).unwrap();
        assert_eq!(result, JsonValue::Number(123.45));
    }

    #[test]
    fn test_parse_json_string() {
        let json = r#""Hello, World!""#;
        let result = parse_json(json).unwrap();
        assert_eq!(result, JsonValue::String("Hello, World!".to_string()));
    }

    #[test]
    fn test_parse_json_array() {
        let json = r#"[null, true, 123.45, "Hello"]"#;

        let result = parse_json(json).unwrap();
        assert_eq!(result, JsonValue::Array(vec![
            JsonValue::Null,
            JsonValue::Bool(true),
            JsonValue::Number(123.45),
            JsonValue::String("Hello".to_string())
        ]));
    }

    #[test]
    fn test_parse_json_object() {
        let json = r#"{"key1": null, "key2": true, "key3": 123.45, "key4": "Hello"}"#;
        let result = parse_json(json).unwrap();
        let expected_names = vec![
            "key1".to_string(),
            "key2".to_string(),
            "key3".to_string(),
            "key4".to_string(),
        ];
        let mut expected = std::collections::HashMap::new();
        expected.insert("key1".to_string(), JsonValue::Null);
        expected.insert("key2".to_string(), JsonValue::Bool(true));
        expected.insert("key3".to_string(), JsonValue::Number(123.45));
        expected.insert("key4".to_string(), JsonValue::String("Hello".to_string()));
        assert_eq!(result, JsonValue::Object(expected_names, expected));
    }

    #[test]
    fn test_parse_nested_json() {
        let json = r#"
        {
            "name": "Thomas",
            "age": 59,
            "is_student": false,
            "courses": ["Math", "Physics"],
            "address": {
                "country": "Germany",
                "state": "Lower Saxony",
                "city": "Hannover",
                "zip": "31000"
            }
        }
        "#;
        let result = parse_json(json).unwrap();
        let expected_names = vec![
            "name".to_string(),
            "age".to_string(),
            "is_student".to_string(),
            "courses".to_string(),
            "address".to_string(),
        ];

        let mut expected = std::collections::HashMap::new();

        expected.insert("name".to_string(), JsonValue::String("Thomas".to_string()));
        expected.insert("age".to_string(), JsonValue::Number(59.0));
        expected.insert("is_student".to_string(), JsonValue::Bool(false));
        expected.insert("courses".to_string(), JsonValue::Array(vec![
            JsonValue::String("Math".to_string()),
            JsonValue::String("Physics".to_string()),
        ]));

        let address_names = vec![
            "country".to_string(),
            "state".to_string(),
            "city".to_string(),
            "zip".to_string(),
        ];
        let mut address = std::collections::HashMap::new();
        address.insert("country".to_string(), JsonValue::String("Germany".to_string()));
        address.insert("state".to_string(), JsonValue::String("Lower Saxony".to_string()));
        address.insert("city".to_string(), JsonValue::String("Hannover".to_string()));
        address.insert("zip".to_string(), JsonValue::String("31000".to_string()));
        expected.insert("address".to_string(), JsonValue::Object(address_names, address));

        assert_eq!(result, JsonValue::Object(expected_names, expected));

        dbg!(result);
    }
}
