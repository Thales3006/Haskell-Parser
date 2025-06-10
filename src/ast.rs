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
    pub condition: GuardCondition,
    pub result: Expression,
}

#[derive(Debug)]
pub enum GuardCondition {
    Otherwise,
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    BinaryOp {
        left: Box<Expression>,
        op: BinOp,
        right: Box<Expression>,
    },
    FuncCall {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    InfixFuncCall {
        left: Box<Expression>,
        function: String,
        right: Box<Expression>,
    },
    Atom(Atom),
}

#[derive(Debug)]
pub enum Atom {
    Paren(Box<Expression>),
    Literal(Literal),
    Ident(String),
}

#[derive(Debug)]
pub enum Literal {
    Integer(i64),
    Decimal(f64),
    Bool(bool),
    List(Vec<Expression>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    Lesser,
    GreaterEq,
    LesserEq,
    And,
    Or,
    Xor,
    Cons,
    Concat,
    Composition,
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Primitive(Primitive),
    List(Box<TypeExpr>),
    Func(Vec<TypeExpr>),
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Int,
    Float,
    Char,
    Bool,
}

#[derive(Debug)]
pub enum Expr {
    Integer(i32),
    Float(f32),
    Bool(bool),
    List(Vec<Expr>),
    Function(String, Vec<Expr>),
    BinOp {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
}
