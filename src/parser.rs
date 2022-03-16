use std::fs::File;
use std::io::Read;
use std::io;
use crate::statics::*;
use crate::scopes::*;
use crate::tokenize::*;
use crate::method_bindings::*;
use crate::static_colors::*;
use crate::logger::*;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Parser {
	tokens : Vec<Token>,
	pub memory : VarScopes,
	pub modules : HashMap<String, Parser>,
	LOG : Logger,
	SEPTOK : Token,
	UDFTOK : Token,
	BINDINGS : Bindings<'static>,
	meta_terminating_newlines : u32,
	meta_print_sep_spaces : u32,
	meta_print_sep_value : String,
	token_block : HashMap<String, bool>,
	dbdepth : i64,
}

impl Parser {
	pub fn new (tokens : Vec<Token>) -> Parser {
		Parser {
			tokens : tokens,
			memory : VarScopes::new(),
			modules : HashMap::new(),
			LOG : Logger::new(),
			SEPTOK : Token::news(SEP, ",", BASE_TOKEN),
			UDFTOK : Token::news(UDF, "UDF", BASE_TOKEN),
			BINDINGS : Bindings::new(),
			meta_terminating_newlines : 1,
			meta_print_sep_spaces : 1,
			meta_print_sep_value : String::from(" "),
			token_block : HashMap::new(),
			dbdepth : 0,
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
	fn bin_and (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			// static ref STRING_RE : Regex = Regex::new(STRING_RE_PAT).unwrap();
		}
		if !NUMBER_RE.is_match(&v1) || v1.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v1) {
			// 	return self.__fault();
			// }
		}
		if !NUMBER_RE.is_match(&v2) || v2.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v2) {
			// 	return self.__fault();
			// }
		}
		return Token::new(LIT, (v1.parse::<i64>().unwrap() & v2.parse::<i64>().unwrap()).to_string(), BASE_TOKEN);
	}
	fn bin_or (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			// static ref STRING_RE : Regex = Regex::new(STRING_RE_PAT).unwrap();
		}
		if !NUMBER_RE.is_match(&v1) || v1.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v1) {
			// 	return self.__fault();
			// }
		}
		if !NUMBER_RE.is_match(&v2) || v2.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v2) {
			// 	return self.__fault();
			// }
		}
		return Token::new(LIT, (v1.parse::<i64>().unwrap() | v2.parse::<i64>().unwrap()).to_string(), BASE_TOKEN);
	}
	fn bin_xor (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			// static ref STRING_RE : Regex = Regex::new(STRING_RE_PAT).unwrap();
		}
		if !NUMBER_RE.is_match(&v1) || v1.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v1) {
			// 	return self.__fault();
			// }
		}
		if !NUMBER_RE.is_match(&v2) || v2.find('.').is_some() {
			return self.__fault();
			// if STRING_RE.is_match(&v2) {
			// 	return self.__fault();
			// }
		}
		return Token::new(LIT, (v1.parse::<i64>().unwrap() ^ v2.parse::<i64>().unwrap()).to_string(), BASE_TOKEN);
	}
	fn modulo_op (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
		}
		if !NUMBER_RE.is_match(&v1) || v1.find('.').is_some() || !NUMBER_RE.is_match(&v2) || v2.find('.').is_some() {
			return self.__fault();
		}
		return Token::new(LIT, (v1.parse::<i64>().unwrap() % v2.parse::<i64>().unwrap()).to_string(), BASE_TOKEN);
	}
	fn log_and (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
		}
		let b1 = Token::new(LIT, v1, BASE_TOKEN).bool();
		let b2 = Token::new(LIT, v2, BASE_TOKEN).bool();
		return Token::news(LIT, match b1 && b2 {true=>"true",_=>"false"}, BASE_TOKEN);
	}
	fn log_or (&self, v1 : String, v2 : String) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
		}
		let b1 = Token::new(LIT, v1, BASE_TOKEN).bool();
		let b2 = Token::new(LIT, v2, BASE_TOKEN).bool();
		return Token::news(LIT, match b1 || b2 {true=>"true",_=>"false"}, BASE_TOKEN);
	}
	fn comp_op (&self, v1 : String, v2 : String, op : &str) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if !(NUMBER_RE.is_match(&v1) && NUMBER_RE.is_match(&v2)) {
			return self.__fault();
		}
		let n1 = v1.parse::<f64>().unwrap();
		let n2 = v2.parse::<f64>().unwrap();
		if op == ">" {
			return Token::new(LIT, (n1 > n2).to_string(), BASE_TOKEN);
		}
		if op == "<" {
			return Token::new(LIT, (n1 < n2).to_string(), BASE_TOKEN);
		}
		if op == ">=" {
			return Token::new(LIT, (n1 >= n2).to_string(), BASE_TOKEN);
		}
		if op == "<=" {
			return Token::new(LIT, (n1 <= n2).to_string(), BASE_TOKEN);
		}
		return self.__fault();
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
		} else if operand == "&" {
			return self.bin_and(v1, v2);
		} else if operand == "|" {
			return self.bin_or(v1, v2);
		} else if operand == "%" {
			return self.modulo_op(v1, v2);
		} else if operand == "^" {
			return self.bin_xor(v1, v2);
		} else if operand == "&&" {
			return self.log_and(v1, v2);
		} else if operand == "||" {
			return self.log_or(v1, v2);
		} else if operand == ">" || operand == "<" || operand == ">=" || operand == "<=" {
			return self.comp_op(v1, v2, operand);
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
				print!("{}", self.meta_print_sep_value.repeat(self.meta_print_sep_spaces as usize));
				continue;
			}
			if new_arg_ready {
				new_arg_ready = false;
				let mut copt : Token;
				let mut toks : Vec<Token> = Vec::new();
				let mut depth : usize = 0;
				loop {
					if i >= l {
						break;
					}
					let token = tokens[i].clone();
					if token.id == NLN {
						i -= 1;
						break;
					}
					if token.id == PAR {
						if token.value == "(" {
							depth += 1;
						} else if token.value == ")" {
							depth -= 1;
						}
					}
					if token == self.SEPTOK && depth == 0 {
						i -= 1;
						break;
					}
					toks.push(token.clone());
					i += 1;
				}
				copt = self.eval_exp(toks);
				if copt.id == REF {
					copt = self.deref(copt);
				}
				let b : Vec<char> = copt.value.chars().collect();
				let mut v = copt.value.clone();
				if b[0] == '"' && b[b.len()-1] == '"' {
					v = copt.value[1..copt.value.len()-1].to_string();
				}
				self.LOG.log(String::from("print component: ") + &v);
				print!("{}", v);
			}
			i += 1;
		}
		print!("{}", String::from("\n").repeat(self.meta_terminating_newlines as usize));
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
		let res = self.eval(t);
		if tokens.len() > i-1 {
			tokens[i-1] = res;
		} else {
			tokens.push(res);
		}
		self.memory.rem_scope();
		return i;
	}
	fn eval_exp (&mut self, mut toks : Vec<Token>) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref DECI_RE : Regex = Regex::new(DECI_RE_PAT).unwrap();
			static ref STRING_RE : Regex = Regex::new(TOKEN_STR_RE_PAT).unwrap();
		}
		if toks.len() == 0 {
			return self.__fault();
		}
		self.dbdepth += 1;
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
			if toks[i].id == LIT || toks[i].id == REF {
				copt = self.derefb(&toks[i]);
			} else if toks[i].id == MAT || toks[i].id == LOG {
				i += 1;
				let mut optoks : Vec<Token> = Vec::new();
				loop {
					if i >= toks.len() {
						break;
					}
					if toks[i].id == NLN || toks[i].id == MAT || toks[i].id == LOG {
						break;
					}
					optoks.push(toks.remove(i));
				}
				let r = self.eval_exp(optoks);
				toks.insert(i, r);
				i -= 1;
				copt = self.operation(&toks[i].value, self.derefb(&copt).value, toks[i+1].value.clone());
				i += 1;
				l = toks.len();
			} else if toks[i].id == DOT {
				if self.BINDINGS.check_valid(&self.modules, &self.derefb(&toks[i-1]), &toks[i+1].value) {
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
			} else if toks[i].id == IDX {
				let tlst = toks[i].list.as_ref().unwrap().clone();
				let t = self.eval_exp(tlst);
				toks.remove(i);
				toks[i-1] = self.derefb(&toks[i-1]);
				if toks[i-1].id == LST {
					toks[i-1] = toks[i-1].get(t.value.parse::<usize>().unwrap());
				} else if toks[i-1].id == DCT {
					toks[i-1] = toks[i-1].getd(t.value);
				}
				copt = toks[i-1].clone();
			} else if toks[i].id == PAR && toks[i].value == "(" && i > 0 && (toks[i-1].id == REF || toks[i-1].id == FUN) {
				i = self.func_call(i, &mut toks);
				i -= 1;
				copt = toks[i].clone();
				l = toks.len();
			} else if toks[i].id == BND {
				i = self.binding_execution(i, &mut toks);
				copt = toks[i].clone();
				l = toks.len();
			}
			i += 1;
		}
		self.dbdepth -= 1;
		return copt;
	}
	fn eval_for_loop (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let t : Vec<Token> = self.derefb(&tokens[i]).list.as_ref().unwrap().clone();
		let mut ind : usize = 0;
		let mut start : i64;
		let mut sc : usize = 0;
		let mut vname : String = String::new();
		let mut ls : Vec<Token> = Vec::new();
		loop {
			if ind >= t.len() {
				break;
			}
			if t[ind].id == SEP && t[ind].value == "*" {
				if sc == 0 {
					sc = 1;
					vname = t[ind-1].value.clone();
				} else {
					break;
				}
			} else if sc == 1 {
				ls.push(t[ind].clone());
			}
			ind += 1;
		}
		let mut j : usize = 0;
		let l = ls.len();
		let mut flag : bool = false;
		let mut ls1 : Vec<Token> = Vec::new();
		let mut ls2 : Vec<Token> = Vec::new();
		loop {
			if j >= l {
				break;
			}
			if ls[j].id == SEP && ls[j].value == ".." {
				flag = true;
			} else if flag {
				ls2.push(ls[j].clone());
			} else {
				ls1.push(ls[j].clone());
			}
			j += 1;
		}
		let v1 = self.eval_exp(ls1).value;
		let v2 = self.eval_exp(ls2).value;
		start = v1.parse::<i64>().unwrap_or(0);
		let end = v2.parse::<i64>().unwrap_or(0);
		tokens.remove(i-1);
		self.memory.new_scope();
		self.memory.flag_var(vname.clone(), 0u8);
		loop {
			if start >= end {
				break;
			}
			self.memory.set(&vname, Token::new(LIT, start.to_string(), BASE_TOKEN));
			let r = self.eval(t.clone());
			if r.id == SIG {
				if r.value == "BREAK" {
					break;
				}
			}
			start += 1;
		}
		self.memory.rem_scope();
		return i;
	}
	fn eval_while_loop (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let t : Vec<Token> = self.derefb(&tokens[i]).list.as_ref().unwrap().clone();
		let mut ind : usize = 0;
		let mut ls : Vec<Token> = Vec::new();
		loop {
			if ind >= t.len() {
				break;
			}
			if t[ind].id == SEP && t[ind].value == "*" {
				break;
			}
			ls.push(t[ind].clone());
			ind += 1;
		}
		tokens.remove(i-1);
		self.memory.new_scope();
		loop {
			if !self.eval_exp(ls.clone()).bool() {
				break;
			}
			let r = self.eval(t.clone());
			if r.id == SIG {
				if r.value == "BREAK" {
					break;
				}
			}
		}
		self.memory.rem_scope();
		return i;
	}
	fn eval_if_block (&mut self, i : usize, tokens : &mut Vec<Token>) -> (usize, bool) {
		let t : Vec<Token> = self.derefb(&tokens[i]).list.as_ref().unwrap().clone();
		let mut ls : Vec<Token> = Vec::new();
		let mut ind : usize = 0;
		loop {
			if ind >= t.len() {
				break;
			}
			if t[ind].id == SEP && t[ind].value == "*" {
				break;
			}
			ls.push(t[ind].clone());
			ind += 1;
		}
		if !self.eval_exp(ls).bool() {
			return (i, false);
		}
		tokens.remove(i-1);
		self.memory.new_scope();
		self.eval(t.clone());
		self.memory.rem_scope();
		return (i, true);
	}
	fn eval_else_block (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let t : Vec<Token> = self.derefb(&tokens[i]).list.as_ref().unwrap().clone();
		tokens.remove(i-1);
		self.memory.new_scope();
		self.eval(t.clone());
		self.memory.rem_scope();
		return i;
	}
	fn eval (&mut self, mut tokens : Vec<Token>) -> Token {
		let mut token_index : usize = 0;
		let mut tokens_length = tokens.len();
		let mut cflag = false;
		loop {
			if token_index >= tokens_length {
				break;
			}
			let token = tokens[token_index].clone();
			// meta properties
			if token.id == MET {
				if token.value == "terminating_newlines" {
					self.meta_terminating_newlines = tokens[token_index+1].value.parse::<u32>().unwrap();
				} else if token.value == "print_sep_spaces" {
					self.meta_print_sep_spaces = tokens[token_index+1].value.parse::<u32>().unwrap();
				} else if token.value == "print_sep_value" {
					let v = &tokens[token_index+1].value;
					self.meta_print_sep_value = String::from(&v[1..v.len()-1]);
				} else if token.value.starts_with("token_block__") {
					let v : &str = &token.value[13..];
					self.token_block.insert(v.to_string(), tokens[token_index+1].value == "true");
				}
			// handle keywords
			} else if token.id == KEY {
				if token.value == "HALT" {
					println!("{}HALTING EXECUTION\x1b[0m", ERROR_RED);
					panic!();
				}
				if token.value == "print" {
					token_index = self.printop(token_index, &tokens);
				} else if token.value == "log" {
					print!("{}", DEBUG_BLUE_PROGRAM_LOG);
					token_index = self.printop(token_index, &tokens);
					print!("\x1b[39m");
				} else if token.value == "return" {
					// let mut r : Token = tokens[token_index+1].clone();
					// if self.derefb(&r).id == FUN && tokens[token_index+2].id == PAR && tokens[token_index+2].value == "(" {
						// self.func_call(token_index+2, &mut tokens);
						// r = tokens[token_index+1].clone();
					// }
					let mut lst : Vec<Token> = Vec::new();
					loop {
						if token_index >= tokens_length {
							break;
						}
						if tokens[token_index].id == NLN {
							break;
						}
						lst.push(tokens[token_index].clone());
						token_index += 1;
					}
					let r = self.eval_exp(lst);
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
					println!("{}{}\x1b[0m", INTERPRETER_DEBUG_BRIGHTPINK, self.derefb(&tokens[token_index+1]));
					printlst::<Token>(&self.derefb(&tokens[token_index+1]).list.as_ref().unwrap());
					token_index += 1;
				} else if token.value == "dumpflags" {
					self.memory.dump_flags();
				} else if token.value == "break" {
					return Token::newsb(SIG, "BREAK");
				} else if token.value == "continue" {
					return Token::newsb(SIG, "CONTINUE");
				} else if token.value == "linkup" {
					self.eval(read_in_file(&tokens[token_index+1].value));
					token_index += 1;
				} else if token.value == "module" {
					let vname = &tokens[token_index+1].value;
					let modu = Parser::new(read_in_file(vname));
					self.modules.insert(tokens[token_index+1].value.clone(), modu);
					self.modules.get_mut(vname).unwrap().run();
					self.memory.flag_var(tokens[token_index+1].value.clone(), 3u8);
					self.memory.set(vname, Token::newsb(MOD, &tokens[token_index+1].value));
					self.memory.set_protection(vname, 1u8);
					token_index += 1;
				} else if token.value == "private" {
					self.memory.inter_flag_set(&tokens[token_index+1].value, 1u8);
					token_index += 1;
				}
			// handle variable assignment
			} else if token.id == ASS {
				let varname = &tokens[token_index-1].clone().value;
				let operand = &token.value;
				let subscript = tokens[token_index-1].id == IDX;
				let imt = match token_index > 1 {true=>token_index-2,_=>0};
				// seperate simple assignment from modification to a value
				if operand == "=" {
					let mut r : Token = tokens[token_index+1].clone();
					if self.derefb(&r).id == FUN && tokens[token_index+2].id == PAR && tokens[token_index+2].value == "(" {
						self.func_call(token_index+2, &mut tokens);
						tokens_length = tokens.len();
						r = tokens[token_index+1].clone();
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
					if subscript {
						let name = tokens[imt].value.clone();
						let is_ref = tokens[imt].id == REF;
						tokens[imt] = self.derefb(&tokens[imt]);
						if tokens[imt].id == LST {
							let index = self.eval_exp(tokens[imt+1].list.as_ref().unwrap().clone()).value.parse::<usize>().unwrap();
							tokens[imt].set(index, r);
						} else {
							let index = self.eval_exp(tokens[imt+1].list.as_ref().unwrap().clone()).value;
							tokens[imt].setd(index, r);
						}
						if is_ref {
							self.memory.set(&name, tokens[imt].clone());
						}
					} else {
						self.memory.set(varname, r);
					}
				} else {
					let mut v2 : String = self.derefb(&tokens[token_index+1]).value;
					if self.derefb(&tokens[token_index+1]).id == FUN {
						self.func_call(token_index+2, &mut tokens);
						v2 = self.derefb(&tokens[token_index+1]).value;
					}
					if subscript {
						let name = tokens[imt].value.clone();
						let is_ref = tokens[imt].id == REF;
						tokens[imt] = self.derefb(&tokens[imt]);
						if tokens[imt].id == LST {
							let index = self.eval_exp(tokens[imt+1].list.as_ref().unwrap().clone()).value.parse::<usize>().unwrap();
							let va = tokens[imt].get(index).value;
							tokens[imt].set(index, self.assignment_operation(&operand, va, v2));
						} else {
							let index = self.eval_exp(tokens[imt+1].list.as_ref().unwrap().clone()).value;
							let va = tokens[imt].getd(index.clone()).value;
							tokens[imt].setd(index, self.assignment_operation(&operand, va, v2));
						}
						if is_ref {
							self.memory.set(&name, tokens[imt].clone());
						}
					} else {
						self.memory.set(varname, self.assignment_operation(&operand, self.memory.get(varname).value, v2));
					}
				}
				tokens_length = tokens.len();
			// handle function initialization
			} else if token.id == FUN {
				self.memory.set(&token.value, token.clone());
			// handle function calls
			} else if token_index > 0 && token.id == PAR && token.value == "(" && self.deref(tokens[token_index-1].clone()).id == FUN {
				token_index = self.func_call(token_index, &mut tokens);
				tokens_length = tokens.len();
			// handle object properties and methods
			} else if token.id == DOT && token_index < tokens_length-1 && tokens[token_index+1].id == REF {
				if self.BINDINGS.check_valid(&self.modules, &self.derefb(&tokens[token_index-1]), &tokens[token_index+1].value) {
					let oi : usize = token_index-1;
					let x : (usize, Token, Vec<Token>) = self.execute(tokens.clone(), token_index);
					tokens = x.2;
					token_index = x.0;
					tokens[token_index-1] = x.1;
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
			} else if token.id == CTL {
				if token.value == "forloop" {
					token_index = self.eval_for_loop(token_index, &mut tokens);
					tokens_length = tokens.len();
				} else if token.value == "ifblock" {
					let x : (usize, bool) = self.eval_if_block(token_index, &mut tokens);
					token_index = x.0;
					cflag = x.1;
					tokens_length = tokens.len();
				} else if token.value == "elseblock" {
					if !cflag {
						token_index = self.eval_else_block(token_index, &mut tokens);
						tokens_length = tokens.len();
					}
				} else if token.value == "whileloop" {
					token_index = self.eval_while_loop(token_index, &mut tokens);
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
	fn exec_builtin (&mut self, name : String, args : Vec<Token>) -> Token {
		lazy_static! {
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		}
		if name == "input" {
			println!("{}", &args[0].value[1..args[0].value.len()-1]);
			let mut input = String::new();
			io::stdin().read_line(&mut input).expect("ERROR READING INPUT");
			println!("");
			return Token::new(LIT, String::from("\"")+&input[0..input.len()-1]+"\"", BASE_TOKEN);
		} else if name == "sum" {
			let mut sum : f64 = 0f64;
			for t in args[0].list.as_ref().unwrap() {
				let v = match t.value.parse::<f64>() {Ok(x)=>Some(x),_=>None};
				if v.is_none() {
					return self.__fault();
				}
				sum += v.unwrap();
			}
			return Token::new(LIT, sum.to_string(), BASE_TOKEN);
		} else if name == "Number" {
			if args[0].data_type == DT_STR {
				let init = &args[0].value[1..args[0].value.len()-1];
				if !NUMBER_RE.is_match(init) {
					return self.__fault();
				}
				if init.find('.').is_some() {
					return Token::new(LIT, init.parse::<f64>().unwrap().to_string(), BASE_TOKEN);
				}
				return Token::new(LIT, init.parse::<i64>().unwrap().to_string(), BASE_TOKEN);
			}
			if args[0].data_type == DT_NUM {
				return args[0].clone();
			}
		} else if name == "String" {
			if args[0].data_type == DT_STR {
				return args[0].clone();
			}
			return Token::new(LIT, String::from("\"")+&args[0].value+"\"", BASE_TOKEN);
		}
		return self.__fault();
	}
	fn binding_execution (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		let name = tokens[i].value.clone();
		let mut depth = 0;
		let mut atoks : Vec<Token> = Vec::new();
		loop {
			if i >= tokens.len() {
				break;
			}
			if tokens[i+1].id == PAR {
				if tokens[i+1].value == ")" {
					depth -= 1;
					if depth == 0 {
						break;
					}
					atoks.push(tokens.remove(i+1));
				} else if tokens[i+1].value == "(" {
					depth += 1;
					if depth > 1 {
						atoks.push(tokens.remove(i+1));
					} else {
						tokens.remove(i+1);
					}
				}
			} else {
				atoks.push(tokens.remove(i+1));
			}
		}
		tokens.remove(i);
		let mut args : Vec<Token> = Vec::new();
		let k = atoks.len();
		let mut o : usize = 0;
		let mut j : usize = 0;
		loop {
			if j >= k {
				break;
			}
			let mut atoksn : Vec<Token> = Vec::new();
			loop {
				if o >= k {
					break;
				}
				if atoks[o].id == SEP {
					o += 1;
					break;
				}
				atoksn.push(atoks[o].clone());
				o += 1;
			}
			args.push(self.eval_exp(atoksn));
			j += 1;
		}
		tokens[i] = self.exec_builtin(name, args);
		return i;
	}
	fn execute (&mut self, mut tokens : Vec<Token>, mut i : usize) -> (usize, Token, Vec<Token>) {
		lazy_static! {
			static ref ALPHA_RE : Regex = Regex::new(ALPHA_RE_PAT).unwrap();
			static ref DIGIT_RE : Regex = Regex::new(DIGIT_RE_PAT).unwrap();
		}
        let target : &str = &tokens[i+1].value.clone();
		let is_ref : bool = tokens[i-1].id == REF;
		let ov : &str = &tokens[i-1].value.clone();
        let mut t : Token = match is_ref {false=>tokens[i-1].clone(),true=>self.memory.get(&tokens[i-1].value)};
        if t.data_type == DT_STR {
            let btype : &&str = self.BINDINGS.get_type(DT_STR, target);
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
            let btype : &&str = self.BINDINGS.get_type(DT_LST, target);
            if btype == &"method" {
				let mut ret : Token = self.__fault();
                if target == "push" {
                    let x : (usize, Token) = self.get_value(&tokens, i);
                    i = x.0 - 2;
                    t.push(x.1);
                } else if target == "pop" {
					let x : (usize, Token) = self.get_value(&tokens, i);
					i = x.0 - 2;
					ret = t.pop();
				} else if target == "remove" {
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
			let btype : &&str = self.BINDINGS.get_type(DT_NUM, target);
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
		} else if t.data_type == DT_MOD {
			let m = &self.modules.get(&t.value).unwrap().memory;
			let r : Token = match m.inter_flag_get(target) != 1u8 {true=>m.get(target).clone(),_=>self.__fault()};
			return (i, r, tokens);
		}
        return (i, self.__fault(), tokens);
    }
	fn __builtin (&mut self, doc : &str, map : &HashMap<&str, u8>) -> () {
		if doc.len() < 1 {
			return;
		}
		let mut lines : Vec<&str> = doc.split("\n").collect();
		if lines.len() < 1 {
			return;
		}
		let mut i : usize = lines.len() - 1;
		loop {
			if lines[i].len() == 0 {
				lines.remove(i);
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		if lines.len() < 2 {
			return;
		}
		let name : &str = lines.remove(0);
		let mut bi_tok : Token = Token::news(FUN, name, LIST_TOKEN);
		let tok_strs : Vec<&str> = lines[0].split(" ").collect();
		let mut i : usize = 0;
		let l = tok_strs.len();
		loop {
			if i >= l {
				break;
			}
			bi_tok.push(Token::newsb(map.get(&tok_strs[i]).unwrap().clone(), tok_strs[i+1]));
			i += 2;
		}
		self.memory.set(name, bi_tok);
		self.memory.set_protection(name, 1u8);
	}
	fn __prelude (&mut self) -> () {
		let mut contents = String::new();
		let mut file = match File::open(FILE_EXT[1..].to_owned()+".prelude") {Ok(f)=>f,_=>{return}};
		match file.read_to_string(&mut contents) {Ok(x)=>x,_=>{return}};
		let contents : Vec<&str> = contents.split("'''").collect();
		let mut map : HashMap<&str, u8> = HashMap::new();
		let mut i : usize = 0;
		let l = TOKEN_ARRAY.len();
		loop {
			if i >= l {
				break;
			}
			map.insert(TOKEN_ARRAY[i], i as u8);
			i += 1;
		}
		for doc in contents {
			self.__builtin(doc, &map);
		}
	}
	pub fn __init (&mut self) -> () {
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
		self.__prelude();
		// let mut bi_input = Token::news(FUN, "input", LIST_TOKEN);
		// bi_input.extend(vec![Token::newsb(REF, "prompt"), Token::newsb(SEP, "*"), Token::newsb(KEY, "return"), Token::newsb(BND, "input"), Token::newsb(PAR, "("), Token::newsb(REF, "$prompt"), Token::newsb(PAR, ")")]);
		// self.memory.set("input", bi_input);
		// self.memory.set_protection("input", 1u8);
		// let mut bi_sum = Token::news(FUN, "sum", LIST_TOKEN);
		// bi_sum.extend(vec![Token::newsb(REF, "list"), Token::newsb(SEP, "*"), Token::newsb(KEY, "return"), Token::newsb(BND, "sum"), Token::newsb(PAR, "("), Token::newsb(REF, "$list"), Token::newsb(PAR, ")")]);
		// self.memory.set("sum", bi_sum);
		// self.memory.set_protection("sum", 1u8);
		// let mut bi_number = Token::news(FUN, "Number", LIST_TOKEN);
		// bi_number.extend(vec![Token::newsb(REF, "value"), Token::newsb(SEP, "*"), Token::newsb(KEY, "return"), Token::newsb(BND, "Number"), Token::newsb(PAR, "("), Token::newsb(REF, "$value"), Token::newsb(PAR, ")")]);
		// self.memory.set("Number", bi_number);
		// self.memory.set_protection("Number", 1u8);
		// let mut bi_string = Token::news(FUN, "String", LIST_TOKEN);
		// bi_string.extend(vec![Token::newsb(REF, "value"), Token::newsb(SEP, "*"), Token::newsb(KEY, "return"), Token::newsb(BND, "String"), Token::newsb(PAR, "("), Token::newsb(REF, "$value"), Token::newsb(PAR, ")")]);
		// self.memory.set("String", bi_string);
		// self.memory.set_protection("String", 1u8);
	}
	pub fn run (&mut self) -> u8 {
		self.memory.new_scope();
		self.eval(self.tokens.clone());
		// self.LOG.dump();
		return 0;
	}
}

fn read_in_file (filename : &str) -> Vec<Token> {
	let mut file = match File::open(filename.to_owned()+FILE_EXT) {Ok(f)=>f,_=>{return Vec::new()}};
	let mut contents = String::new();
	match file.read_to_string(&mut contents) {Ok(x)=>x,_=>{return Vec::new()}};
	let contents : Vec<&str> = contents.split("\n").collect();
	return tokenize(contents);
}