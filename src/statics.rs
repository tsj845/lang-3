use std::fmt;

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

// namespace identifiers
pub const CONSTANT : usize = 0;
pub const GLOBAL : usize = 1;
pub const LOCAL : usize = 2;

// token types
pub const NUL : u8 = 0;
pub const FUN : u8 = 1;
pub const REF : u8 = 2;
pub const LIT : u8 = 3;
pub const KEY : u8 = 4;
pub const MAT : u8 = 5;
pub const LOG : u8 = 6;
pub const ASS : u8 = 7;
pub const EOF : u8 = 8;
pub const ERR : u8 = 9;
pub const TST : u8 = 10;
pub const TOKEN_ARRAY : [&str; 11] = ["NUL", "FUN", "REF", "LIT", "KEY", "MAT", "LOG", "ASS", "EOF", "ERR", "TST"];
pub const FILE_EXT : &str = ".fpp";

// constant variables
pub const CONST_VARS : [[&str; 3]; 4] = [["null", "NULL", "null"], ["true", "bool", "true"], ["false", "bool", "false"], ["version", "str", "ALPH_0"]];

pub struct Token {
	id : u8,
	value : String,
}

impl Token {
	pub fn new (id : u8, value : String) -> Token {
		Token {
			id : id,
			value : value,
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
		write!(f, "Token {{{}, \"{}\"}}", TOKEN_ARRAY[self.id as usize], self.value)
	}
}