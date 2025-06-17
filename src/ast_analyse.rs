use std::{collections::HashMap, vec};

use crate::ast::*;

#[derive(Debug)]
pub struct Symbol {
    pub ty: TypeExpr,
    pub explicit: bool,
    pub instances: Vec<Instance>,
}

#[derive(Debug)]
pub struct Instance {
    pub args: Vec<Pattern>,
    pub local: SymTab,
    pub body: Body,
}

#[derive(Debug)]
pub struct Class {
    pub functions: Vec<(String, TypeExpr)>,
}

#[derive(Debug)]
pub struct Type {
    pub instances: Vec<String>,
}

type SymTab = HashMap<String, Symbol>;
type ClassTab = HashMap<String, Class>;
type TypeTab = HashMap<String, Type>;

pub fn analyze_program(program: Program) -> Result<(SymTab, ClassTab, TypeTab), String> {
    let mut symbol_table: SymTab = HashMap::new();
    let mut class_table: ClassTab = HashMap::new();
    let mut type_table: TypeTab = HashMap::new();

    for stmt in program.statements {
        analyze_stmt(stmt, &mut symbol_table, &mut class_table, &mut type_table)?;
    }

    Ok((symbol_table, class_table, type_table))
}

fn analyze_stmt(
    stmt: Statement,
    sym_tab: &mut SymTab,
    class_tab: &mut ClassTab,
    typ_tab: &mut TypeTab,
) -> Result<(), String> {
    match stmt {
        Statement::Declaration(decl) => analyze_decl(decl, sym_tab, class_tab, typ_tab),
        Statement::Definition(def) => analyze_def(def, sym_tab, class_tab, typ_tab),
        Statement::TypeDeclaration { .. } => Ok(()),
        Statement::Comment(_) => Ok(()),
    }
}

fn analyze_decl(
    decl: Declaration,
    sym_tab: &mut SymTab,
    _class_tab: &mut ClassTab,
    _typ_tab: &mut TypeTab,
) -> Result<(), String> {
    if let Some(_) = sym_tab.get_mut(&decl.name) {
        Err(format!(
            "Symbol: '{}' was declared more than once",
            decl.name
        ))
    } else {
        sym_tab.insert(
            decl.name.clone(),
            Symbol {
                ty: decl.ty.clone(),
                explicit: true,
                instances: vec![],
            },
        );
        Ok(())
    }
}

fn analyze_def(
    def: Definition,
    sym_tab: &mut SymTab,
    class_tab: &mut ClassTab,
    typ_tab: &mut TypeTab,
) -> Result<(), String> {
    if let Some(symbol) = sym_tab.get_mut(&def.name) {
        symbol.instances.push(Instance {
            local: local_table(&def, symbol, class_tab, typ_tab)?,
            args: def.args,
            body: def.body,
        });
        Ok(())
    } else {
        Err(format!("Symbol: '{}' was not declared before", def.name))
    }
}

fn local_table(
    def: &Definition,
    symbol: &Symbol,
    _class_tab: &ClassTab,
    _typ_tab: &TypeTab,
) -> Result<SymTab, String> {
    let mut locals = HashMap::new();
    match symbol.ty {
        TypeExpr::Func(ref types) => {
            if def.args.len() != types.len() - 1 {
                return Err(format!(
                    "Definition: '{}' does not have the right quantity of arguments",
                    def.name
                ));
            }
            for (pat, ty) in def.args.iter().zip(types.iter()) {
                // Assuming Pattern::Ident(String)
                if let Pattern::Ident(name) = pat {
                    locals.insert(
                        name.clone(),
                        Symbol {
                            ty: ty.clone(),
                            explicit: true,
                            instances: vec![],
                        },
                    );
                }
            }
        }
        _ => (),
    }
    Ok(locals)
}
