use crate::statics::CONST_VARS;

fn v_bs_to_v_hs (original : Vec<&str>) -> Vec<String> {
	let mut v : Vec<String> = Vec::new();
	let mut i = 0usize;
	let l = original.len();
	loop {
		if i >= l {
			break;
		}
		v.push(String::from(original[i]));
		i += 1;
	}
	return v;
}

pub struct VarSpace {
	names : Vec<String>,
	values : Vec<Vec<String>>,
}

impl VarSpace {
	pub fn new () -> VarSpace {
		VarSpace {
			names : Vec::new(),
			values : Vec::new(),
		}
	}
	pub fn add (&mut self, name : &str, value : Vec<&str>) {
		self.names.push(String::from(name));
		self.values.push(v_bs_to_v_hs(value));
	}
	pub fn index (&self, name : &str) -> i32 {
		let mut i = 0i32;
		let l = self.names.len();
		loop {
			if i >= l as i32 {
				break;
			}
			if self.names[i as usize] == name {
				return i;
			}
			i += 1;
		}
		return -1;
	}
	pub fn has (&self, name : &str) -> bool {
		return self.index(name) >= 0;
	}
	pub fn get (&self, name : &str) -> &Vec<String> {
		let index = self.index(name);
		if index < 0 {
			panic!("could not find element");
		}
		return &self.values[index as usize];
	}
	pub fn set (&mut self, name : &str, value : Vec<&str>) {
		let index = self.index(name);
		if index < 0 {
			panic!("could not find element");
		}
		self.values[index as usize] = v_bs_to_v_hs(value);
	}
}

pub struct VarSpaces {
	spaces : Vec<VarSpace>,
}

impl VarSpaces {
	pub fn new () -> VarSpaces {
		VarSpaces {
			spaces : vec![VarSpace::new(), VarSpace::new()],
		}
	}
	pub fn write_constants (&mut self, values : [[&str;3];CONST_VARS.len()]) {
		let mut i = 0usize;
		let l = values.len();
		loop {
			if i >= l {
				break;
			}
			self.write_to_scope(0usize, values[i][0], vec![values[i][1], values[i][2]]);
			i += 1;
		}
	}
	pub fn write_to_scope (&mut self, mut id : usize, name : &str, value : Vec<&str>) {
		if id > 2 {
			panic!("invalid scope id");
		}
		if id == 2 {
			id = self.spaces.len() - 1;
		}
		let mut scope = &mut self.spaces[id];
		if scope.has(name) {
			scope.set(name, value);
		} else {
			scope.add(name, value);
		}
	}
	pub fn new_scope (&mut self) {
		self.spaces.push(VarSpace::new());
	}
	pub fn rem_scope (&mut self) {
		self.spaces.pop();
	}
	pub fn find (&self, name : &str) -> (i32, i32) {
		let mut i = self.spaces.len() - 1;
		loop {
			if self.spaces[i].has(name.clone()) {
				return (i as i32, self.spaces[i].index(name));
			}
			if i == 0 {
				break;
			}
			i -= 1;
		}
		return (-1i32, -1i32);
	}
	pub fn get (&self, name : &str) -> &Vec<String> {
		let (y, x) = self.find(name.clone());
		if y < 0 {
			panic!("could not find value location");
		}
		return &self.spaces[y as usize].values[x as usize];
	}
	pub fn set (&mut self, name : &str, value : Vec<&str>) {
		let (y, x) = self.find(name.clone());
		if y < 0 {
			panic!("could not find value location");
		}
		self.spaces[y as usize].values[x as usize] = v_bs_to_v_hs(value);
	}
	pub fn add (&mut self, name : &str, value : Vec<&str>) {
		let last = self.spaces.len()-1;
		if self.spaces[last].has(name.clone()) {
			panic!("scope already has value");
		}
		self.spaces[last].add(name, value);
	}
}