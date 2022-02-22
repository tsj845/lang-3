extern crate regex;

use regex::Regex;
use lazy_static::lazy_static;

use crate::statics::*;
use crate::printv;

fn get_complement_surround (given : char) -> char {
	match given {
		'{' => '}',
		'}' => '{',
		'"' => '"',
		'(' => ')',
		')' => '(',
		'[' => ']',
		']' => '[',
		_ => {panic!("unrecognized surrounding")}
	}
}

fn get_containing (mut i : usize, opening : char, closing : char, line_str : String) -> (usize, String) {
	let line : Vec<char> = line_str.chars().collect();
	i += 1;
	let mut word : String = String::new();
	word += &opening.to_string();
	let line_len : usize= line.len();
	loop {
		if i >= line_len {
			panic!("somthing isn't closed");
		}
		word += &line[i].to_string();
		if line[i] == closing && line[i-1] != '\\' {
			break;
		}
		i += 1;
	}
	return (i, word);
}

fn get_number (mut i : usize, line_str : String) -> (usize, String) {
	let line : Vec<char> = line_str.chars().collect();
	let line_len : usize = line.len();
	let mut decu : bool = false;
	let mut typedef : bool = false;
	let mut base : u32 = 10;
	let mut num : String = String::new();
	loop {
		if i >= line_len {
			break;
		}
		if !line[i].is_digit(base) && (line[i] != '.' || typedef || decu) && (typedef || decu || !match line[i] {'x'=>true,'b'=>true,_=>false}) {
			break;
		}
		if line[i] == '.' {
			decu = true;
		}
		if match line[i] {'x'=>true,'b'=>true,_=>false} {
			typedef = true;
			base = match line[i] {'x'=>16,'b'=>2,_=>{panic!("")}};
		}
		num += &line[i].to_string();
		i += 1;
	}
	return (i, num);
}

fn get_word (mut i : usize, line_str : String) -> (usize, String) {
	lazy_static! {
		static ref WORD_RE : Regex = Regex::new("[[:word:]]").unwrap();
	}
	let line : Vec<char> = line_str.chars().collect();
	let line_len : usize = line.len();
	let mut word : String = String::new();
	loop {
		if i >= line_len {
			break;
		}
		let c = &line[i].to_string();
		if WORD_RE.is_match(&c) {
			word += c;
		} else {
			i -= 1;
			break;
		}
		i += 1;
	}
	return (i, word);
}

fn process_grp (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	if tokens[i].value == "{" || tokens[i].value == "}" {
		let x = tokens[i].clone();
		return (i+1, tokens, x);
	} else if tokens[i].value == "[" {
		let mut lst : Vec<Token> = Vec::new();
		loop {
			if i >= tokens.len() {
				panic!("GRP finding out of bounds");
			}
			if tokens[i].value == "]" {
				tokens.remove(i);
				break;
			} else {
				lst.push(tokens.remove(i));
			}
		}
		let mut f : Token = Token::new(LST, "LST".to_string(), LIST_TOKEN);
		for t in lst {
			if t.id != SEP {
				f.push(t.clone());
			}
		}
		return (i, tokens, f);
	} else {
		panic!("unkown GRP token");
	}
}

pub fn preprocess (mut tokens : Vec<Token>) -> Vec<Token> {
	let mut fv : Vec<Token> = Vec::new();
	let mut i : usize = 0;
	let mut l = tokens.len();
	loop {
		if i >= l {
			break;
		}
		if tokens[i].id == GRP {
			let x : (usize, Vec<Token>, Token) = process_grp(i, tokens);
			i = x.0;
			tokens = x.1;
			l = tokens.len();
			// printv!(&x.2);
			fv.push(x.2);
			continue;
		} else {
			fv.push(tokens[i].clone());
		}
		i += 1;
	}
	printlst::<Token>(&fv);
	return fv;
}

pub fn tokenize (lines : Vec<&str>) -> Vec<Token> {
	// regexp
	lazy_static! {
		// generic words
		static ref WORD_RE : Regex = Regex::new(WORD_RE_PAT).unwrap();
		// things that contain other things
		static ref CONTAINER_RE : Regex = Regex::new(CONTAINER_RE_PAT).unwrap();
		// numbers
		static ref NUMBER_RE : Regex = Regex::new(NUMBER_RE_PAT).unwrap();
		// literals
		static ref LITERAL_RE : Regex = Regex::new(LITERAL_RE_PAT).unwrap();
		// parentheses
		static ref PAREN_RE : Regex = Regex::new(PAREN_RE_PAT).unwrap();
		// groups
		static ref GROUP_RE : Regex = Regex::new(GROUP_RE_PAT).unwrap();
		// seperators
		static ref SEPER_RE : Regex = Regex::new(SEPER_RE_PAT).unwrap();
		// keywords
		static ref KEYWD_RE : Regex = Regex::new(KEYWD_RE_PAT).unwrap();
		// assignments
		static ref ASIGN_RE : Regex = Regex::new(ASIGN_RE_PAT).unwrap();
		// mathmatics
		static ref MATHM_RE : Regex = Regex::new(MATHM_RE_PAT).unwrap();
	}
	let mut line_index = 0;
	let lines_len_total = lines.len();
	let mut words : Vec<String> = Vec::new();
	'outer : loop {
		if line_index >= lines_len_total {
			break 'outer;
		}
		let line : Vec<char> = lines[line_index].chars().collect();
		let mut i : usize = 0usize;
		let line_len = line.len();
		'inner : loop {
			if i >= line_len {
				break 'inner;
			}
			if line[i] == ' ' {
				i += 1;
				continue;
			}
			if line[i] == ';' {
				words.push(String::from(";"));
			} else if line[i] == '"' {
				let x : (usize, String) = get_containing(i, line[i].clone(), get_complement_surround(line[i].clone()), lines[line_index].to_string());
				i = x.0;
				words.push(x.1);
			} else if line[i].is_digit(10) {
				let x : (usize, String) = get_number(i, lines[line_index].to_string());
				i = x.0 - 1;
				words.push(x.1);
			} else if line[i].is_alphabetic() {
				let x : (usize, String) = get_word(i, lines[line_index].to_string());
				i = x.0;
				words.push(x.1);
			} else if line[i] == '+' || line[i] == '-' || line[i] == '*' || line[i] == '/' {
				let mut v = line[i].to_string();
				if line[i+1] == '=' {
					v += &line[i+1].to_string();
				}
				words.push(v);
				i += 1;
			} else {
				words.push(line[i].to_string());
			}
			i += 1;
		}
		line_index += 1;
	}
	let mut tokens : Vec<Token> = Vec::new();
	for word in words {
		if word == ";" {
			tokens.push(Token::new(NLN, word, BASE_TOKEN));
		} else if word.starts_with('"') || NUMBER_RE.is_match(&word) || LITERAL_RE.is_match(&word) {
			tokens.push(Token::new(LIT, word, BASE_TOKEN));
		} else if KEYWD_RE.is_match(&word) {
			tokens.push(Token::new(KEY, word, BASE_TOKEN));
		} else if WORD_RE.is_match(&word) {
			tokens.push(Token::new(REF, word, BASE_TOKEN));
		} else if PAREN_RE.is_match(&word) {
			tokens.push(Token::new(PAR, word, BASE_TOKEN));
		} else if GROUP_RE.is_match(&word) {
			tokens.push(Token::new(GRP, word, BASE_TOKEN));
		} else if SEPER_RE.is_match(&word) {
			tokens.push(Token::new(SEP, word, BASE_TOKEN));
		} else if ASIGN_RE.is_match(&word) {
			tokens.push(Token::new(ASS, word, BASE_TOKEN));
		} else if MATHM_RE.is_match(&word) {
			tokens.push(Token::new(MAT, word, BASE_TOKEN));
		} else {
			tokens.push(Token::new(UDF, word, BASE_TOKEN));
		}
	}
	// printlst::<Token>(&tokens);
	return preprocess(tokens);
}