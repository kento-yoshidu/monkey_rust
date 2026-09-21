use crate::token::{Token, TokenType};

pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };

        l.read_char();

        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn new_token(token_type: TokenType, ch: u8) -> Token {
        Token { token_type, literal: (ch as char).to_string() }
    }

    // chのTokenを作成する
    // 次の文字に進んでTokenを返す
    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            b'=' => Self::new_token(TokenType::Assign, self.ch),
            b';' => Self::new_token(TokenType::Semicolon, self.ch),
            b'(' => Self::new_token(TokenType::Lparen, self.ch),
            b')' => Self::new_token(TokenType::Rparen, self.ch),
            b',' => Self::new_token(TokenType::Comma, self.ch),
            b'+' => Self::new_token(TokenType::Plus, self.ch),
            b'{' => Self::new_token(TokenType::Lbrace, self.ch),
            b'}' => Self::new_token(TokenType::Rbrace, self.ch),
            0 => Token { token_type: TokenType::Eof, literal: String::new() },
            _ => Self::new_token(TokenType::Illegal, self.ch),
        };

        self.read_char();

        token
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase(TokenType, &'static str);

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = [
            TestCase(TokenType::Assign, "="),
            TestCase(TokenType::Plus, "+"),
            TestCase(TokenType::Lparen, "("),
            TestCase(TokenType::Rparen, ")"),
            TestCase(TokenType::Lbrace, "{"),
            TestCase(TokenType::Rbrace, "}"),
            TestCase(TokenType::Comma, ","),
            TestCase(TokenType::Semicolon, ";"),
            TestCase(TokenType::Eof, ""),
        ];

        let mut lexer = Lexer::new(input.to_string());

        for TestCase(token_type, literal) in tests {
            let token = lexer.next_token();

            assert_eq!(token_type, token.token_type);
            assert_eq!(literal, token.literal);
        }
    }
}
