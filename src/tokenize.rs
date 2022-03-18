use regex::Regex;
use lazy_static::lazy_static;

use crate::statics::*;
use crate::static_colors::*;

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
		if !line[i].is_digit(base) && (line[i] != '.' || (line[i] == '.' && !line[i+1].is_digit(10)) || typedef || decu) && (typedef || decu || !match line[i] {'x'=>true,'b'=>true,_=>false}) {
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

fn get_meta (mut i : usize, line_str : String) -> (usize, String, String) {
	i += 2;
	let mut meta : String = String::new();
	let mut value : String = String::new();
	let line : Vec<char> = line_str.chars().collect();
	let line_len : usize = line.len();
	loop {
		if i >= line_len {
			break;
		}
		if line[i] == '=' {
			break;
		}
		meta.push(line[i].clone());
		i += 1;
	}
	loop {
		if i >= line_len {
			break;
		}
		if line[i] == '=' {
			i += 1;
			continue;
		}
		if line[i] == ']' {
			break;
		}
		value.push(line[i].clone());
		i += 1;
	}
	return (i+1, meta, value);
}

fn process_grp (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	if tokens[i].value == "${" {
		let mut lst : Vec<Token> = Vec::new();
		tokens.remove(i);
		loop {
			if i >= tokens.len() {
				panic!("GRP finding out of bounds");
			}
			if tokens[i].value == "}" {
				tokens.remove(i);
				break;
			} else {
				lst.push(tokens.remove(i));
			}
		}
		let mut f : Token = Token::news(DCT, "DCT", DICT_TOKEN);
		f.meta(tokens[i].line, tokens[i].chara);
		let mut c : usize = 0;
		let l = lst.len();
		loop {
			if c >= l {
				break;
			}
			if lst[c].id == SEP {
				f.setd(lst[c-1].value.clone(), lst[c+1].clone());
				c += 1;
			}
			c += 1;
		}
		return (i, tokens, f);
	} else if tokens[i].value == "$[" {
		let linev = tokens[i].line;
		let charav = tokens[i].chara;
		tokens.remove(i);
		let mut lst : Vec<Token> = Vec::new();
		let mut depth : usize = 1;
		loop {
			if i >= tokens.len() {
				panic!("GRP finding out of bounds");
			}
			if tokens[i].id == GRP {
				if tokens[i].value == "]" {
					depth -= 1;
					if depth == 0 {
						tokens.remove(i);
						break;
					}
				} else if tokens[i].value == "$[" {
					depth += 1;
				}
				lst.push(tokens.remove(i));
			} else {
				lst.push(tokens.remove(i));
			}
		}
		lst = preprocess(lst);
		print!("{}", INTERPRETER_DEBUG_ORANGE);
		printlst(&lst);
		print!("\x1b[0m");
		let mut f : Token = Token::news(LST, "LST", LIST_TOKEN);
		f.meta(linev, charav);
		for t in lst {
			if t.id != SEP {
				f.push(t.clone());
			}
		}
		return (i, tokens, f);
	} else {
		let f = tokens[i].clone();
		return (i+1, tokens, f);
	}
}

fn process_fun (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	let mut f : Token = Token::new(FUN, tokens[i+1].value.clone(), LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 0;
	tokens.remove(i);
	tokens.remove(i);
	tokens.remove(i);
	loop {
		if i >= tokens.len() {
			panic!("function def end not found");
		}
		if tokens[i].id == PAR && tokens[i].value == ")" {
			tokens.remove(i);
			f.push(Token::news(SEP, "*", BASE_TOKEN));
			break;
		}
		f.push(tokens.remove(i));
	}
	loop {
		if i >= tokens.len() {
			panic!("function end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			} else if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list.as_mut().unwrap().extend(preprocess(ft));
	// println!("{}", f.value);
	// printlst::<Token>(&f.list.as_ref().unwrap());
	return (i, tokens, f);
}

fn process_idx (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	// println!("{}PREPROCESS IDX\x1b[0m", INTERPRETER_DEBUG_BRIGHTPINK);
	// printlst::<Token>(&tokens);
	let mut f : Token = Token::news(IDX, "IDX", LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 0;
	loop {
		if i >= tokens.len() {
			panic!("index end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "[" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			} else if tokens[i].value == "]" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list = Some(preprocess(ft));
	return (i, tokens, f);
}

fn process_forloop (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	let mut f : Token = Token::news(CTL, "forloop", LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 1;
	tokens.remove(i);
	loop {
		if i >= tokens.len() {
			panic!("loop def end not found");
		}
		if tokens[i].id == GRP && tokens[i].value == "{" {
			tokens.remove(i);
			f.push(Token::news(SEP, "*", BASE_TOKEN));
			break;
		}
		if tokens[i].id == KEY && tokens[i].value == "in" {
			tokens.remove(i);
			f.push(Token::news(SEP, "*", BASE_TOKEN));
			continue;
		}
		if tokens[i].id == DOT && tokens[i+1].id == DOT {
			tokens.remove(i);
			tokens.remove(i);
			f.push(Token::news(SEP, "..", BASE_TOKEN));
			continue;
		}
		f.push(tokens.remove(i));
	}
	loop {
		if i >= tokens.len() {
			panic!("loop end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			} else if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list.as_mut().unwrap().extend(preprocess(ft));
	return (i-1, tokens, f);
}

fn process_whileloop (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	let mut f : Token = Token::news(CTL, "whileloop", LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 1;
	tokens.remove(i);
	loop {
		if i >= tokens.len() {
			panic!("loop def end not found");
		}
		if tokens[i].id == GRP && tokens[i].value == "{" {
			tokens.remove(i);
			f.push(Token::news(SEP, "*", BASE_TOKEN));
			break;
		}
		f.push(tokens.remove(i));
	}
	loop {
		if i >= tokens.len() {
			panic!("loop end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			} else if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list.as_mut().unwrap().extend(preprocess(ft));
	return (i-1, tokens, f);
}

fn process_ifblock (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	let mut f : Token = Token::news(CTL, "ifblock", LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 1;
	tokens.remove(i);
	loop {
		if i >= tokens.len() {
			panic!("if def end not found");
		}
		if tokens[i].id == GRP && tokens[i].value == "{" {
			tokens.remove(i);
			f.push(Token::news(SEP, "*", BASE_TOKEN));
			break;
		}
		f.push(tokens.remove(i));
	}
	loop {
		if i >= tokens.len() {
			panic!("if end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			} else if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list.as_mut().unwrap().extend(preprocess(ft));
	return (match i>0{true=>i-1,_=>i}, tokens, f);
}

fn process_elseblock (i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	let mut f : Token = Token::news(CTL, "elseblock", LIST_TOKEN);
	f.meta(tokens[i].line, tokens[i].chara);
	let mut ft : Vec<Token> = Vec::new();
	let mut depth : u32 = 0;
	tokens.remove(i);
	loop {
		if i >= tokens.len() {
			panic!("else end not found");
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					tokens.remove(i);
					break;
				}
			} else if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			}
		}
		ft.push(tokens.remove(i));
	}
	f.list.as_mut().unwrap().extend(preprocess(ft));
	return (i-1, tokens, f);
}

fn process_class (mut i : usize, mut tokens : Vec<Token>) -> (usize, Vec<Token>, Token) {
	i += 1;
	// pop class keyword
	// tokens.remove(i);
	// generate object token
	let mut f = Token::new(OBJ, tokens.remove(i).value, DICT_TOKEN);
	let mut inheritance = Token::news(LST, "\"class inheritance", LIST_TOKEN);
	// checks if the class is inheriting from anything
	if tokens[i].matchup(KEY, "inheriting") {
		tokens.remove(i);
		loop {
			if i >= tokens.len() {
				panic!("class declaration fault");
			}
			if tokens[i].matchup(GRP, "{") {
				break;
			}
			if tokens[i].id == SEP {
				tokens.remove(i);
			}
			inheritance.push(tokens.remove(i));
		}
	}
	// set inheritance property
	f.setd(String::from("\"class inheritance"), inheritance);
	let mut depth : usize = 0;
	// all tokens within class body
	let mut lst : Vec<Token> = Vec::new();
	loop {
		if i >= tokens.len() {
			break;
		}
		if tokens[i].id == GRP {
			if tokens[i].value == "{" {
				depth += 1;
				if depth == 1 {
					tokens.remove(i);
					continue;
				}
			} else if tokens[i].value == "}" {
				depth -= 1;
				if depth == 0 {
					break;
				}
			}
		}
		lst.push(tokens.remove(i));
	}
	let mut ind : usize = 0;
	let mut l = lst.len();
	let mut props : Token = Token::news(LST, "props", LIST_TOKEN);
	loop {
		if ind >= l {
			break;
		}
		if lst[ind].matchup(KEY, "method") {
			let x : (usize, Vec<Token>, Token) = process_fun(ind, lst);
			ind = x.0;
			lst = x.1;
			l = lst.len();
			lst.insert(ind, x.2);
		}
		if lst[ind].matchup(KEY, "property") {
			lst.remove(ind);
			let mut ls : Token = Token::news(LST, &lst[ind].value, LIST_TOKEN);
			lst.remove(ind);
			lst.remove(ind);
			loop {
				if ind >= lst.len() {
					panic!("class body property fault");
				}
				if lst[ind].id == NLN {
					break;
				}
				ls.push(lst.remove(ind));
			}
			props.push(ls);
			l = lst.len();
		}
		ind += 1;
	}
	ind = 0;
	l = lst.len();
	f.setd(String::from("\"props"), props);
	loop {
		if ind >= l {
			break;
		}
		if lst[ind].id == FUN {
			f.setd(lst[ind].value.clone(), lst.remove(ind));
			l = lst.len();
			continue;
		}
		ind += 1;
	}
	return (i, tokens, f);
}

pub fn preprocess (mut tokens : Vec<Token>) -> Vec<Token> {
	// println!("{}PREPROCESS\x1b[0m", INTERPRETER_DEBUG_BRIGHTPINK);
	// printlst::<Token>(&tokens);
	let mut fv : Vec<Token> = Vec::new();
	let mut i : usize = 0;
	let mut l = tokens.len();
	loop {
		if i >= l {
			break;
		}
		if tokens[i].id == GRP {
			if tokens[i].value.chars().nth(0).unwrap() == '$' {
				let x : (usize, Vec<Token>, Token) = process_grp(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			} else {
				if tokens[i].value == "[" {
					// println!("TOKENI");
					// println!("{}, {}", tokens[i], i);
					let x : (usize, Vec<Token>, Token) = process_idx(i, tokens);
					// let oi = i;
					i = x.0;
					tokens = x.1;
					// println!("LXLST");
					// printlst::<Token>(&tokens);
					// printlst::<Token>(x.2.list.as_ref().unwrap());
					l = tokens.len();
					fv.push(x.2);
					continue;
				}
			}
		} else if tokens[i].id == KEY {
			if tokens[i].value == "func" {
				let x : (usize, Vec<Token>, Token) = process_fun(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			}
			if tokens[i].value == "for" {
				let x : (usize, Vec<Token>, Token) = process_forloop(i, tokens);
				i = x.0;
				tokens = x.1;
				// println!("AFTER FOR LOOP");
				// printlst(&tokens);
				l = tokens.len();
				fv.push(x.2);
				// printlst(&fv);
				continue;
			}
			if tokens[i].value == "while" {
				let x : (usize, Vec<Token>, Token) = process_whileloop(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			}
			if tokens[i].value == "if" {
				let x : (usize, Vec<Token>, Token) = process_ifblock(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			}
			if tokens[i].value == "else" {
				let x : (usize, Vec<Token>, Token) = process_elseblock(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			}
			if tokens[i].value == "class" {
				fv.push(tokens[i].clone());
				let x : (usize, Vec<Token>, Token) = process_class(i, tokens);
				i = x.0;
				tokens = x.1;
				l = tokens.len();
				fv.push(x.2);
				continue;
			}
			fv.push(tokens[i].clone());
		} else {
			fv.push(tokens[i].clone());
		}
		i += 1;
	}
	// println!("FINAL PRINT");
	// printlst(&fv);
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
		// logical operatins
		static ref LOGIC_RE : Regex = Regex::new(LOGIC_RE_PAT).unwrap();
	}
	let mut line_index = 0;
	let lines_len_total = lines.len();
	let mut words : Vec<String> = Vec::new();
	let mut meta_data : Vec<[usize; 2]> = Vec::new();
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
			if line[i] == ' ' || line[i] == '\t' {
				i += 1;
				continue;
			}
			if line_len > i+1 && line[i] == '/' && line[i+1] == '/' {
				break;
			}
			if line[i] == '.' {
				meta_data.push([line_index, i]);
				words.push(String::from("."));
			} else if i == 0 && line[i] == '#' {
				let x : (usize, String, String) = get_meta(i, lines[line_index].to_string());
				meta_data.push([line_index, i]);
				words.push(String::from("#") + &x.1);
				meta_data.push([line_index, i]);
				words.push(x.2);
				i = x.0;
		 	} else if line[i] == ';' {
				meta_data.push([line_index, i]);
				words.push(String::from(";"));
			} else if line[i] == '"' {
				let x : (usize, String) = get_containing(i, line[i].clone(), get_complement_surround(line[i].clone()), lines[line_index].to_string());
				i = x.0;
				meta_data.push([line_index, i]);
				words.push(x.1);
			} else if line[i].is_digit(10) {
				let x : (usize, String) = get_number(i, lines[line_index].to_string());
				i = x.0 - 1;
				meta_data.push([line_index, i]);
				words.push(x.1);
			} else if line[i].is_alphabetic() || line[i] == '_' {
				let x : (usize, String) = get_word(i, lines[line_index].to_string());
				i = x.0;
				meta_data.push([line_index, i]);
				words.push(x.1);
			} else if line[i] == '+' || line[i] == '-' || line[i] == '*' || line[i] == '/' {
				let mut v = line[i].to_string();
				if line[i+1] == '=' {
					v += &line[i+1].to_string();
				}
				meta_data.push([line_index, i]);
				words.push(v);
				i += 1;
			} else if line[i] == '$' {
				if GROUP_RE.is_match(&line[i+1].to_string()) {
					meta_data.push([line_index, i]);
					words.push(line[i].to_string() + &line[i+1].to_string());
					i += 1;
				} else {
					let x : (usize, String) = get_word(i+1, lines[line_index].to_string());
					i = x.0;
					meta_data.push([line_index, i]);
					words.push(String::from("$")+&x.1);
				}
			} else if line[i] == '&' || line[i] == '|' || line[i] == '!' || line[i] == '^' || line[i] == '%' {
				meta_data.push([line_index, i]);
				if line[i] == '&' || line[i] == '|' {
					if line[i+1] == line[i] {
						i += 1;
						words.push(line[i].to_string().repeat(2));
						i += 1;
						continue;
					}
				}
				words.push(line[i].to_string());
			} else {
				meta_data.push([line_index, i]);
				words.push(line[i].to_string());
			}
			i += 1;
		}
		line_index += 1;
	}
	let mut tokens : Vec<Token> = Vec::new();
	let mut c : usize = 0;
	// printlst(&words);
	for word in words {
		if word.chars().nth(0).unwrap() == '.' {
			tokens.push(Token::new(DOT, word, BASE_TOKEN));
		} else if word.chars().nth(0).unwrap() == '#' {
			tokens.push(Token::news(MET, &word[1..], BASE_TOKEN));
		} else if word == ";" {
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
		} else if LOGIC_RE.is_match(&word) {
			tokens.push(Token::new(LOG, word, BASE_TOKEN));
		} else {
			tokens.push(Token::new(UDF, word, BASE_TOKEN));
		}
		let ind = tokens.len()-1;
		tokens[ind].meta(meta_data[c][0]+1, meta_data[c][1]+1);
		c += 1;
	}
	return preprocess(tokens);
}