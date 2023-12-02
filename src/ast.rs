use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn print(&self) -> String;
}

#[derive(Debug)]
pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            StatementNode::Let(e) => e.token_literal(),
            StatementNode::Return(e) => e.token_literal(),
        }
    }

    fn print(&self) -> String {
        match self {
            StatementNode::Let(e) => e.print(),
            StatementNode::Return(e) => e.print(),
        }
    }
}


#[derive(Debug)]
pub enum ExpressionNode {
    IdentifierNode(Identifier)
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            ExpressionNode::IdentifierNode(e) => e.token_literal()
        }
    }

    fn print(&self) -> String {
        match self {
            ExpressionNode::IdentifierNode(e) => e.print()
        }
    }
}

pub struct Program {
    pub statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Let(e) => e.token_literal(),
                StatementNode::Return(e) => e.token_literal()
            }
        } else {
            "".to_string()
        }
    }

    fn print(&self) -> String {
        let mut output = String::new();
        for statement in &self.statements {
            output.push_str(&statement.print());
        }
        output
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<ExpressionNode>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.token_literal());
        output.push_str(" ");
        output.push_str(&self.name.print());
        output.push_str(" = ");
        match &self.value {
            Some(value) => output.push_str(&value.print()),
            None => output.push_str("None")
        }
        output.push_str(";");
        output
    }
}

#[derive(Debug, Default)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<ExpressionNode>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.token_literal());
        output.push_str(" ");
        match &self.return_value {
            Some(value) => output.push_str(&value.print()),
            None => output.push_str("None")
        }
        output.push_str(";");
        output
    }
}