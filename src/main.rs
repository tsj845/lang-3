#![allow(non_snake_case)]
#[allow(unused_imports)]
#[macro_use] extern crate lang_1 as this;

use std::fs::File;
use std::io::Read;
use this::statics::*;
use this::scopes::*;
use this::tokenize::*;
use this::method_bindings::*;
use this::static_colors::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Parser {
	tokens : Vec<Token>,
	memory : VarScopes,
	SEPTOK : Token,
	UDFTOK : Token,
	BINDINGS : Bindings<'static>,
	terminating_newlines : u32,
	print_sep_spaces : u32,
	token_block : HashMap<String, bool>,
}

impl Parser {
	fn new (tokens : Vec<Token>) -> Parser {
		Parser {
			tokens : tokens,
			memory : VarScopes::new(),
			SEPTOK : Token::news(SEP, ",", BASE_TOKEN),
			UDFTOK : Token::news(UDF, "UDF", BASE_TOKEN),
			BINDINGS : Bindings::new(),
			terminating_newlines : 1,
			print_sep_spaces : 1,
			token_block : HashMap::new(),
		}
	}
	fn __fault (&self) -> Token {
		return Token::new(UDF, String::from("UDF"), BASE_TOKEN);
	}
	fn __filter (&self, tokens : &Vec<Token>) -> Vec<Token> {
		let mut r : Vec<Token> = Vec::new();
		for tok in tokens {
			if !(self.token_block.contains_key(TOKEN_ARRAY[tok.id as usize]) && *self.token_block.get(TOKEN_ARRAY[tok.id as usize]).unwrap()) {
				r.push(tok.clone());
			}
		}
		return r;
	}
	fn addition (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			static ref STRING_RE : Regex = Regex::new(TOKEN_STR_RE_PAT).unwrap();
		}
		if NUMBER_RE.is_match(&v1) {
			if !NUMBER_RE.is_match(&v2) {
				panic!("mismatched types");
			}
			if DECI_RE.is_match(&v1) || DECI_RE.is_match(&v2) {
				let mut v : f64 = v1.parse().unwrap();
				v += v2.parse::<f64>().unwrap();
				return Token::new(LIT, v.to_string(), BASE_TOKEN);
			}
			let mut v : i64 = v1.parse().unwrap();
			v += v2.parse::<i64>().unwrap();
			return Token::new(LIT, v.to_string(), BASE_TOKEN);
		} else if STRING_RE.is_match(&v1) {
			if !STRING_RE.is_match(&v2) {
				println!("{}", v2);
				panic!("mismatched types");
			}
			return Token::new(LIT, String::from("\"") + &String::from(&v1[1..v1.len()-1]) + &v2[1..v2.len()-1] + "\"", BASE_TOKEN);
		}
		return self.__fault();
	}
	fn subtraction (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
		}
		if !(NUMBER_RE.is_match(&v1) && NUMBER_RE.is_match(&v2)) {
			return self.__fault();
		}
		if DECI_RE.is_match(&v1) || DECI_RE.is_match(&v2) {
				let mut v : f64 = v1.parse().unwrap();
				v -= v2.parse::<f64>().unwrap();
				return Token::new(LIT, v.to_string(), BASE_TOKEN);
			}
		let mut v : i64 = v1.parse().unwrap();
		v -= v2.parse::<i64>().unwrap();
		return Token::new(LIT, v.to_string(), BASE_TOKEN);
	}
	fn multiplication (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			static ref STRING_RE : Regex = Regex::new(TOKEN_STR_RE_PAT).unwrap();
		}
		if NUMBER_RE.is_match(&v1) {
			if !NUMBER_RE.is_match(&v2) {
				panic!("mismatched types");
			}
			if DECI_RE.is_match(&v1) || DECI_RE.is_match(&v2) {
				let mut v : f64 = v1.parse().unwrap();
				v *= v2.parse::<f64>().unwrap();
				return Token::new(LIT, v.to_string(), BASE_TOKEN);
			}
			let mut v : i64 = v1.parse().unwrap();
			v *= v2.parse::<i64>().unwrap();
			return Token::new(LIT, v.to_string(), BASE_TOKEN);
		} else if STRING_RE.is_match(&v1) {
			if !NUMBER_RE.is_match(&v2) {
				panic!("mismatched types");
			}
			return Token::new(LIT, String::from("\"") + &String::from(&v1[1..v1.len()-1]).repeat(v2.parse::<usize>().unwrap()) + "\"", BASE_TOKEN);
		}
		return self.__fault();
	}
	fn division (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
		}
		if !(NUMBER_RE.is_match(&v1) && NUMBER_RE.is_match(&v2)) {
			return self.__fault();
		}
		if DECI_RE.is_match(&v1) || DECI_RE.is_match(&v2) {
				let mut v : f64 = v1.parse().unwrap();
				v /= v2.parse::<f64>().unwrap();
				return Token::new(LIT, v.to_string(), BASE_TOKEN);
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
		if t.value.starts_with('$') {
			t.value = String::from(&t.value[1..]);
		}
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
	fn derefb (&self, t : &Token) -> Token {
		if t.id != REF {
			return t.clone();
		}
		let mut names : HashMap<String, u8> = HashMap::new();
		let mut n : String = t.value.clone();
		if n.chars().nth(0).unwrap() == '$' {
			n = String::from(&n[1..]);
		}
		names.insert(n.clone(), 0);
		let mut r : Token = self.memory.get(&n);
		loop {
			if r.id != REF {
				return r;
			}
			if names.insert(r.value.clone(), 0).is_some() {
				return self.__fault();
			}
			r = self.memory.get(&r.value);
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
	fn printop (&mut self, mut i : usize, tokens : &Vec<Token>) -> usize {
		i += 1;
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
				let mut toks : Vec<Token> = Vec::new();
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
					toks.push(token.clone());
					i += 1;
				}
				// print!("\x1b[38;2;0;255;255m");
				// printlst::<Token>(&toks);
				// print!("\x1b[39m");
				copt = self.eval_exp(toks);
				// println!("printing COPT");
				// println!("{}", copt);
				if copt.id == REF {
					copt = self.deref(copt);
				}
				// println!("{}", copt);
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
		let t : Token = self.derefb(&tokens[i+1]);
		return match t.tt() {LIST_TOKEN => t.get(tokens[i-1].value.parse::<usize>().unwrap()), _ => t.getd(tokens[i-1].value.clone())};
	}
	fn func_call (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let t : Vec<Token> = self.derefb(&tokens[i-1]).list.as_ref().unwrap().clone();
		let mut depth = 0;
		let mut atoks : Vec<Token> = Vec::new();
		loop {
			if i >= tokens.len() {
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
					depth += 1;
					if depth > 1 {
						atoks.push(tokens.remove(i));
					} else {
						tokens.remove(i);
					}
				}
			} else {
				atoks.push(tokens.remove(i));
			}
		}
		tokens.remove(i-1);
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
			let v = self.eval_exp(atoksn);
			self.memory.set(&vname, v);
			j += 1;
		}
		tokens[i-1] = self.eval(t);
		self.memory.rem_scope();
		return i;
	}
	fn eval_exp (&mut self, mut toks : Vec<Token>) -> Token {
		if toks.len() == 0 {
			return self.__fault();
		}
		let mut i : usize = 0;
		loop {
			if i >= toks.len() {
				break;
			}
			if toks[i].id == REF {
				if toks[i].value.chars().nth(0).unwrap() == '$' {
					toks[i] = self.deref(toks[i].clone());
				}
			}
			i += 1;
		}
		let mut copt : Token = toks[0].clone();
		i = 1;
		let mut l = toks.len();
		loop {
			if i >= l {
				break;
			}
			// println!("\x1b[38;2;255;0;255m{}\x1b[39m", toks[i]);
			if toks[i].id == LIT || toks[i].id == REF {
				copt = self.derefb(&toks[i]).clone();
			} else if toks[i].id == MAT {
				if toks[i+2].id == DOT {
					// println!("BINDING");
					i += 2;
					let oi : usize = i-1;
					let x : (usize, Token, Vec<Token>) = self.execute(toks.clone(), i);
					toks = x.2;
					i = x.0;
					toks[i-1] = x.1;
					// println!("abrem: {}, {}", toks[i], toks[i+1]);
					toks.remove(i);
					toks.remove(i);
					i -= 1;
					loop {
						if oi >= i {
							break;
						}
						// println!("REM: {}", toks[oi]);
						toks.remove(oi);
						i -= 1;
					}
					l = toks.len();
					i -= 1;
				}
				copt = self.operation(&toks[i].value, self.derefb(&copt).value, self.derefb(&toks[i+1]).value);
				i += 1;
			} else if toks[i].id == LOG {
				// do logical operations
			} else if toks[i].id == DOT {
				if self.BINDINGS.check_valid(&self.derefb(&toks[i-1]), &toks[i+1].value) {
					let oi : usize = i-1;
					let x : (usize, Token, Vec<Token>) = self.execute(toks.clone(), i);
					toks = x.2;
					i = x.0;
					toks[i-1] = x.1;
					copt = toks[i-1].clone();
					toks.remove(i);
					toks.remove(i);
					i -= 1;
					loop {
						if oi >= i {
							break;
						}
						toks.remove(oi);
						i -= 1;
					}
					l = toks.len();
				}
			}
			i += 1;
		}
		return copt;
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
				} else if token.value.starts_with("token_block__") {
					let v : &str = &token.value[13..];
					self.token_block.insert(v.to_string(), tokens[token_index+1].value == "true");
				}
			// handle keywords
			} else if token.id == KEY { 
				if token.value == "print" {
					token_index = self.printop(token_index, &tokens);
				} else if token.value == "log" {
					print!("{}", DEBUG_BLUE_PROGRAM_LOG);
					token_index = self.printop(token_index, &tokens);
					print!("\x1b[39m");
				} else if token.value == "return" {
					let mut r : Token = tokens[token_index+1].clone();
					if self.derefb(&r).id == FUN && tokens[token_index+2].id == PAR && tokens[token_index+2].value == "(" {
						self.func_call(token_index+2, &mut tokens);
						r = tokens[token_index+1].clone();
					}
					// if r.id == REF && r.value.starts_with('$') {
					// 	r = self.deref(r);
					// }
					// println!("{}", self.derefb(&r));
					return self.derefb(&r);
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
					self.memory.rm(&tokens[token_index+1].value);
					token_index += 1;
				} else if token.value == "garbage" {
					if tokens[token_index+1].id == NLN {
						self.memory.clear();
					} else {
						self.memory.garbage(&tokens[token_index+1].value);
						token_index += 1;
					}
				} else if token.value == "dumptoks" {
					printlst::<Token>(&self.__filter(&tokens));
				} else if token.value == "dumplc" {
					println!("DUMPLC");
					printlst::<Token>(&self.derefb(&tokens[token_index+1]).list.as_ref().unwrap());
					token_index += 1;
				} else if token.value == "dumpflags" {
					self.memory.dump_flags();
				}
			// handle variable assignment
			} else if token.id == ASS {
				let varname = &tokens[token_index-1].clone().value;
				let operand = &token.value;
				// seperate simple assignment from modification to a value
				if operand == "=" {
					let mut r : Token = tokens[token_index+1].clone();
					if self.derefb(&r).id == FUN && tokens[token_index+2].id == PAR && tokens[token_index+2].value == "(" {
						self.func_call(token_index+2, &mut tokens);
						tokens_length = tokens.len();
						r = tokens[token_index+1].clone();
						token_index += 1;
					}
					let mut lst : Vec<Token> = Vec::new();
					lst.push(r.clone());
					let mut ind : usize = token_index+2;
					loop {
						if ind >= tokens_length {
							break;
						}
						if tokens[ind].id == NLN {
							break;
						}
						lst.push(tokens[ind].clone());
						ind += 1;
					}
					r = self.eval_exp(lst);
					self.memory.set(varname, r);
				} else {
					let v2 : String = self.derefb(&tokens[token_index+1]).value;
					// println!("{}v1:{},v2:{},op:{}\x1b[39m", INTERPRETER_DEBUG_ORANGE, self.memory.get(varname).value, v2, operand);
					self.memory.set(varname, self.assignment_operation(&operand, self.memory.get(varname).value, v2));
				}
			// handle function initialization
			} else if token.id == FUN {
				self.memory.set(&token.value, token.clone());
			// handle function calls
			} else if token_index > 0 && token.id == PAR && token.value == "(" && self.deref(tokens[token_index-1].clone()).id == FUN {
				token_index = self.func_call(token_index, &mut tokens);
				tokens_length = tokens.len();
			// handle object properties and methods
			} else if token.id == DOT && token_index < tokens_length-1 && tokens[token_index+1].id == REF {
				// println!("{}unchecked binding\x1b[39m", INTERPRETER_DEBUG_BRIGHTPINK);
				if self.BINDINGS.check_valid(&self.derefb(&tokens[token_index-1]), &tokens[token_index+1].value) {
					// println!("{}binding\x1b[39m", INTERPRETER_DEBUG_BRIGHTPINK);
					let oi : usize = token_index-1;
					let x : (usize, Token, Vec<Token>) = self.execute(tokens.clone(), token_index);
					tokens = x.2;
					token_index = x.0;
					tokens[token_index-1] = x.1;
					// println!("abrem: {}, {}", tokens[token_index], tokens[token_index+1]);
					tokens.remove(token_index);
					tokens.remove(token_index);
					token_index -= 1;
					loop {
						if oi >= token_index {
							break;
						}
						tokens.remove(oi);
						token_index -= 1;
					}
					tokens_length = tokens.len();
				}
			}
			token_index += 1;
		}
		return self.UDFTOK.clone();
	}
	fn get_value (&mut self, tokens : &Vec<Token>, mut i : usize) -> (usize, Token) {
		let mut toks : Vec<Token> = Vec::new();
		let l = tokens.len();
		let mut depth : u16 = 0;
		loop {
			if i >= l {
				return (i-1, self.__fault());
			}
			if tokens[i].id == PAR {
				if tokens[i].value == "(" {
					depth += 1;
				} else if tokens[i].value == ")" {
					depth -= 1;
					if depth == 0 {
						break;
					}
				}
				if depth > 1 {
					toks.push(tokens[i].clone());
				}
			} else {
				if depth > 0 {
					toks.push(tokens[i].clone());
				}
			}
			i += 1;
		}
		return (i+1, self.eval_exp(toks));
	}
	fn execute (&mut self, mut tokens : Vec<Token>, mut i : usize) -> (usize, Token, Vec<Token>) {
		lazy_static! {
			static ref ALPHA_RE : Regex = Regex::new(ALPHA_RE_PAT).unwrap();
			static ref DIGIT_RE : Regex = Regex::new(DIGIT_RE_PAT).unwrap();
		}
		// println!("{}EXECUTION\x1b[39m", INTERPRETER_DEBUG_BRIGHTPINK);
        let target : &str = &tokens[i+1].value.clone();
		let is_ref : bool = tokens[i-1].id == REF;
		let ov : &str = &tokens[i-1].value.clone();
        let mut t : Token = match is_ref {false=>tokens[i-1].clone(),true=>self.memory.get(&tokens[i-1].value)};
		// println!("{}IS_REF={}, {}\x1b[39m", INTERPRETER_DEBUG_ORANGE, is_ref, t.value);
        if t.data_type == DT_STR {
            let btype : &&str = self.BINDINGS.get_type(0, target);
            if btype == &"method" {
				let mut ret : Token = self.__fault();
				if target == "is_alpha" {
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = Token::new(LIT, ALPHA_RE.is_match(&t.value[1..t.value.len()-1]).to_string(), BASE_TOKEN);
				} else if target == "is_digit" {
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = Token::new(LIT, DIGIT_RE.is_match(&t.value[1..t.value.len()-1]).to_string(), BASE_TOKEN);
				}
				if is_ref {
					self.memory.set(ov, t);
				} else {
					tokens[i-1] = t;
				}
                return (i, ret, tokens);
            } else if btype == &"property" {
                if target == "length" {
                    return (i, Token::new(LIT, (t.value.len()-2).to_string(), BASE_TOKEN), tokens);
                }
            }
        }
        if t.data_type == DT_LST {
            let btype : &&str = self.BINDINGS.get_type(1, target);
            if btype == &"method" {
				let mut ret : Token = self.__fault();
                if target == "push" {
					println!("{}PUSH EXECUTION\x1b[39m", INTERPRETER_DEBUG_ORANGE);
                    let x : (usize, Token) = self.get_value(&tokens, i);
					// printlst::<Token>(&tokens);
					println!("{}, {}", i, t);
                    i = x.0 - 2;
                    t.push(x.1);
                } else if target == "pop" {
					println!("{}POP EXECUTION\x1b[39m", INTERPRETER_DEBUG_ORANGE);
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = t.pop();
				} else if target == "remove" {
					println!("{}REMOVE EXECUTION\x1b[39m", INTERPRETER_DEBUG_ORANGE);
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = t.popitem(x.1.value.parse::<usize>().unwrap());
				} else if target == "get" {
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = t.list.as_ref().unwrap()[x.1.value.parse::<usize>().unwrap()].clone();
				}
				if is_ref {
					self.memory.set(ov, t);
				} else {
					tokens[i-1] = t;
				}
                return (i, ret, tokens);
            } else if btype == &"property" {
                if target == "length" {
                    return (i, Token::new(LIT, t.length.to_string(), BASE_TOKEN), tokens);
                }
            }
        }
		if t.data_type == DT_NUM {
			let btype : &&str = self.BINDINGS.get_type(3, target);
			if btype == &"property" {
				return (i, self.__fault(), tokens);
			} else if btype == &"method" {
				if target == "to_string" {
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					return (i, Token::new(LIT, String::from(r#"""#)+&t.value+r#"""#, BASE_TOKEN), tokens);
				}
			}
		}
		if t.data_type == DT_DCT {
			// todo
		}
		if t.data_type == DT_OBJ {
			let r : Token = t.getd(target.to_string()).clone();
			if r.id == FUN {
				// todo
			}
			return (i, r, tokens);
		}
        return (i, self.__fault(), tokens);
    }
	fn __init (&mut self) -> () {
		let mut systok : Token = Token::news(OBJ, "SYS", DICT_TOKEN);
		systok.setd(String::from("lime"), Token::news(LIT, r#""\x1b[38;2;0;255;0m""#, BASE_TOKEN));
		systok.setd(String::from("red"), Token::news(LIT, r#""\x1b[38;2;255;0;0m""#, BASE_TOKEN));
		systok.setd(String::from("blue"), Token::news(LIT, r#""\x1b[38;2;0;0;255m""#, BASE_TOKEN));
		systok.setd(String::from("cyan"), Token::news(LIT, r#""\x1b[38;2;0;255;255m""#, BASE_TOKEN));
		systok.setd(String::from("yellow"), Token::news(LIT, r#""\x1b[38;2;255;255;0m""#, BASE_TOKEN));
		systok.setd(String::from("violet"), Token::news(LIT, r#""\x1b[38;2;255;0;255m""#, BASE_TOKEN));
		systok.setd(String::from("default"), Token::news(LIT, r#""\x1b[39m""#, BASE_TOKEN));
		self.memory.set("System", systok);
		self.memory.set_protection("System", 1u8);
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
	program.__init();
	println!("\n{}program output:\x1b[39m\n", INTERPRETER_LIME);
	program.run();
	println!("\n\n");
}