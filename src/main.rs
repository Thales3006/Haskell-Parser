use std::fs;

use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "haskell.pest"]
pub struct HaskellParser;

fn main(){
    let file = fs::read_to_string("tests/test_0.hs")
        .expect("Error while reading the file");
        
        let _parsed = HaskellParser::parse(Rule::program, file.as_str());
}

mod tests {

    #[test]
    fn checkout_grammar() {
        use super::*;

        let file = fs::read_to_string("tests/test_0.hs")
        .expect("Error while reading the file");
        
        let parsed = HaskellParser::parse(Rule::program, file.as_str());
        assert!(parsed.is_ok());

        println!("{:#?}", parsed.unwrap());
    }
}