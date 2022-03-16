#![allow(non_snake_case)]
#[allow(unused_imports)]
#[macro_use] extern crate lang_1 as this;

use std::fs::File;
use std::io::Read;
use this::statics::*;
use this::tokenize::*;
use this::parser::*;
use this::static_colors::*;

fn main () {
	let mut file = File::open("code".to_owned()+FILE_EXT).expect("FAILURE");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("FAILURE");
	let contents : Vec<_> = contents.split("\n").collect();
	let tokens : Vec<Token> = tokenize(contents);
	let mut program : Parser = Parser::new(tokens);
	program.__init();
	println!("\n{}program output:\x1b[39m\n", INTERPRETER_LIME);
	program.run();
	println!("\n\n");
}