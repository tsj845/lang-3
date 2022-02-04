pub struct Var_space {
	names : Vec<String>,
	values : Vec<Vec<String>>,
}

impl Var_space {
	pub fn new () -> Var_space {
		Var_space {
			names : Vec::new(),
			values : Vec::new(),
		}
	}
	pub fn add (&mut self, name : String, value : Vec<String>) {
		self.names.push(name);
		self.values.push(value);
	}
	pub fn index (&mut self, name : String) -> i32 {
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
	pub fn has (&mut self, name : String) -> bool {
		return self.index(name) >= 0;
	}
	pub fn get (&mut self, name : String) -> &Vec<String> {
		let index = self.index(name);
		if index < 0 {
			panic!("could not find element");
		}
		return &self.values[index as usize];
	}
	pub fn set (&mut self, name : String, value : Vec<String>) {
		let index = self.index(name);
		if index < 0 {
			panic!("could not find element");
		}
		self.values[index as usize] = value;
	}
}

pub struct Var_spaces {
	spaces : Vec<Var_space>,
}

impl Var_spaces {
	pub fn new () -> Var_spaces {
		Var_spaces {
			spaces : vec![Var_space::new(), Var_space::new()],
		}
	}
	pub fn new_scope (&mut self) {
		self.spaces.push(Var_space::new());
	}
	pub fn rem_scope (&mut self) {
		self.spaces.pop();
	}
	pub fn find (&mut self, name : String) -> (i32, i32) {
		let mut i = self.spaces.len() - 1;
		loop {
			if i < 0 {
				break;
			}
			if self.spaces[i].has(name.clone()) {
				return (i as i32, self.spaces[i].index(name));
			}
			i -= 1;
		}
		return (-1i32, -1i32);
	}
	pub fn get (&mut self, name : String) -> &Vec<String> {
		let (y, x) = self.find(name.clone());
		if y < 0 {
			panic!("could not find value location");
		}
		return &self.spaces[y as usize].values[x as usize];
	}
	pub fn set (&mut self, name : String, value : Vec<String>) {
		let (y, x) = self.find(name.clone());
		if y < 0 {
			panic!("could not find value location");
		}
		self.spaces[y as usize].values[x as usize] = value;
	}
}