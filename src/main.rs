#[macro_use] extern crate lang_1 as this;

use std::fmt;
use std::fs::File;
use std::io::Read;
use this::statics::*;
use this::scopes::*;

struct Parser {
	tokens : Vec<Token>,
}

impl Parser {
	fn new (tokens : Vec<Token>) -> Parser {
		Parser {
			tokens : tokens,
		}
	}
}

fn tokenize (lines : Vec<&str>) -> Vec<Token> {
	let mut final_vec = Vec::new();
	let mut line_index = 0;
	let lines_len_total = lines.len();
	'outer : loop {
		if line_index >= lines_len_total {
			break 'outer;
		}
		let line : Vec<char> = lines[line_index].chars().collect();
		let mut i : usize = 0usize;
		let line_len = line.len();
		'inner : loop {
			if i >= line_len {
				break 'inner;
			}
			print!("{}", line[i].to_string() + " ");
			i += 1;
		}
		println!("");
		line_index += 1;
	}
	return final_vec;
}

fn main () {
	let mut file = File::open("code".to_owned()+FILE_EXT).expect("FAILURE");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("FAILURE");
	let contents: Vec<_> = contents.split("\n").collect();
	let mut i = 0;
	let l = contents.len();
	loop {
		if i >= l {
			break;
		}
		printv!(contents[i]);
		i += 1;
	}
	tokenize(contents);
}