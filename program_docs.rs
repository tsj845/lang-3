// implements: Clone, Display, Debug
struct Token {
	pub list : Option<Vec<Token>>,
	pub dict : Option<HashMap<String, Token>>,
	pub length : usize,
	pub id : u8,
	pub data_type : u8,
	pub value : String,
	tt : u8,
}

impl Token {
	pub fn new (id : u8, value : String, tt : u8) -> Token {}
	pub fn news (id : u8, value : &str, tt : u8) -> Token {}
	pub fn tt (&self) -> u8 {}
	// panics on invalid key
	pub fn _check_vkey_dict (&self, key : &str) -> () {}
	pub fn getd (&self, key : String) -> Token {}
	pub fn setd (&mut self, key : String, value : Token) -> () {}
	pub fn popd (&mut self, key : String) -> () {}
	pub fn get (&self, key : usize) -> Token {}
	pub fn set (&mut self, key : usize, value : Token) -> () {}
	pub fn push (&mut self, value : Token) -> () {}
	pub fn pop (&mut self) -> Token {}
	pub fn popitem(&mut self, key : usize) -> Token {}
}