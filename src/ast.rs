use pest::pratt_parser::PrattParser;

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

    TypeDeclaration {
        name: String,
        constructors: Vec<ConstructDecl>,
    },
    Comment(String),
}

#[derive(Debug)]
pub struct ConstructDecl {
    pub name: String,
    pub ty: TypeExpr,
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Int,
    Float,
    Char,
    Bool,
    List(Box<TypeExpr>),
    Func(Vec<TypeExpr>),
    CustomType(String),
    Generic(String),
}

#[derive(Debug)]
pub enum Pattern {
    Ident(String),
    Literal(Literal),
    Wildcard,
    Constructor { name: String, args: Vec<Pattern> },
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

lazy_static::lazy_static! {
    pub static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(application, Left))
            .op(Op::infix(or, Left))
            .op(Op::infix(and, Left))
            .op(
                Op::infix(less_or_equal, Left) | Op::infix(great_or_equal, Left)
                |Op::infix(lesser, Left) | Op::infix(greater, Left)
                |Op::infix(equal, Left) | Op::infix(not_equal, Left)
            )
            .op(Op::infix(cons, Left) | Op::infix(concat, Left))
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::prefix(negative))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
            .op(Op::infix(exp_int, Left) | Op::infix(exp_float, Left) | Op::infix(exp_frac, Left))
            .op(Op::infix(composition, Left) | Op::infix(indexing, Left))
    };
}
