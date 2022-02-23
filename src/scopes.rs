use crate::statics::{Token, UDF, BASE_TOKEN};
use std::collections::HashMap;

/**
 * variable flags
 * 0 - (default behavior, local) read up, write up
 * 1 - read up, write down
 * 2 - read down, write up
 * 3 - (global) read down, write down
 */

pub struct VarScopes {
	scopes : Vec<HashMap<String, Token>>,
	scope_count : usize,
	var_flags : Vec<HashMap<String, u8>>,
}

impl VarScopes {
	pub fn new () -> VarScopes {
		VarScopes {
			scopes : vec![HashMap::new()],
			scope_count : 1,
			var_flags : vec![HashMap::new()],
		}
	}
	fn dumpscope (&self, index : usize) {
		println!("\n\x1b[38;2;202;78;202mdumping\x1b[39m var scope \x1b[38;2;25;150;255m{}\x1b[0m:", index);
		for (key, val) in &self.scopes[index] {
			println!("{} : {}", key, val);
		}
	}
	pub fn dump (&self, sid : usize) {
		if sid == 2 {
			let mut i : usize = 0;
			let l = self.scopes.len();
			loop {
				if i >= l {
					break;
				}
				self.dumpscope(i);
				i += 1;
			}
		} else {
			self.dumpscope(sid);
		}
	}
	fn find_flag (&self, varname : String) -> u8 {
		let mut i : usize = self.scope_count-1;
		loop {
			if self.var_flags[i].contains_key(&varname) {
				return self.var_flags[i].get(&varname).unwrap().clone();
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		return 0u8;
	}
	pub fn var_has_flag (&self, varname : String) -> bool {
		return self.var_flags[self.scope_count-1].contains_key(&varname);
	}
	pub fn flag_var (&mut self, varname : String, flag_value : u8) {
		self.var_flags[self.scope_count-1].insert(varname, flag_value);
	}
	pub fn write_to_scope (&mut self, mut id : usize, name : &str, value : Token) {
		if id > 1 {
			panic!("invalid scope id");
		}
		if id == 1 {
			id = self.scopes.len() - 1;
		}
		let scope = &mut self.scopes[id];
		scope.insert(name.to_string(), value);
	}
	pub fn new_scope (&mut self) {
		self.scope_count += 1;
		self.scopes.push(HashMap::new());
		self.var_flags.push(HashMap::new());
	}
	pub fn rem_scope (&mut self) {
		self.scope_count -= 1;
		self.scopes.pop();
		self.var_flags.pop();
	}
	fn get_r (&self, name : &str) -> Token {
		let mut i : usize = self.scope_count-1;
		loop {
			if self.scopes[i].contains_key(name.clone()) {
				return self.scopes[i].get(&name.to_string()).unwrap().clone();
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		return Token::news(UDF, "UDF", BASE_TOKEN);
	}
	fn get_f (&self, name : &str) -> Token {
		let mut i : usize = 0;
		loop {
			if i >= self.scope_count {
				break;
			}
			if self.scopes[i].contains_key(name.clone()) {
				return self.scopes[i].get(&name.to_string()).unwrap().clone();
			}
			i += 1;
		}
		return Token::news(UDF, "UDF", BASE_TOKEN);
	}
	pub fn get (&self, name : &str) -> Token {
		if self.find_flag(name.to_string()) > 1 {
			return self.get_r(name);
		}
		return self.get_f(name);
	}
	fn set_r (&mut self, name : &str, value : Token) {
		self.scopes[self.scope_count-1].insert(name.to_string(), value);
	}
	fn set_f (&mut self, name : &str, value : Token) {
		self.scopes[0].insert(name.to_string(), value);
	}
	pub fn set (&mut self, name : &str, value : Token) {
		if self.find_flag(name.to_string()) % 2 == 0 {
			self.set_r(name, value);
		} else {
			self.set_f(name, value);
		}
	}
	fn rm_r (&mut self, name : &str) {
		let mut i : usize = self.scope_count-1;
		loop {
			if self.scopes[i].remove(name).is_some() {
				break;
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
	}
	fn rm_f (&mut self, name : &str) {
		let mut i : usize = 0;
		loop {
			if i >= self.scope_count {
				break;
			}
			if self.scopes[i].remove(name).is_some() {
				break;
			}
			i += 1;
		}
	}
	pub fn rm (&mut self, name : &str) {
		if self.find_flag(name.to_string()) > 1 {
			self.rm_f(name);
		} else {
			self.rm_r(name);
		}
	}
	pub fn garbage (&mut self, name : &str) {
		let mut i : usize = 0;
		loop {
			if i >= self.scope_count {
				break;
			}
			self.scopes[i].remove(name);
			i += 1;
		}
	}
}