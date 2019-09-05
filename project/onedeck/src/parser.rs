use super::syntax::*;
use super::onedeck_parser;


pub fn parse(src: &str) -> Vec<Toplevel> {
    onedeck_parser::ProgramParser::new().parse(src).unwrap()
}