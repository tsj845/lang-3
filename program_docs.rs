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
	pub fn tt (&self) -> u8 {
		// properties
		self.tt;
	}
}

impl Token {
	// panics on invalid key
	pub fn _check_vkey_dict (&self, key : &str) -> () {
		// properties
		self.tt;
		self.dict;
	}
	pub fn getd (&self, key : String) -> Token {
		// properties
		self.tt;
		self.dict;
	}
	pub fn setd (&mut self, key : String, value : Token) -> () {
		// properties
		self.tt;
		self.dict;
	}
	pub fn popd (&mut self, key : String) -> () {
		// properties
		self.tt;
		self.dict;
	}
}

impl Token {
	pub fn get (&self, key : usize) -> Token {
		// properties
		self.tt;
		self.list;
		self.length;
	}
	pub fn set (&mut self, key : usize, value : Token) -> () {
		// properties
		self.tt;
		self.list;
		self.length;
	}
	pub fn push (&mut self, value : Token) -> () {
		// properties
		self.tt;
		self.list;
		self.length;
	}
	pub fn pop (&mut self) -> Token {
		// properties
		self.tt;
		self.list;
		self.length;
	}
	pub fn popitem(&mut self, key : usize) -> Token {
		// properties
		self.tt;
		self.list;
		self.length;
	}
}

struct VarScopes {
	scopes : Vec<HashMap<String, Token>>,
	scope_count : usize,
	var_flags : Vec<HashMap<String, u8>>,
}

impl VarScopes {
	pub fn new () -> VarScopes {}
	fn dumpscope (&self, index : usize) -> () {
		// properties
		self.scopes;
	}
	pub fn dump (&self, sid : usize) -> () {
		// properties
		self.scopes;
		// methods
		self.dumpscope;
	}
	fn find_flag (&self, varname : String) -> u8 {
		// properties
		self.scope_count;
		self.var_flags;
	}
	pub fn var_has_flag (&self, varname : String) -> bool {
		// properties
		self.scope_count;
		self.var_flags;
	}
	pub fn flag_var (&mut self, varname : String, flag_value : u8) -> () {
		// properties
		self.scope_count;
		self.var_flags;
	}
	pub fn write_to_scope (&mut self, mut id : usize, name : &str, value : Token) -> () {
		// properties
		self.scopes;
	}
	pub fn new_scope (&mut self) -> () {
		// properties
		self.scope_count;
		self.scopes;
		self.var_flags;
	}
	pub fn rem_scope (&mut self) -> () {
		// properties
		self.scope_count;
		self.scopes;
		self.var_flags;
	}
	fn get_r (&self, name : &str) -> Token {
		// properties
		self.scope_count;
		self.scopes;
	}
	fn get_f (&self, name : &str) -> Token {
		// properties
		self.scope_count;
		self.scopes;
	}
	pub fn get (&self, name : &str) -> Token {}
	fn set_r (&mut self, name : &str, value : Token) -> () {
		// properties
		self.scope_count;
		self.scopes;
	}
	fn set_f (&mut self, name : &str, value : Token) -> () {
		// properties
		self.scopes;
	}
	pub fn set (&mut self, name : &str, value : Token) -> () {}
	fn rm_r (&mut self, name : &str) -> () {
		// properties
		self.scope_count;
		self.scopes;
	}
	fn rm_f (&mut self, name : &str) -> () {
		// properties
		self.scope_count;
		self.scopes;
	}
	fn rm (&mut self, name : &str) -> () {}
	pub fn garbage (&mut self, name : &str) -> () {
		// properties
		self.scope_count;
		self.scopes;
	}
	pub fn clear (&mut self) -> () {
		// properties
		self.scope_count;
		self.scopes;
	}
}

pub struct Bindings<'a> {
    lists : HashMap<&'a str, &'a str>,
    dicts : HashMap<&'a str, &'a str>,
    strings : HashMap<&'a str, &'a str>,
    numbers : HashMap<&'a str, &'a str>,
	objects : HashMap<&'a str, &'a str>,
}

impl Bindings<'_> {
    pub fn new () -> Bindings<'static> {}
    pub fn get_type (&self, i : u8, t : &str) -> &&str {
		// properties
		self.strings;
		self.lists;
		self.dicts;
		self.numbers;
	}
    pub fn check_valid (&self, t : &Token, target : &str) -> bool {
		// properties
        self.strings;
        self.numbers;
        self.lists;
        self.dicts;
    }
	pub fn check_object (&self, t : &Token) -> bool {
		// properties
		self.objects;
	}
}

struct Parser {
	tokens : Vec<Token>,
	memory : VarScopes,
	SEPTOK : Token,
	UDFTOK : Token,
	BINDINGS : Bindings<'static>,
	terminating_newlines : u32,
	print_sep_spaces : u32,
}

impl Parser {
	pub fn new (tokens : Vec<Token>) -> Parser {}
	pub fn run (&mut self) -> u8 {
		// properties
		self.memory;
		self.tokens;
		// methods
		self.eval;
	}
	fn __fault (&self) -> Token {
		// properties
		self.UDFTOK;
	}
	fn eval (&mut self, mut tokens : Vec<Token>) -> Token {
		// properties
		self.memory;
		self.BINDINGS;
		self.UDFTOK;
		// methods
		self.printop;
		self.func_call;
		self.derefb;
		self.deref;
		self.dumpscope;
		self.assignment_operation;
		self.execute;
	}
	fn execute (&mut self, mut tokens : Vec<Token>, mut i : usize) -> (usize, Token, Vec<Token>) {
		// properties
		self.memory;
		self.BINDINGS;
		// methods
		self.__fault;
		self.get_value;
	}
	fn get_value (&self, tokens : &Vec<Token>, mut i : usize) -> (usize, Token) {
		// methods
		self.eval_exp;
		self.__fault;
	}
	fn eval_exp (&self, mut toks : Vec<Token>) -> Token {
		// methods
		self.operation;
		self.deref;
		self.derefb;
	}
	fn func_call (&mut self, i : usize, tokens : &mut Vec<Token>) -> usize {
		// properties
		self.memory;
		// methods
		self.derefb;
	}
	fn parse_of (&self, i : usize, tokens : &Vec<Token>) -> Token {
		// methods
		self.derefb;
	}
	fn dumpscope (&self, mut i : usize, tokens : &Vec<Token>) -> usize {
		// properties
		self.memory;
	}
	fn printop (&self, mut i : usize, tokens : &Vec<Token>) -> usize {
		// properties
		self.SEPTOK;
		self.UDFTOK;
		// methods
		self.deref;
		self.gen_op;
		self.parse_of;
	}
	fn gen_op (&self, mut t1 : Token, t2 : Token, mut t3 : Token) -> Token {
		// methods
		self.deref;
		self.__fault;
	}
	fn deref (&self, mut t : Token) -> Token {
		// properties
		self.memory;
		// methods
		self.__fault;
	}
	fn derefb (&self, t : &Token) -> Token {
		// properties
		self.memory;
		// methods
		self.__fault;
	}
	fn assignment_operation (&self, operand : &str, v1 : String, v2 : String) -> Token {
		// methods
		self.operation;
	}
	fn operation (&self, operand : &str, v1 : String, v2 : String) -> Token {
		// methods
		self.__fault;
		self.addition;
		self.subtraction;
		self.multiplication;
		self.division;
	}
	fn addition (&self, v1 : String, v2 : String) -> Token {
		// methods
		self.__fault;
	}
	fn subtraction (&self, v1 : String, v2 : String) -> Token {
		// methods
		self.__fault;
	}
	fn multiplication (&self, v1 : String, v2 : String) -> Token {
		// methods
		self.__fault;
	}
	fn division (&self, v1 : String, v2 : String) -> Token {
		// methods
		self.__fault;
	}
}