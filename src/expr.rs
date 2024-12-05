use crate::scanner::Token;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Nil,
}

pub enum Expr {
    Binary {
        lef: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
