use crate::replacer::Replacer;
use std::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

// a print operation that can accept any number of arguments then prints them
#[macro_export]
macro_rules! printv {
	// vodoo
	( $( $x:expr ),* ) => {
		// more vodoo
		{
			// even more vodoo
			$(
				// prints the argument
				print!("{} ", $x);
			)*
			// prints a newline
			print!("\n");
		}
	};
}

// function that prints list contents
pub fn printlst<T: std::fmt::Display> (lst : &Vec<T>) {
	print!("[ ");
	for item in lst {
		print!(" '{}', ", item);
	}
	println!(" ]");
}

// namespace identifiers
pub const GLOBAL : usize = 0;
pub const LOCAL : usize = 1;

// memory optimization
pub const BASE_TOKEN : u8 = 0;
pub const DICT_TOKEN : u8 = 1;
pub const LIST_TOKEN : u8 = 2;

// token types
pub const NUL : u8 = 0;
pub const FUN : u8 = 1;
pub const REF : u8 = 2;
pub const LIT : u8 = 3;
pub const KEY : u8 = 4;
pub const MAT : u8 = 5;
pub const LOG : u8 = 6;
pub const ASS : u8 = 7;
pub const PAR : u8 = 8;
pub const LST : u8 = 9;
pub const DCT : u8 = 10;
pub const SEP : u8 = 11;
pub const SYM : u8 = 12;
pub const GRP : u8 = 13;
pub const DOT : u8 = 14;
pub const NLN : u8 = 15;
pub const UDF : u8 = 16;
pub const MET : u8 = 17;
pub const TOKEN_ARRAY : [&str; 18] = ["NUL", "FUN", "REF", "LIT", "KEY", "MAT", "LOG", "ASS", "PAR", "LST", "DCT", "SEP", "SYM", "GRP", "DOT", "NLN", "UDF", "MET"];
pub const FILE_EXT : &str = ".ihl";

// program keywords
pub const KEYWORDS : [&str; 12] = ["gloabl", "local", "func", "print", "of", "dumpscope", "rm", "garbage", "log", "return", "dumptoks", "dumplc"];

// tokenization regex patterns
pub const WORD_RE_PAT : &str = r"[[:alpha:]]+[[:word:]]*";
pub const CONTAINER_RE_PAT : &str = r#"[{\["(]"#;
pub const NUMBER_RE_PAT : &str = r"^(0b[01]+|0x[0-9a-f]+|[0-9]+(\.[0-9]+)?)";
pub const DECI_RE_PAT : &str = r"[0-9]+\.[0-9]+";
pub const LITERAL_RE_PAT : &str = r"true|false|null";
pub const PAREN_RE_PAT : &str = r"[()]";
pub const GROUP_RE_PAT : &str = r"$?[{}\[\]]";
pub const SEPER_RE_PAT : &str = r"[:,]";
pub const KEYWD_RE_PAT : &str = r"global|local|func|print|of|dumpscope|rm|garbage|log|return|dumptoks|dumplc";
pub const ASIGN_RE_PAT : &str = r"=";
pub const MATHM_RE_PAT : &str = r"[-+*/]";
pub const TOKEN_STR_RE_PAT : &str = r#"".*""#;
pub const TOKEN_BIN_NUM_RE_PAT : &str = r"^0b[01]+";
pub const TOKEN_HEX_NUM_RE_PAT : &str = r"^0x[0-9a-f]+";
pub const TOKEN_DEC_NUM_RE_PAT : &str = r"^[0-9]+(\.[0-9]+)?";

// data types
pub const DT_UDF : u8 = 0;
pub const DT_STR : u8 = 1;
pub const DT_NUM : u8 = 2;
pub const DT_BOL : u8 = 3;
pub const DT_LST : u8 = 4;
pub const DT_DCT : u8 = 5;

pub struct Token {
	pub id : u8,
	pub value : String,
	pub dict : Option<HashMap<String, Token>>,
	pub list : Option<Vec<Token>>,
	pub length : usize,
	pub data_type : u8,
	tt : u8,
}

impl Token {
	pub fn new (id : u8, mut value : String, tt : u8) -> Token {
		lazy_static! {
			static ref STRING_RE : Regex = Regex::new(TOKEN_STR_RE_PAT).unwrap();
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref BIN_NUM_RE : Regex = Regex::new(TOKEN_BIN_NUM_RE_PAT).unwrap();
			static ref HEX_NUM_RE : Regex = Regex::new(TOKEN_HEX_NUM_RE_PAT).unwrap();
			static ref DEC_NUM_RE : Regex = Regex::new(TOKEN_DEC_NUM_RE_PAT).unwrap();
			static ref REPLACER : Replacer = Replacer::new();
		}
		let mut data_type : u8 = DT_UDF;
		if id == LIT {
			if STRING_RE.is_match(&value) {
				data_type = DT_STR;
				value = REPLACER.replace(r"\x1b", REPLACER.BACKSLASH.clone(), REPLACER.replace(r"\t", REPLACER.BACKSLASH.clone(), REPLACER.replace(r"\n", REPLACER.BACKSLASH.clone(), value, "\n"), "\t"), "\x1b").replace(r"\\", r"\");
			} else if NUMBER_RE.is_match(&value) {
				data_type = DT_NUM;
				if !DEC_NUM_RE.is_match(&value) {
					let mut n : i32 = 0;
					let mut place : u32 = 1;
					let mut v : Vec<char> = value.chars().collect::<Vec<char>>();
					v.reverse();
					if BIN_NUM_RE.is_match(&value) {
						for c in v {
							if c == 'b' {
								break;
							}
							n += (place * c.to_digit(2).unwrap()) as i32;
							place *= 2;
						}
					} else if HEX_NUM_RE.is_match(&value) {
						for c in v {
							if c == 'x' {
								break;
							}
							n += (place * c.to_digit(16).unwrap()) as i32;
							place *= 16;
						}
					}
					value = n.to_string();
				}
			}
		} else if id == LST {
			data_type = DT_LST;
		} else if id == DCT {
			data_type = DT_DCT;
		}
		if tt == BASE_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : None,
				list : None,
				length : 0,
				data_type : data_type,
				tt : tt,
			};
		} else if tt == DICT_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : Some(HashMap::new()),
				list : None,
				length : 0,
				data_type : data_type,
				tt : tt,
			};
		} else if tt == LIST_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : None,
				list : Some(Vec::new()),
				length : 0,
				data_type : data_type,
				tt : tt,
			};
		}
		panic!("invalid optimization type");
	}
	pub fn news (id : u8, value : &str, tt : u8) -> Token {
		return Token::new(id, value.to_string(), tt);
	}
	pub fn tt (&self) -> u8 {
		return self.tt;
	}
}

// dict methods
impl Token {
	pub fn _check_vkey_dict (&self, key : &str) {
		if !self.dict.as_ref().unwrap().contains_key(key) {
			panic!("invalid key");
		}
	}
	pub fn getd (&self, key : String) -> Token {
		if self.tt != DICT_TOKEN {
			panic!("invalid operation");
		}
		self._check_vkey_dict(&key);
		return self.dict.as_ref().unwrap().get(&key).unwrap().clone();
	}
	pub fn setd (&mut self, key : String, value : Token) {
		if self.tt != DICT_TOKEN {
			panic!("invalid operation");
		}
		self.dict.as_mut().unwrap().insert(key, value);
	}
	pub fn popd (&mut self, key : String) -> Token {
		if self.tt != DICT_TOKEN {
			panic!("invalid operation");
		}
		self._check_vkey_dict(&key);
		return self.dict.as_mut().unwrap().remove(&key).unwrap();
	}
}

// list methods
impl Token {
	pub fn get (&self, key : usize) -> Token {
		if self.tt != LIST_TOKEN {
			panic!("invalid operation");
		}
		if self.length <= key {
			panic!("index out of range");
		}
		return self.list.as_ref().unwrap()[key].clone();
	}
	pub fn set (&mut self, key : usize, value : Token) {
		if self.tt != LIST_TOKEN {
			panic!("invalid operation");
		}
		if self.length <= key {
			panic!("index out of range");
		}
		self.list.as_mut().unwrap()[key] = value;
	}
	pub fn push (&mut self, v : Token) {
		if self.tt != LIST_TOKEN {
			panic!("invalid operation");
		}
		self.length += 1;
		self.list.as_mut().unwrap().push(v);
	}
	pub fn pop (&mut self) -> Token {
		if self.tt != LIST_TOKEN {
			panic!("invalid operation");
		}
		if self.length == 0 {
			panic!("can't remove from empty list");
		}
		self.length -= 1;
		return self.list.as_mut().unwrap().pop().unwrap();
	}
	pub fn popitem (&mut self, key : usize) -> Token {
		if self.tt != LIST_TOKEN {
			panic!("invalid operation");
		}
		if self.length <= key {
			panic!("index out of range");
		}
		self.length -= 1;
		return self.list.as_mut().unwrap().remove(key);
	}
}

impl std::clone::Clone for Token {
	fn clone (&self) -> Token {
		Token {
			id : self.id,
			value : self.value.to_string(),
			dict : self.dict.clone(),
			list : self.list.clone(),
			length : self.length,
			data_type : self.data_type,
			tt : self.tt,
		}
	}
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
		.field("id", &self.id)
		.field("value", &self.value)
		.finish()
    }
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, \"{}\")", TOKEN_ARRAY[self.id as usize], self.value)
	}
}

impl std::cmp::PartialEq for Token {
	fn eq (&self, other: &Token) -> bool {
		return other.id == self.id && other.value == self.value && other.data_type == self.data_type;
	}
}