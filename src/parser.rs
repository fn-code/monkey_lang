use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use crate::ast::{Program, StatementNode, Node, LetStatement, Identifier, ReturnStatement};

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            cur_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
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

        while !self.cur_token_is(TokenKind::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        match self.cur_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementNode> {
        let mut stmt: LetStatement = LetStatement {
            token: self.cur_token.clone(),
            name: Default::default(),
            value: None,
        };

        return if !self.expect_peek(TokenKind::Ident) {
            None
        } else {
            stmt.name = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
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
        };
    }

    fn parse_return_statement(&mut self) -> Option<StatementNode> {
        let stmt = StatementNode::Return(ReturnStatement {
            token: self.cur_token.clone(),
            return_value: Default::default(),
        });

        self.next_token();

        while !self.cur_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(stmt)
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind.clone()) {
            self.next_token();
            return true;
        }
        self.peek_error(kind);
        false
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn cur_token_is(&self, kind: TokenKind) -> bool {
        self.cur_token.kind == kind
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, kind: TokenKind) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", kind, self.peek_token.kind);
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
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
        check_parser_error(parser);

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

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 993322;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parser_error(parser);

        match program {
            Some(p) => {
                assert_eq!(p.statements.len(), 3, "program.statements does not contain 3 statements. got {}", p.statements.len());

                for stmt in p.statements {
                    match stmt {
                        StatementNode::Return(ret) => {
                            assert_eq!(ret.token_literal(), "return", "return statement token literal not 'return', got {}", ret.token_literal());
                        },
                        _ => panic!("stmt is not ReturnStatement. got {:?}", stmt)
                    }
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
        };
    }

    fn check_parser_error(parser: Parser) {
        let errors = parser.errors();
        if errors.len() == 0 {
            return;
        }

        for msg in errors.clone().into_iter() {
            eprintln!("parser error: {}", msg);
        }

        panic!("parser has {} errors", errors.len());
    }
}