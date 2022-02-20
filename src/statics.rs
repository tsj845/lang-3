use std::fmt;
use std::collections::HashMap;

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
pub const EOF : u8 = 14;
pub const ERR : u8 = 15;
pub const UDF : u8 = 16;
pub const TST : u8 = 17;
pub const TOKEN_ARRAY : [&str; 18] = ["NUL", "FUN", "REF", "LIT", "KEY", "MAT", "LOG", "ASS", "PAR", "LST", "DCT", "SEP", "SYM", "GRP", "EOF", "ERR", "UDF", "TST"];
pub const FILE_EXT : &str = ".fpp";

// program keywords
pub const KEYWORDS : [&str; 2] = ["gloabl", "local"];

// tokenization regex patterns
pub const WORD_RE_PAT : &str = r"[[:alpha:]]+[[:word:]]*";
pub const CONTAINER_RE_PAT : &str = r#"[{\["(]"#;
pub const NUMBER_RE_PAT : &str = r"0b[01]+|0x[0-9a-f]+|[0-9]+(\.[0-9]{1,})?";
pub const LITERAL_RE_PAT : &str = r"true|false|null";
pub const PAREN_RE_PAT : &str = r"[()]";
pub const GROUP_RE_PAT : &str = r"[{}\[\]]";
pub const SEPER_RE_PAT : &str = r"[:,]";
pub const KEYWD_RE_PAT : &str = r"global|local";

pub struct Token {
	pub id : u8,
	pub value : String,
	pub dict : Option<HashMap<String, Token>>,
	pub list : Option<Vec<Token>>,
	pub length : usize,
}

impl Token {
	pub fn new (id : u8, value : String, tt : u8) -> Token {
		if tt == BASE_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : None,
				list : None,
				length : 0,
			};
		} else if tt == DICT_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : Some(HashMap::new()),
				list : None,
				length : 0,
			};
		} else if tt == LIST_TOKEN {
			return Token {
				id : id,
				value : value,
				dict : None,
				list : Some(Vec::new()),
				length : 0,
			};
		}
		panic!("invalid optimization type");
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
		return other.id == self.id && other.value == self.value;
	}
}