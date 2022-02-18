#[macro_use] extern crate lang_1 as this;
// extern crate regex;

// use std::fmt;
use std::fs::File;
use std::io::Read;
use this::statics::*;
use this::scopes::*;
use this::tokenize::*;
// use regex::Regex;

struct Parser {
	tokens : Vec<Token>,
	memory : VarScopes,
}

impl Parser {
	fn new (tokens : Vec<Token>) -> Parser {
		Parser {
			tokens : tokens,
			memory : VarScopes::new(),
		}
	}
	fn run (&mut self) -> u8 {
		let mut token_index : usize = 0;
		let tokens_length = self.tokens.len();
		loop {
			if token_index >= tokens_length {
				break;
			}
			token_index += 1;
		}
		return 0;
	}
}

fn main () {
	let mut file = File::open("code".to_owned()+FILE_EXT).expect("FAILURE");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("FAILURE");
	let contents: Vec<_> = contents.split("\n").collect();
	let tokens : Vec<Token> = tokenize(contents);
	let mut program : Parser = Parser::new(tokens);
	program.run();
	println!("\n\n");
}