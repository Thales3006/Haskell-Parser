#[derive(pest_derive::Parser)]
#[grammar = "haskell.pest"]
pub struct HaskellParser;

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Declaration {
        name: String,
        ty: TypeExpr,
    },

    Definition {
        name: String,
        args: Vec<Pattern>,
        body: Body,
    },
    Comment(String),
}

#[derive(Debug)]
pub enum Pattern {
    Ident(String),
    Literal(Literal),
    Wildcard,
}

#[derive(Debug)]
pub enum Body {
    Expression(Expression),
    Guards(Vec<Guard>),
}

#[derive(Debug)]
pub struct Guard {
    pub condition: Expression,
    pub result: Expression,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Ident(String),

    FuncCall {
        function: String,
        args: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Decimal(f64),
    Bool(bool),
    List(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Int,
    Float,
    Char,
    Bool,
    List(Box<TypeExpr>),
    Func(Vec<TypeExpr>),
}
