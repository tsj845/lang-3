use crate::statics::{Token, UDF, BASE_TOKEN};
use std::collections::HashMap;

pub struct VarScopes {
	scopes : Vec<HashMap<String, Token>>,
}

impl VarScopes {
	pub fn new () -> VarScopes {
		VarScopes {
			scopes : vec![HashMap::new()],
		}
	}
	fn dumpscope (&self, index : usize) {
		println!("dumping var scope {}", index);
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
		self.scopes.push(HashMap::new());
	}
	pub fn rem_scope (&mut self) {
		self.scopes.pop();
	}
	pub fn get (&self, name : &str) -> Token {
		let mut i = self.scopes.len()-1;
		loop {
			if self.scopes[i].contains_key(name.clone()) {
				return self.scopes[i].get(&name.to_string()).unwrap().clone();
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		return Token::new(UDF, String::from("UDF"), BASE_TOKEN);
	}
	pub fn set (&mut self, name : &str, value : Token) {
		let l = self.scopes.len()-1;
		self.scopes[l].insert(name.to_string(), value);
	}
}