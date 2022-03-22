use crate::replacer::Replacer;
use std::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

// data for interpreter info()
pub const VERSION : &str = "β1.1.class";

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
pub const IDX : u8 = 4;
pub const KEY : u8 = 5;
pub const MAT : u8 = 6;
pub const LOG : u8 = 7;
pub const ASS : u8 = 8;
pub const PAR : u8 = 9;
pub const LST : u8 = 10;
pub const DCT : u8 = 11;
pub const OBJ : u8 = 12;
pub const SEP : u8 = 13;
pub const SYM : u8 = 14;
pub const GRP : u8 = 15;
pub const DOT : u8 = 16;
pub const CTL : u8 = 17;
pub const NLN : u8 = 18;
pub const UDF : u8 = 19;
pub const MET : u8 = 20;
pub const SIG : u8 = 21;
pub const BND : u8 = 22;
pub const MOD : u8 = 23;
pub const PTH : u8 = 24;
pub const PTR : u8 = 25;
pub const TOKEN_ARRAY : [&str; 26] = ["NUL", "FUN", "REF", "LIT", "IDX", "KEY", "MAT", "LOG", "ASS", "PAR", "LST", "DCT", "OBJ", "SEP", "SYM", "GRP", "DOT", "CTL", "NLN", "UDF", "MET", "SIG", "BND", "MOD", "PTH", "PTR"];
pub const FILE_EXT : &str = ".ihl";

// program keywords
pub const KEYWORDS : [&str; 34] = ["gloabl", "local", "unique", "parent", "func", "print", "dumpscope", "rm", "garbage", "log", "return", "dumptoks", "dumplc", "dumpflags", "in", "for", "HALT", "break", "continue", "while", "if", "else", "linkup", "module", "readonly", "private", "class", "property", "method", "inheriting", "from", "dumpobj", "create", "static"];

// tokenization regex patterns
pub const WORD_RE_PAT : &str = r"[[:alpha:]]+[[:word:]]*";
pub const CONTAINER_RE_PAT : &str = r#"[{\["(]"#;
pub const NUMBER_RE_PAT : &str = r"^(0b[01]+|0x[0-9a-f]+|[0-9]+(\.[0-9]+)?)";
pub const DECI_RE_PAT : &str = r"[0-9]+\.[0-9]+";
pub const LITERAL_RE_PAT : &str = r"true|false|null";
pub const LOGIC_RE_PAT : &str = r"[!^%|&><]|(<=|>=)";
pub const PAREN_RE_PAT : &str = r"[()]";
pub const GROUP_RE_PAT : &str = r"$?[{}\[\]]";
pub const SEPER_RE_PAT : &str = r"[:,]";
pub const KEYWD_RE_PAT : &str = r"\b(global|local|unique|parent|func|print|dumpscope|rm|garbage|log|return|dumptoks|dumplc|dumpflags|in|for|HALT|break|continue|while|if|else|linkup|module|readonly|private|class|property|method|inheriting|from|dumpobj|create|static)\b";
pub const ASIGN_RE_PAT : &str = r"=";
pub const MATHM_RE_PAT : &str = r"[-+*/]";
pub const TOKEN_STR_RE_PAT : &str = r#"^".*"$"#;
pub const TOKEN_BIN_NUM_RE_PAT : &str = r"^0b[01]+";
pub const TOKEN_HEX_NUM_RE_PAT : &str = r"^0x[0-9a-f]+";
pub const TOKEN_DEC_NUM_RE_PAT : &str = r"^[0-9]+(\.[0-9]+)?";

// method supports
pub const ALPHA_RE_PAT : &str = r"^[a-zA-Z]*$";
pub const DIGIT_RE_PAT : &str = r"^[0-9]*$";

// data types
pub const DT_UDF : u8 = 0;
pub const DT_STR : u8 = 1;
pub const DT_NUM : u8 = 2;
pub const DT_BOL : u8 = 3;
pub const DT_LST : u8 = 4;
pub const DT_DCT : u8 = 5;
pub const DT_OBJ : u8 = 6;
pub const DT_MOD : u8 = 7;

pub struct Token {
	pub id : u8,
	pub cid : u8,
	pub value : String,
	pub dict : Option<HashMap<String, Token>>,
	pub list : Option<Vec<Token>>,
	pub length : usize,
	pub data_type : u8,
	pub escape : bool,
	pub line : usize,
	pub chara : usize,
	tt : u8,
}

impl Token {
	fn calc_dt (id : u8, mut value : String) -> (u8, String) {
		lazy_static! {
			static ref STRING_RE : Regex = Regex::new(TOKEN_STR_RE_PAT).unwrap();
			static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
			static ref BIN_NUM_RE : Regex = Regex::new(TOKEN_BIN_NUM_RE_PAT).unwrap();
			static ref HEX_NUM_RE : Regex = Regex::new(TOKEN_HEX_NUM_RE_PAT).unwrap();
			static ref DEC_NUM_RE : Regex = Regex::new(TOKEN_DEC_NUM_RE_PAT).unwrap();
			static ref LIT_RE : Regex = Regex::new(LITERAL_RE_PAT).unwrap();
			static ref REPLACER : Replacer = Replacer::new();
		}
		if id == LIT {
			if STRING_RE.is_match(&value) {
				return (DT_STR, REPLACER.replace(r"\x1b", REPLACER.BACKSLASH.clone(), REPLACER.replace(r"\t", REPLACER.BACKSLASH.clone(), REPLACER.replace(r"\n", REPLACER.BACKSLASH.clone(), value, "\n"), "\t"), "\x1b").replace(r"\\", r"\"));
			} else if NUMBER_RE.is_match(&value) {
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
				return (DT_NUM, value);
			} else if LIT_RE.is_match(&value) {
				return (DT_BOL, value);
			}
		} else if id == LST {
			return (DT_LST, value);
		} else if id == DCT {
			return (DT_DCT, value);
		} else if id == OBJ {
			return (DT_OBJ, value);
		} else if id == MOD {
			return (DT_MOD, value);
		}
		return (DT_UDF, value);
	}
	pub fn new (id : u8, mut value : String, tt : u8) -> Token {
		let x : (u8, String) = Token::calc_dt(id, value);
		let data_type = x.0;
		value = x.1;
		// println!("{}", TOKEN_ARRAY[id as usize]);
		if tt == BASE_TOKEN {
			return Token {
				id : id,
				cid : id,
				value : value,
				dict : None,
				list : None,
				length : 0,
				data_type : data_type,
				escape : false,
				line : 0,
				chara : 0,
				tt : tt,
			};
		} else if tt == DICT_TOKEN {
			return Token {
				id : id,
				cid : id,
				value : value,
				dict : Some(HashMap::new()),
				list : None,
				length : 0,
				data_type : data_type,
				escape : false,
				line : 0,
				chara : 0,
				tt : tt,
			};
		} else if tt == LIST_TOKEN {
			return Token {
				id : id,
				cid : id,
				value : value,
				dict : None,
				list : Some(Vec::new()),
				length : 0,
				data_type : data_type,
				escape : false,
				line : 0,
				chara : 0,
				tt : tt,
			};
		}
		panic!("invalid optimization type");
	}
	pub fn new_ptr (cid : u8, value : String, rv : String) -> Token {
		let x : (u8, String) = Token::calc_dt(cid, rv);
		Token {
			id : PTR,
			cid : cid,
			value : value,
			dict : None,
			list : None,
			length : 0,
			data_type : x.0,
			escape : false,
			line : 0,
			chara : 0,
			tt : BASE_TOKEN,
		}
	}
	pub fn news (id : u8, value : &str, tt : u8) -> Token {
		return Token::new(id, value.to_string(), tt);
	}
	pub fn newsb (id : u8, value : &str) -> Token {
		return Token::new(id, value.to_string(), BASE_TOKEN);
	}
	pub fn tt (&self) -> u8 {
		return self.tt;
	}
	pub fn meta (&mut self, line : usize, chara : usize) {
		self.line = line;
		self.chara = chara;
	}
	pub fn bool (&self) -> bool {
		if self.data_type == DT_STR {
			return self.value.len() > 2;
		}
		if self.data_type == DT_NUM {
			return self.value.parse::<f64>().unwrap() > 0f64;
		}
		if self.data_type == DT_BOL {
			return self.value == "true";
		}
		if self.data_type == UDF {
			return false;
		}
		if self.data_type == DT_LST {
			return self.list.as_ref().unwrap().len() > 0;
		}
		return false;
	}
	pub fn matchupb (&self, id : u8, value : String) -> bool {
		return self.id == id && self.value == value;
	}
	pub fn matchup (&self, id : u8, value : &str) -> bool {
		return self.id == id && self.value == value;
	}
}

// dict methods
impl Token {
	pub fn hasd (&self, key : &str) -> Result<bool, String> {
		if self.tt != DICT_TOKEN {
			return Err("invalid operation".to_owned());
		}
		return Ok(self.dict.as_ref().unwrap().contains_key(key));
	}
	pub fn _check_vkey_dict (&self, key : &str) -> Result<(), String> {
		if self.tt != DICT_TOKEN {
			return Err(String::from("invalid operation"));
		}
		if !self.dict.as_ref().unwrap().contains_key(key) {
			return Err(String::from("invalid key ") + key);
		}
		return Ok(());
	}
	pub fn getd (&self, key : String) -> Result<Token, String> {
		if self.tt != DICT_TOKEN {
			return Err("invalid operation".to_owned());
		}
		match self._check_vkey_dict(&key) {Ok(_)=>{},Err(e)=>{return Err(e)}};
		return Ok(self.dict.as_ref().unwrap().get(&key).unwrap().clone());
	}
	pub fn setd (&mut self, key : String, value : Token) -> Result<(), String> {
		if self.tt != DICT_TOKEN {
			return Err("invalid operation".to_owned());
		}
		self.dict.as_mut().unwrap().insert(key, value);
		return Ok(());
	}
	pub fn popd (&mut self, key : String) -> Result<Token, String> {
		if self.tt != DICT_TOKEN {
			return Err("invalid operation".to_owned());
		}
		match self._check_vkey_dict(&key) {Ok(_)=>{},Err(e)=>{return Err(e)}};
		return Ok(self.dict.as_mut().unwrap().remove(&key).unwrap());
	}
}

// list methods
impl Token {
	pub fn get (&self, key : usize) -> Result<Token, String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		if self.length <= key {
			return Err("index out of range".to_owned());
		}
		return Ok(self.list.as_ref().unwrap()[key].clone());
	}
	pub fn set (&mut self, key : usize, value : Token) -> Result<(), String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		if self.length <= key {
			return Err("index out of range".to_owned());
		}
		self.list.as_mut().unwrap()[key] = value;
		return Ok(());
	}
	pub fn push (&mut self, v : Token) -> Result<(), String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		self.length += 1;
		self.list.as_mut().unwrap().push(v);
		return Ok(());
	}
	pub fn pop (&mut self) -> Result<Token, String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		if self.length == 0 {
			return Err("can't remove from empty list".to_owned());
		}
		self.length -= 1;
		return Ok(self.list.as_mut().unwrap().pop().unwrap());
	}
	pub fn popitem (&mut self, key : usize) -> Result<Token, String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		if self.length <= key {
			return Err("index out of range".to_owned());
		}
		self.length -= 1;
		return Ok(self.list.as_mut().unwrap().remove(key));
	}
	pub fn extend (&mut self, v : Vec<Token>) -> Result<(), String> {
		if self.tt != LIST_TOKEN {
			return Err("invalid operation".to_owned());
		}
		let l = self.list.as_mut().unwrap();
		for token in v {
			self.length += 1;
			l.push(token);
		}
		return Ok(());
	}
}

impl std::clone::Clone for Token {
	fn clone (&self) -> Token {
		Token {
			id : self.id,
			cid : self.cid,
			value : self.value.to_string(),
			dict : self.dict.clone(),
			list : self.list.clone(),
			length : self.length,
			data_type : self.data_type,
			escape : false,
			line : self.line,
			chara : self.chara,
			tt : self.tt,
		}
	}
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
		.field("id", &self.id)
		.field("value", &self.value.replace("\x1b", "\\x1b").replace("\n", "\\n").replace("\t", "\\t"))
		.finish()
    }
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, \"{}\")", TOKEN_ARRAY[self.id as usize], &self.value.replace("\x1b", "\\x1b").replace("\n", "\\n").replace("\t", "\\t"))
	}
}

impl std::cmp::PartialEq for Token {
	fn eq (&self, other: &Token) -> bool {
		return other.id == self.id && other.value == self.value && other.data_type == self.data_type;
	}
}