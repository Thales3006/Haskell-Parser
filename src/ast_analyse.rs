use std::collections::HashMap;

use crate::ast::*;

#[derive(Debug)]
pub struct Symbol {
    ty: TypeExpr,
}

#[derive(Debug)]
pub struct Class {
    functions: Vec<(String, TypeExpr)>,
}

#[derive(Debug)]
pub struct Type {
    instances: Vec<String>,
}

type SymTab = HashMap<String, Symbol>;
type ClassTab = HashMap<String, Class>;
type TypeTab = HashMap<String, Type>;

pub fn analyze_program(program: &Program) -> Result<(SymTab, ClassTab, TypeTab), String> {
    let mut symbol_table: SymTab = HashMap::new();
    let mut class_table: ClassTab = HashMap::new();
    let mut type_table: TypeTab = HashMap::new();

    for stmt in &program.statements {
        analyze_stmt(stmt, &mut symbol_table, &mut class_table, &mut type_table)?;
    }

    Ok((symbol_table, class_table, type_table))
}

fn analyze_stmt(
    stmt: &Statement,
    sym_tab: &mut SymTab,
    class_tab: &mut ClassTab,
    typ_tab: &mut TypeTab,
) -> Result<(), String> {
    match stmt {
        Statement::Declaration(decl) => analyze_decl(decl, sym_tab, class_tab, typ_tab),
        Statement::TypeDeclaration { .. } => Ok(()),
        Statement::Definition { .. } => Ok(()),
        Statement::Comment(_) => Ok(()),
    }
}

fn analyze_decl(
    decl: &Declaration,
    sym_tab: &mut SymTab,
    class_tab: &mut ClassTab,
    typ_tab: &mut TypeTab,
) -> Result<(), String> {
    if sym_tab.contains_key(&decl.name) {
        return Err(format!(
            "Symbol: '{}' was declared more than once",
            decl.name
        ));
    }

    sym_tab.insert(
        decl.name.clone(),
        Symbol {
            ty: decl.ty.clone(),
        },
    );

    Ok(())
}
