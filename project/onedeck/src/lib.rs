#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub onedeck_parser);

pub mod syntax;
pub mod translation;
pub mod obj;
pub mod util;