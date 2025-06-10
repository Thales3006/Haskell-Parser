use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};

use crate::ast::*;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left) | Op::infix(sub, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
    };
}

pub fn build_tree(ast: Pairs<'_, Rule>) -> Program {
    Program {
        statements: ast.map(build_statement).collect(),
    }
}

fn build_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::declaration => {
            let mut it = pair.into_inner();
            Statement::Declaration {
                name: it.next().unwrap().as_str().to_string(),
                ty: build_type(it.next().unwrap()),
            }
        }
        Rule::definition => {
            let mut it = pair.into_inner();
            Statement::Definition {
                name: it.next().unwrap().as_str().to_string(),
                args: vec![],
                body: Body::Expression(Expression::Atom(Atom::Literal(Literal::Integer(1)))),
            }
        }
        Rule::comment => Statement::Comment(pair.as_str().to_string()),
        Rule::EOI => Statement::Comment("End of Input".to_string()),
        
        rule => panic!("Statement error: {:?}", rule),
    }
}

fn build_type(pair: Pair<Rule>) -> TypeExpr {
    match pair.as_rule() {
        Rule::int_type => TypeExpr::Primitive(Primitive::Int),
        Rule::float_type => TypeExpr::Primitive(Primitive::Float),
        Rule::char_type => TypeExpr::Primitive(Primitive::Char),
        Rule::bool_type => TypeExpr::Primitive(Primitive::Bool),

        Rule::list_type => TypeExpr::List(Box::new(build_type(pair.into_inner().next().unwrap()))),
        Rule::func_type => TypeExpr::Func(pair.into_inner().map(build_type).collect()),
        rule => panic!("Type definition error {:?}", rule),
    }
}
