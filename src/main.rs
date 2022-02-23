#![allow(non_snake_case)]
extern crate lang_1 as this;

use std::fs::File;
use std::io::Read;
use this::statics::*;
use this::scopes::*;
use this::tokenize::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Parser {
	tokens : Vec<Token>,
	memory : VarScopes,
	SEPTOK : Token,
	UDFTOK : Token,
	terminating_newlines : u32,
	print_sep_spaces : u32,
}

impl Parser {
	fn new (tokens : Vec<Token>) -> Parser {
		Parser {
			tokens : tokens,
			memory : VarScopes::new(),
			SEPTOK : Token::news(SEP, ",", BASE_TOKEN),
			UDFTOK : Token::news(UDF, "UDF", BASE_TOKEN),
			terminating_newlines : 1,
			print_sep_spaces : 1,
		}
	}
	fn __fault (&self) -> Token {
		return Token::new(UDF, String::from("UDF"), BASE_TOKEN);
	}
	fn addition (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if NUMBER_RE.is_match(&v1) {
			if !NUMBER_RE.is_match(&v2) {
				panic!("mismatched types");
			}
			let mut v : i64 = v1.parse().unwrap();
			v += v2.parse::<i64>().unwrap();
			return Token::new(LIT, v.to_string(), BASE_TOKEN);
		}
		return self.__fault();
	}
	fn subtraction (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if !(NUMBER_RE.is_match(&v1) && NUMBER_RE.is_match(&v2)) {
			return self.__fault();
		}
		let mut v : i64 = v1.parse().unwrap();
		v -= v2.parse::<i64>().unwrap();
		return Token::new(LIT, v.to_string(), BASE_TOKEN);
	}
	fn multiplication (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if NUMBER_RE.is_match(&v1) {
			if !NUMBER_RE.is_match(&v2) {
				panic!("mismatched types");
			}
			let mut v : i64 = v1.parse().unwrap();
			v *= v2.parse::<i64>().unwrap();
			return Token::new(LIT, v.to_string(), BASE_TOKEN);
		}
		return self.__fault();
	}
	fn division (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if !(NUMBER_RE.is_match(&v1) && NUMBER_RE.is_match(&v2)) {
			return self.__fault();
		}
		let v : i64 = v1.parse().unwrap();
		return Token::new(LIT, (v/v2.parse::<i64>().unwrap()).to_string(), BASE_TOKEN);
	}
	fn operation (&self, operand : &str, v1 : String, v2 : String) -> Token {
		if operand == "+" {
			return self.addition(v1, v2);
		} else if operand == "-" {
			return self.subtraction(v1, v2);
		} else if operand == "*" {
			return self.multiplication(v1, v2);
		} else if operand == "/" {
			return self.division(v1, v2);
		}
		return self.__fault();
	}
	fn assignment_operation (&self, operand : &str, v1 : String, v2 : String) -> Token {
		return self.operation(&operand.chars().collect::<Vec<char>>()[0].to_string(), v1, v2);
	}
	fn deref (&self, mut t : Token) -> Token {
		let mut names : HashMap<String, u8> = HashMap::new();
		loop {
			if t.id != REF {
				return t;
			}
			if names.insert(t.value.clone(), 0).is_some() {
				return self.__fault();
			}
			t = self.memory.get(&t.value);
		}
	}
	fn gen_op (&self, mut t1 : Token, t2 : Token, mut t3 : Token) -> Token {
		if t1.id == REF {
			t1 = self.deref(t1);
		}
		if t3.id == REF {
			t3 = self.deref(t3);
		}
		if (t2.id > 6 || t2.id < 5) || (t1.id > 4 && t1.id < 7) || (t3.id > 4 && t3.id < 7) {
			return self.__fault();
		}
		if t2.id == 5 {
			return self.operation(&t2.value, t1.value, t3.value);
		}
		return self.__fault();
	}
	fn printop (&self, mut i : usize) -> usize {
		let l = self.tokens.len();
		let mut new_arg_ready = true;
		loop {
			if i >= l {
				break;
			}
			if self.tokens[i].id == NLN {
				break;
			}
			if self.tokens[i] == self.SEPTOK {
				new_arg_ready = true;
				i += 1;
				continue;
			}
			if new_arg_ready {
				new_arg_ready = false;
				let mut copt : Token = self.UDFTOK.clone();
				loop {
					if i >= l {
						break;
					}
					if self.tokens[i].id == NLN {
						i -= 1;
						break;
					}
					if self.tokens[i] == self.SEPTOK {
						i -= 1;
						break;
					}
					if copt == self.UDFTOK {
						copt = self.tokens[i].clone();
					} else {
						if (self.tokens[i].id > 6 || self.tokens[i].id < 5) && (self.tokens[i].id != KEY || self.tokens[i].value != "of") {
							copt = self.tokens[i].clone();
						} else if self.tokens[i].id == KEY {
							copt = self.parse_of(i);
							i += 1;
						} else {
							copt = self.gen_op(copt, self.tokens[i].clone(), self.tokens[i+1].clone());
							i += 1;
						}
					}
					i += 1;
				}
				if copt.id == REF {
					copt = self.deref(copt);
				}
				let b : Vec<char> = copt.value.chars().collect();
				if b[0] == '"' && b[b.len()-1] == '"' {
					copt.value = copt.value[1..copt.value.len()-1].to_string();
				}
				print!("{}{}", copt.value, String::from(" ").repeat(self.print_sep_spaces as usize));
			}
			i += 1;
		}
		print!("{}", String::from("\n").repeat(self.terminating_newlines as usize));
		return i;
	}
	fn dumpscope (&self, mut i : usize) -> usize {
		i += 1;
		let c = self.tokens[i].value.clone();
		if c == "0" {
			self.memory.dump(0);
		} else if c == "1" {
			self.memory.dump(1);
		} else if c == "2" {
			self.memory.dump(2);
		}
		return i;
	}
	fn parse_of (&self, i : usize) -> Token {
		let mut t : Token = self.tokens[i+1].clone();
		if t.id == REF {
			t = self.deref(t);
		}
		return match t.tt() {LIST_TOKEN => t.get(self.tokens[i-1].value.parse::<usize>().unwrap()), _ => t.getd(self.tokens[i-1].value.clone())};
	}
	fn run (&mut self) -> u8 {
		let mut token_index : usize = 0;
		let tokens_length = self.tokens.len();
		loop {
			if token_index >= tokens_length {
				break;
			}
			// meta properties
			if self.tokens[token_index].id == MET {
				if self.tokens[token_index].value == "terminating_newlines" {
					self.terminating_newlines = self.tokens[token_index+1].value.parse::<u32>().unwrap();
				} else if self.tokens[token_index].value == "print_sep_spaces" {
					self.print_sep_spaces = self.tokens[token_index+1].value.parse::<u32>().unwrap();
				}
			// handle keywords
			} else if self.tokens[token_index].id == KEY { 
				if self.tokens[token_index].value == "print" {
					let x : usize = self.printop(token_index);
					token_index = x;
				} else if self.tokens[token_index].value == "dumpscope" {
					let x : usize = self.dumpscope(token_index);
					token_index = x;
				}
			// handle variable assignment
			} else if self.tokens[token_index].id == ASS {
				let varname = &self.tokens[token_index-1].value;
				let operand = &self.tokens[token_index].value;
				// seperate simple assignment from modification to a value
				if operand == "=" {
					self.memory.set(varname, self.tokens[token_index+1].clone());
				} else {
					self.memory.set(varname, self.assignment_operation(&operand, self.memory.get(varname).value, self.tokens[token_index+1].value.clone()));
				}
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
	println!("\n\x1b[38;2;0;255;0mprogram output:\x1b[39m\n");
	program.run();
	println!("\n\n");
}