mod ast;
mod ast_build;

use std::fs;

use crate::{
    ast::{HaskellParser, Rule},
    ast_build::build_tree,
};
use inline_colorization::*;
use pest::{Parser, iterators::Pairs};

fn main() {
    let file = fs::read_to_string("tests/test_0.hs").expect("Error while reading the file");

    let parsed = HaskellParser::parse(Rule::program, file.as_str());
    match parsed {
        Ok(parsed) => {
            println!("{style_bold}{:#?}{style_reset}", build_tree(parsed));
        }
        Err(err) => {
            println!("{style_bold}{color_red}ERRO! {style_reset}{:#?}", err);
        }
    }
}

pub fn print_ast(ast: Pairs<'_, Rule>, offset: usize) {
    let spaces = if offset > 0 {
        format!("{}|   ", "    ".repeat(offset))
    } else {
        String::new()
    };

    for pair in ast {
        println!(
            "{spaces}{color_green}{:?} {style_bold}{color_yellow}{:?}{style_reset}",
            pair.as_rule(),
            pair.as_str()
        );
        print_ast(pair.into_inner(), offset + 1);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn single_file() {
        use super::*;

        let file = fs::read_to_string("tests/test_0.hs").expect("Error while reading the file");

        let parsed = HaskellParser::parse(Rule::program, file.as_str());
        if let Err(error) = &parsed {
            println!("{:#?}", error);
        }

        assert!(parsed.is_ok());
        print_ast(parsed.unwrap(), 0);
    }

    #[test]
    fn all_tests() {
        use super::*;

        let test_dir = "tests/";

        let entries = fs::read_dir(test_dir).expect("Erro ao ler o diretório de testes");

        for entry in entries {
            let entry = entry.expect("Error while reading the folder");
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hs") {
                let file_path = path.to_string_lossy();

                let file = fs::read_to_string(&path).expect("Error while reading the file");

                let parsed = HaskellParser::parse(Rule::program, file.as_str());
                assert!(parsed.is_ok(), "Erro no parsing de '{}'", file_path);
            }
        }
    }
}
