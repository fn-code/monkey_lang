use crate::token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };

        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position]
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();

        let token = match self.ch {
            '=' =>  {
                if self.peek_char() == '=' {
                    self.read_char();
                    token::Token { kind: token::TokenKind::Eq, literal: "==".to_string() }
                } else {
                    Lexer::new_token(token::TokenKind::Assign, self.ch)
                }
            },
            ';' => Lexer::new_token(token::TokenKind::Semicolon, self.ch),
            '(' => Lexer::new_token(token::TokenKind::LParen, self.ch),
            ')' => Lexer::new_token(token::TokenKind::RParen, self.ch),
            ',' => Lexer::new_token(token::TokenKind::Comma, self.ch),
            '+' => Lexer::new_token(token::TokenKind::Plus, self.ch),
            '{' => Lexer::new_token(token::TokenKind::LBrace, self.ch),
            '}' => Lexer::new_token(token::TokenKind::RBrace, self.ch),
            '\0' => token::Token{ kind: token::TokenKind::EOF, literal: "".to_string() },
            '-' => Lexer::new_token(token::TokenKind::Minus, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token::Token { kind: token::TokenKind::NotEq, literal: "!=".to_string() }
                } else {
                    Lexer::new_token(token::TokenKind::Bang, self.ch)
                }
            }
            '/' => Lexer::new_token(token::TokenKind::Slash, self.ch),
            '*' => Lexer::new_token(token::TokenKind::Asterisk, self.ch),
            '<' => Lexer::new_token(token::TokenKind::Lt, self.ch),
            '>' => Lexer::new_token(token::TokenKind::Gt, self.ch),
            _ => {
                return if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = token::lookup_ident(&literal);
                    return token::Token { kind: kind, literal: literal }
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    return token::Token { kind: token::TokenKind::Int, literal: literal }
                }else {
                    Lexer::new_token(token::TokenKind::Illegal, self.ch)
                }
            },
        };

        self.read_char();

        token
    }


    fn read_identifier(&mut self) -> String {
       let mut indentifier = String::new();
        while Lexer::is_letter(self.ch) {
            indentifier.push(self.ch);
            self.read_char();
        }

       indentifier
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }


    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while Lexer::is_digit(self.ch) {
            number.push(self.ch);
            self.read_char();
        }

        number
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn is_letter(ch: char) -> bool {
        Lexer::is_alphabetic(ch) || ch == '_'
    }

    fn is_alphabetic(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z'
    }

    fn new_token(kind: token::TokenKind, ch: char) -> token::Token {
        token::Token { kind: kind, literal: ch.to_string() }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::{Token, TokenKind};


    #[test]
    fn test_next_token() {
        let input = r#"
        let five = 5;
        let ten = 10;

        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        "#;

        let expected = vec![
            Token { kind: TokenKind::Let, literal: "let".to_string() },
            Token { kind: TokenKind::Ident, literal: "five".to_string() },
            Token { kind: TokenKind::Assign, literal: "=".to_string() },
            Token { kind: TokenKind::Int, literal: "5".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Let, literal: "let".to_string() },
            Token { kind: TokenKind::Ident, literal: "ten".to_string() },
            Token { kind: TokenKind::Assign, literal: "=".to_string() },
            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Let, literal: "let".to_string() },
            Token { kind: TokenKind::Ident, literal: "add".to_string() },
            Token { kind: TokenKind::Assign, literal: "=".to_string() },
            Token { kind: TokenKind::Function, literal: "fn".to_string() },
            Token { kind: TokenKind::LParen, literal: "(".to_string() },
            Token { kind: TokenKind::Ident, literal: "x".to_string() },
            Token { kind: TokenKind::Comma, literal: ",".to_string() },
            Token { kind: TokenKind::Ident, literal: "y".to_string() },
            Token { kind: TokenKind::RParen, literal: ")".to_string() },
            Token { kind: TokenKind::LBrace, literal: "{".to_string() },
            Token { kind: TokenKind::Ident, literal: "x".to_string() },
            Token { kind: TokenKind::Plus, literal: "+".to_string() },
            Token { kind: TokenKind::Ident, literal: "y".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
            Token { kind: TokenKind::RBrace, literal: "}".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Let, literal: "let".to_string() },
            Token { kind: TokenKind::Ident, literal: "result".to_string() },
            Token { kind: TokenKind::Assign, literal: "=".to_string() },
            Token { kind: TokenKind::Ident, literal: "add".to_string() },
            Token { kind: TokenKind::LParen, literal: "(".to_string() },
            Token { kind: TokenKind::Ident, literal: "five".to_string() },
            Token { kind: TokenKind::Comma, literal: ",".to_string() },
            Token { kind: TokenKind::Ident, literal: "ten".to_string() },
            Token { kind: TokenKind::RParen, literal: ")".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Bang, literal: "!".to_string() },
            Token { kind: TokenKind::Minus, literal: "-".to_string() },
            Token { kind: TokenKind::Slash, literal: "/".to_string() },
            Token { kind: TokenKind::Asterisk, literal: "*".to_string() },
            Token { kind: TokenKind::Int, literal: "5".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Int, literal: "5".to_string() },
            Token { kind: TokenKind::Lt, literal: "<".to_string() },
            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::Gt, literal: ">".to_string() },
            Token { kind: TokenKind::Int, literal: "5".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::If, literal: "if".to_string() },
            Token { kind: TokenKind::LParen, literal: "(".to_string() },
            Token { kind: TokenKind::Int, literal: "5".to_string() },
            Token { kind: TokenKind::Lt, literal: "<".to_string() },
            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::RParen, literal: ")".to_string() },
            Token { kind: TokenKind::LBrace, literal: "{".to_string() },
            Token { kind: TokenKind::Return, literal: "return".to_string() },
            Token { kind: TokenKind::True, literal: "true".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
            Token { kind: TokenKind::RBrace, literal: "}".to_string() },
            Token { kind: TokenKind::Else, literal: "else".to_string() },
            Token { kind: TokenKind::LBrace, literal: "{".to_string() },
            Token { kind: TokenKind::Return, literal: "return".to_string() },
            Token { kind: TokenKind::False, literal: "false".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },
            Token { kind: TokenKind::RBrace, literal: "}".to_string() },

            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::Eq, literal: "==".to_string() },
            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::Semicolon, literal: ";".to_string() },

            Token { kind: TokenKind::Int, literal: "10".to_string() },
            Token { kind: TokenKind::NotEq, literal: "!=".to_string() },
            Token { kind: TokenKind::Int, literal: "9".to_string() },

        ];


        let mut l = Lexer::new(input);

        for (idx, exp) in expected.into_iter().enumerate() {
            let recv_token = l.next_token();
            assert_eq!(exp.kind, recv_token.kind, "tests[{}] - tokentype wrong. expected={}, got={}", idx, exp.kind, recv_token.kind);
            assert_eq!(exp.literal, recv_token.literal, "tests[{}] - literal wrong. expected={}, got={}", idx, exp.literal, recv_token.literal)
        }
    }
}