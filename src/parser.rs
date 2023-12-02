use std::iter::Product;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::ast::{Program, StatementNode, Node, LetStatement, Identifier};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Default::default(),
            peek_token: Default::default(),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: vec![]
        };

        while !self.cur_token(TokenKind::EOF) {
            match self.parse_statement() {
                Some(stmt) => program.statements.push(stmt),
                None => return None
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        match self.cur_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementNode> {
        let mut stmt: LetStatement = LetStatement {
            token: self.cur_token.clone(),
            name: Default::default(),
            value: None
        };

        return if !self.expect_peek(TokenKind::Ident) {
            None
        } else {
            stmt.name = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone()
            };

            if !self.expect_peek(TokenKind::Assign) {
                None
            } else {
                self.next_token();
                while !self.cur_token_is(TokenKind::Semicolon) {
                    self.next_token();
                }

                Some(StatementNode::Let(stmt))
            }

        }
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind) {
            self.next_token();
            return true
        }

        false
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn cur_token_is(&self, kind: TokenKind) -> bool {
        self.cur_token.kind == kind
    }

}

#[cfg(test)]
mod tests {
    use std::panic::panic_any;
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        match program {
            Some(p) => {
                assert_eq!(p.statements.len(), 3, "program.statements does not contain 3 statements. got {}", p.statements.len());

                let tests = vec!["x", "y", "foobar"];

                for (i, tt) in tests.iter().enumerate() {
                    let stmt = &p.statements[i];
                    test_let_statement(stmt, tt)
                }
            }
            None => panic!("parse program should not be None")
        }

    }

    fn test_let_statement(stmt: &StatementNode, exp: &str) {
        assert_eq!(stmt.token_literal(), "let", "toke literal not 'let' got {}", stmt.token_literal());
        match stmt {
            StatementNode::Let(e) => {
                assert_eq!(e.name.value, exp, "LetStatement name value not {}, got {}", exp, e.name.value);
                assert_eq!(e.name.token_literal(), exp, "LetStatement token literal not {}, got {}", exp, e.name.token_literal());
            }
            other => panic!("stmt is not LetStatement. got {:?}", other)
        }
    }
}