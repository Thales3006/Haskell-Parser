use pest::{
    iterators::{Pair, Pairs},
    pratt_parser::PrattParser,
};
use std::str::FromStr;

use crate::ast::*;

lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        PrattParser::new()
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
                args: it.next().unwrap().into_inner().map(build_pattern).collect(),
                body: build_body(it.next().unwrap()),
            }
        }
        Rule::comment => Statement::Comment(pair.as_str().to_string()),
        Rule::EOI => Statement::Comment("End of Input".to_string()),

        rule => panic!(
            "Statement error: rule found: {:?}, string:{:?}",
            rule,
            pair.as_str()
        ),
    }
}

fn build_body(pair: Pair<Rule>) -> Body {
    match pair.as_rule() {
        Rule::expression => Body::Expression(build_expression(pair)),
        Rule::guards => {
            let it = pair.into_inner();
            Body::Guards(it.map(build_guard).collect())
        }
        _ => unreachable!(),
    }
}

fn build_guard(pair: Pair<Rule>) -> Guard {
    let mut it = pair.into_inner();
    let condition = it.next().unwrap();
    Guard {
        condition: match condition.as_rule() {
            Rule::otherwise => Expression::Literal(Literal::Bool(true)),
            Rule::expression => build_expression(condition),
            _ => unreachable!(),
        },
        result: build_expression(it.next().unwrap()),
    }
}

fn build_type(pair: Pair<Rule>) -> TypeExpr {
    match pair.as_rule() {
        Rule::int_type => TypeExpr::Int,
        Rule::float_type => TypeExpr::Float,
        Rule::char_type => TypeExpr::Char,
        Rule::bool_type => TypeExpr::Bool,

        Rule::list_type => TypeExpr::List(Box::new(build_type(pair.into_inner().next().unwrap()))),
        Rule::func_type => TypeExpr::Func(pair.into_inner().map(build_type).collect()),

        rule => panic!(
            "Type definition error: rule found: {:?}, string:{:?}",
            rule,
            pair.as_str()
        ),
    }
}

fn build_pattern(pair: Pair<Rule>) -> Pattern {
    match pair.as_rule() {
        Rule::ident => Pattern::Ident(pair.as_str().to_string()),
        Rule::literal => Pattern::Literal(build_literal(pair.into_inner().next().unwrap())),
        Rule::wildcard => Pattern::Wildcard,

        rule => panic!(
            "Pattern error: rule found: {:?}, string:{:?}",
            rule,
            pair.as_str()
        ),
    }
}

fn build_literal(pair: Pair<Rule>) -> Literal {
    match pair.as_rule() {
        Rule::integer => Literal::Integer(i64::from_str(pair.as_str()).unwrap()),
        Rule::decimal => Literal::Decimal(f64::from_str(pair.as_str()).unwrap()),
        Rule::bool => Literal::Bool(match pair.into_inner().next().unwrap().as_rule(){
            Rule::true_literal => true,
            Rule::false_literal => false,
            _ => unreachable!(),
        }),
        Rule::list => Literal::List(pair.into_inner().map(build_expression).collect()),

        rule => panic!(
            "Literal error: rule found: {:?}, string:{:?}",
            rule,
            pair.as_str()
        ),
    }
}

fn build_expression(pair: Pair<Rule>) -> Expression {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            Rule::func_prefix => {
                let mut it = primary.into_inner();
                Expression::FuncCall {
                    function: it.next().unwrap().as_str().to_string(),
                    args: it.map(build_atom).collect(),
                }
            }
            _ => build_atom(primary),
        })
        .map_prefix(|prefix, rhs|Expression::FuncCall {
            function: prefix.as_str().to_string(),
            args: vec![rhs],
        })
        .map_infix(|lhs, infix, rhs| Expression::FuncCall {
            function: infix.as_str().to_string(),
            args: vec![lhs, rhs],
        })
        .parse(pair.into_inner())
}

fn build_atom(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => build_expression(pair),
        Rule::literal => Expression::Literal(build_literal(pair.into_inner().next().unwrap())),
        Rule::ident => Expression::Ident(pair.as_str().to_string()),

        _ => unreachable!(),
    }
}
