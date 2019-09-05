use std::env;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use onedeck::onedeck_parser;

fn input_file(file_name: &Path) -> std::io::Result<String> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        println!("Please input file name.");
        return;
    }

    let file_name = Path::new(&args[1]);
    let src_str = input_file(file_name).unwrap();
    // TODO: コメントの除去
    let toplevels = onedeck_parser::ProgramParser::new().parse(&src_str).unwrap();
    let mut interp = onedeck::translation::setup(toplevels).unwrap();
    interp.run();
}
