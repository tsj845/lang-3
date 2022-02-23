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
	fn printop (&self, mut i : usize, tokens : &Vec<Token>) -> usize {
		let l = tokens.len();
		let mut new_arg_ready = true;
		loop {
			if i >= l {
				break;
			}
			let token = tokens[i].clone();
			if token.id == NLN {
				break;
			}
			if token == self.SEPTOK {
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
					let token = tokens[i].clone();
					if token.id == NLN {
						i -= 1;
						break;
					}
					if token == self.SEPTOK {
						i -= 1;
						break;
					}
					if copt == self.UDFTOK {
						copt = token.clone();
					} else {
						if (token.id > 6 || token.id < 5) && (token.id != KEY || token.value != "of") {
							copt = token.clone();
						} else if token.id == KEY {
							copt = self.parse_of(i, &tokens);
							i += 1;
						} else {
							copt = self.gen_op(copt, token.clone(), tokens[i+1].clone());
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
	fn dumpscope (&self, mut i : usize, tokens : &Vec<Token>) -> usize {
		i += 1;
		let c = tokens[i].value.clone();
		if c == "0" {
			self.memory.dump(0);
		} else if c == "1" {
			self.memory.dump(1);
		} else if c == "2" {
			self.memory.dump(2);
		}
		return i;
	}
	fn parse_of (&self, i : usize, tokens : &Vec<Token>) -> Token {
		let mut t : Token = tokens[i+1].clone();
		if t.id == REF {
			t = self.deref(t);
		}
		return match t.tt() {LIST_TOKEN => t.get(self.tokens[i-1].value.parse::<usize>().unwrap()), _ => t.getd(self.tokens[i-1].value.clone())};
	}
	fn func_call (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let t : Vec<Token> = self.deref(tokens[i-1].clone()).list.as_ref().unwrap().clone();
		let l = tokens.len();
		let mut depth = 0;
		let mut atoks : Vec<Token> = Vec::new();
		loop {
			if i >= l {
				break;
			}
			if tokens[i].id == PAR {
				if tokens[i].value == ")" {
					depth -= 1;
					if depth == 0 {
						break;
					}
					atoks.push(tokens.remove(i));
				} else if tokens[i].value == "(" {
					atoks.push(tokens.remove(i));
					depth += 1;
				}
			} else {
				atoks.push(tokens.remove(i));
			}
			// i += 1;
		}
		self.memory.new_scope();
		let mut j : usize = 0;
		let k = t.len();
		let mut o : usize = 0;
		let p = atoks.len();
		loop {
			if j >= k {
				break;
			}
			if t[j].id == SEP && t[j].value == "*" {
				break;
			}
			if t[j].id == SEP {
				j += 1;
				continue;
			}
			let vname = t[j].value.clone();
			self.memory.flag_var(vname.clone(), 0u8);
			let mut atoksn : Vec<Token> = Vec::new();
			loop {
				if o >= p {
					break;
				}
				if atoks[o].id == SEP {
					o += 1;
					break;
				}
				atoksn.push(atoks[o].clone());
				o += 1;
			}
			self.memory.set(&vname, self.eval_exp(atoksn));
			j += 1;
		}
		tokens[i] = self.eval(t);
		self.memory.rem_scope();
		return i;
	}
	fn eval_exp (&self, toks : Vec<Token>) -> Token {
		return toks[0].clone();
	}
	fn eval (&mut self, mut tokens : Vec<Token>) -> Token {
		let mut token_index : usize = 0;
		let mut tokens_length = tokens.len();
		loop {
			if token_index >= tokens_length {
				break;
			}
			let token = tokens[token_index].clone();
			// meta properties
			if token.id == MET {
				if token.value == "terminating_newlines" {
					self.terminating_newlines = tokens[token_index+1].value.parse::<u32>().unwrap();
				} else if token.value == "print_sep_spaces" {
					self.print_sep_spaces = tokens[token_index+1].value.parse::<u32>().unwrap();
				}
			// handle keywords
			} else if token.id == KEY { 
				if token.value == "print" {
					token_index = self.printop(token_index, &tokens);
				} else if token.value == "log" {
					print!("\x1b[38;2;64;175;255m");
					token_index = self.printop(token_index, &tokens);
					print!("\x1b[39m");
				} else if token.value == "return" {
					return tokens[token_index+1].clone();
				} else if token.value == "dumpscope" {
					token_index = self.dumpscope(token_index, &tokens);
				} else if token.value == "global" {
					if tokens[token_index+1].id == REF {
						self.memory.flag_var(tokens[token_index+1].value.clone(), 3u8);
					}
				} else if token.value == "local" {
					if tokens[token_index+1].id == REF {
						self.memory.flag_var(tokens[token_index+1].value.clone(), 0u8);
					}
				} else if token.value == "rm" {
					println!("rm");
					self.memory.rm(&tokens[token_index+1].value);
					token_index += 1;
				} else if token.value == "garbage" {
					println!("garbage");
					self.memory.garbage(&tokens[token_index+1].value);
					token_index += 1;
				}
			// handle variable assignment
			} else if token.id == ASS {
				let varname = &tokens[token_index-1].value;
				let operand = &token.value;
				// seperate simple assignment from modification to a value
				if operand == "=" {
					self.memory.set(varname, tokens[token_index+1].clone());
				} else {
					self.memory.set(varname, self.assignment_operation(&operand, self.memory.get(varname).value, tokens[token_index+1].value.clone()));
				}
			// handle function initialization
			} else if token.id == FUN {
				self.memory.set(&token.value, token.clone());
			// handle function calls
			} else if token_index > 0 && self.deref(tokens[token_index-1].clone()).id == FUN && token.id == PAR {
				token_index = self.func_call(token_index, &mut tokens);
				tokens_length = tokens.len();
			}
			token_index += 1;
		}
		return self.UDFTOK.clone();
	}
	fn run (&mut self) -> u8 {
		self.memory.new_scope();
		self.eval(self.tokens.clone());
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