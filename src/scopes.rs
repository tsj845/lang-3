use crate::statics::{Token, UDF, BASE_TOKEN};
use crate::static_colors::*;
use std::collections::HashMap;
use rand::random;

/**
 * variable flags
 * 0 - (default behavior, local) read up, write up
 * 1 - read up, write down
 * 2 - read down, write up
 * 3 - (global) read down, write down
 * 4 - (unique) only accessed in the scope it was defined in
 * 5 - (parent) go up exactly one level
 * global flags
 * 0 - (default behavior, normal) program managed
 * 1 - (protected) protected object, immutable and always accessed as global
 */

pub struct VarScopes {
	pub pointer_mem : HashMap<String, Token>,
	scopes : Vec<HashMap<String, Token>>,
	scope_count : usize,
	var_flags : Vec<HashMap<String, u8>>,
	gv_flags : HashMap<String, u8>,
	inter_flags : HashMap<String, u8>,
}

impl VarScopes {
	pub fn new () -> VarScopes {
		VarScopes {
			pointer_mem : HashMap::new(),
			scopes : vec![HashMap::new()],
			scope_count : 1,
			var_flags : vec![HashMap::new()],
			gv_flags : HashMap::new(),
			inter_flags : HashMap::new(),
		}
	}
	pub fn dump_flags (&self) {
		let mut i : usize = 0;
		let l = self.var_flags.len();
		loop {
			if i >= l {
				break;
			}
			println!("\n{}dumping\x1b[39m flags {}{}\x1b[0m", DEBUG_PURPLE, DEBUG_BLUE_SCOPE_DUMP, i);
			for (flag, v) in &self.var_flags[i] {
				println!("{} : {}", flag, v);
			}
			i += 1;
		}
		println!("");
	}
	fn dumpscope (&self, index : usize, show_options : u32) {
		println!("\n{}dumping\x1b[39m var scope {}{}\x1b[0m:", DEBUG_PURPLE, DEBUG_BLUE_SCOPE_DUMP, index);
		for (key, val) in &self.scopes[index] {
			if self.find_gv(key.to_string()) == 1 {
				if show_options & 0b1 == 1 {
					continue;
				}
				println!("{} : {} {}protected\x1b[39m", key, val, DEBUG_BLUE_SCOPE_DUMP);
			} else if self.find_flag_for_scope(key.clone(), index) == 4 {
				println!("{} : {} {}unique\x1b[39m", key, val, DEBUG_BLUE_SCOPE_DUMP);
			} else {
				if show_options & 0b10 == 2 && key.starts_with("\"ptr ") {
					continue;
				}
				println!("{} : {}", key, val);
			}
		}
	}
	pub fn dump (&self, sid : usize, show_options : u32) {
		if sid == 2 {
			let mut i : usize = 0;
			let l = self.scopes.len();
			loop {
				if i >= l {
					break;
				}
				self.dumpscope(i, show_options);
				i += 1;
			}
		} else {
			self.dumpscope(sid, show_options);
		}
	}
	fn find_gv (&self, varname : String) -> u8 {
		if self.gv_flags.contains_key(&varname) {
			return self.gv_flags.get(&varname).unwrap().clone();
		}
		return 0u8;
	}
	pub fn find_flag (&self, varname : String) -> u8 {
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
	fn find_flag_for_scope (&self, varname : String, scope : usize) -> u8 {
		return self.var_flags[scope].get(&varname).unwrap_or(&0u8).clone();
	}
	pub fn var_has_flag (&self, varname : String) -> bool {
		return self.var_flags[self.scope_count-1].contains_key(&varname);
	}
	pub fn flag_var (&mut self, varname : String, flag_value : u8) {
		self.var_flags[self.scope_count-1].insert(varname, flag_value);
	}
	pub fn set_protection (&mut self, varname : &str, flag_value : u8) {
		self.gv_flags.insert(varname.to_string(), flag_value);
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
	// ensures that all pointers are unique
	pub fn get_ptr_name (&self, name : &str) -> String {
		let mut cand = String::from("-scope-") + &(self.scope_count-1).to_string() + name + &random::<u32>().to_string();
		loop {
			if !self.pointer_mem.contains_key(&cand) {
				break;
			}
			cand = String::from("-scope-") + &(self.scope_count-1).to_string() + name + &random::<u32>().to_string();
		}
		return cand;
	}
	pub fn ptr_alloc (&mut self, ptr_name : &str) {
		self.scopes[self.scope_count-1].insert(String::from("\"ptr ")+ptr_name, Token::newsb(UDF, "UDF"));
	}
	pub fn ptr_alloc_global (&mut self, ptr_name : &str) {
		self.scopes[0].insert(String::from("\"ptr ")+ptr_name, Token::newsb(UDF, "UDF"));
	}
	pub fn new_scope (&mut self) {
		self.scope_count += 1;
		self.scopes.push(HashMap::new());
		self.var_flags.push(HashMap::new());
	}
	pub fn rem_scope (&mut self) {
		self.scope_count -= 1;
		for (key, _) in &self.scopes[self.scope_count] {
			if key.starts_with("\"ptr ") {
				self.pointer_mem.remove(&key[5..]);
			}
		}
		self.scopes.pop();
		self.var_flags.pop();
	}
	fn get_r (&self, name : &str) -> Token {
		let mut i : usize = self.scope_count-1;
		loop {
			// println!("GRD: {}, {}, {}", name, self.scopes[i].contains_key(name), self.find_flag_for_scope(name.to_owned(), i));
			if self.scopes[i].contains_key(name) && (self.find_flag_for_scope(name.to_owned(), i) != 4 || i == self.scope_count-1) {
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
			if self.scopes[i].contains_key(name.clone()) && (self.find_flag_for_scope(name.to_owned(), i) != 4 || i == self.scope_count-1) {
				return self.scopes[i].get(&name.to_string()).unwrap().clone();
			}
			i += 1;
		}
		return Token::news(UDF, "UDF", BASE_TOKEN);
	}
	fn get_p (&self, name : &str) -> Token {
		return self.scopes[self.scope_count-2].get(name).unwrap_or(&Token::news(UDF, "UDF", BASE_TOKEN)).clone();
	}
	pub fn get (&self, name : &str) -> Token {
		if self.find_gv(name.to_string()) == 1 {
			return self.scopes[0].get(name).unwrap().clone();
		}
		if name.starts_with("\"pth ") {
			let things : Vec<&str> = name.split(' ').collect();
			let memblock : &Token = self.pointer_mem.get(&self.get(things[1]).value).unwrap();
			return memblock.getd(things[2].to_owned()).unwrap();
		}
		let flag = self.find_flag(name.to_string());
		if flag == 5 {
			return self.get_p(name);
		}
		if flag > 1 {
			return self.get_f(name);
		}
		return self.get_r(name);
	}
	fn set_r (&mut self, name : &str, value : Token) {
		let mut i : usize = self.scope_count-1;
		loop {
			if self.scopes[i].contains_key(name.clone()) && (self.find_flag_for_scope(name.to_owned(), i) != 4 || i == self.scope_count-1) {
				self.scopes[i].insert(name.to_string(), value);
				return;
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		self.scopes[self.scope_count-1].insert(name.to_string(), value);
	}
	fn set_f (&mut self, name : &str, value : Token) {
		let mut i : usize = 0;
		loop {
			if i >= self.scope_count {
				break;
			}
			if self.scopes[i].contains_key(name.clone()) && (self.find_flag_for_scope(name.to_owned(), i) != 4 || i == self.scope_count-1) {
				self.scopes[i].insert(name.to_string(), value);
				return;
			}
			i += 1;
		}
		self.scopes[0].insert(name.to_string(), value);
	}
	fn set_p (&mut self, name : &str, value : Token) {
		self.scopes[self.scope_count-2].insert(name.to_string(), value);
	}
	pub fn set (&mut self, name : &str, value : Token) {
		if self.find_gv(name.to_string()) == 1 {
			return;
		}
		if name.starts_with("\"pth ") {
			let things : Vec<&str> = name.split(' ').collect();
			let memblock : &mut Token = self.pointer_mem.get_mut(&self.get(things[1]).value).unwrap();
			memblock.setd(things[2].to_owned(), value).unwrap();
			return;
		}
		let flag = self.find_flag(name.to_string());
		if flag % 2 == 0 && flag < 3 {
			self.set_r(name, value);
		} else if flag < 4 {
			self.set_f(name, value);
		} else if flag == 5 {
			self.set_p(name, value);
		} else if flag == 4 {
			self.scopes[self.scope_count-1].insert(name.to_string(), value);
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
	pub fn clear (&mut self) {
		let mut i : usize = 0;
		loop {
			if i >= self.scope_count {
				break;
			}
			self.scopes[i].clear();
			i += 1;
		}
	}
	pub fn inter_flag_set (&mut self, varname : &str, flag : u8) {
		self.inter_flags.insert(varname.to_owned(), flag);
	}
	pub fn inter_flag_get (&self, varname : &str) -> u8 {
		return self.inter_flags.get(varname).unwrap_or(&0u8).clone();
	}
}