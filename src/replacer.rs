use regex::Regex;
use lazy_static::lazy_static;

pub struct Replacer {
	pub BACKSLASH : Regex,
}

impl Replacer {
	pub fn new () -> Replacer {
		Replacer {
			BACKSLASH : Regex::new(r"\\").unwrap(),
		}
	}
	pub fn replace (&self, p : &str, n : Regex, mut s : String, r : &str) -> String {
		let plen = p.len();
		let mut f : String = String::new();
		// optimization because it is known that "f" will generally be around the same length as "s"
		f.reserve(s.capacity());
		loop {
			let i = match s.find(p) {
				Some(v)=>v,
				None=>{f+=&s;break;}
			};
			if i == 0 {
				s = String::from(&s[i+plen..]);
			} else {
				if n.is_match(&s[0..i]) && n.find(&s[0..i]).unwrap().1 == s[0..i].len() {
					f += &s[0..i+plen];
				} else {
					f += &s[0..i];
					f += r;
				}
				s = String::from(&s[i+plen..]);
			}
		}
		return f;
	}
}