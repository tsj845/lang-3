use crate::statics::*;
use std::collections::HashMap;

// this module provides bindings from types to rust code

// method bindings & list of system objects
pub struct Bindings<'a> {
    lists : HashMap<&'a str, &'a str>,
    dicts : HashMap<&'a str, &'a str>,
    strings : HashMap<&'a str, &'a str>,
    numbers : HashMap<&'a str, &'a str>,
}

impl Bindings<'_> {
    pub fn new () -> Bindings<'static> {
        let lists : [[&str; 2]; 5] = [["method", "push"], ["method", "pop"], ["method", "remove"], ["property", "length"], ["method", "get"]];
        let dicts : [[&str; 2]; 0] = [];
        let strings : [[&str; 2]; 3] = [["property", "length"], ["method", "is_alpha"], ["method", "is_digit"]];
        let numbers : [[&str; 2]; 1] = [["method", "to_string"]];
        // maps from slices to hashmaps
        let mut ld : HashMap<&str, &str> = HashMap::new();
        let mut dd : HashMap<&str, &str> = HashMap::new();
        let mut sd : HashMap<&str, &str> = HashMap::new();
        let mut nd : HashMap<&str, &str> = HashMap::new();
        for v in lists.iter() {
            ld.insert(v[1], v[0]);
        }
        for v in dicts.iter() {
            dd.insert(v[1], v[0]);
        }
        for v in strings.iter() {
            sd.insert(v[1], v[0]);
        }
        for v in numbers.iter() {
            nd.insert(v[1], v[0]);
        }
        Bindings {
            lists : ld,
            dicts : dd,
            strings : sd,
            numbers : nd,
        }
    }
    pub fn get_type (&self, i : u8, t : &str) -> &&str {
		if !self.validate(i, t) {
			return &"";
		}
		if i == DT_STR {
			return self.strings.get(t).unwrap();
		} else if i == DT_LST {
			return self.lists.get(t).unwrap();
		} else if i == DT_DCT {
			return self.dicts.get(t).unwrap();
		} else if i == DT_NUM {
			return self.numbers.get(t).unwrap();
		}
		return &"";
	}
    
    fn validate (&self, t : u8, target : &str) -> bool {
        // println!("{}, {}", t, target);
        if t == DT_BOL || t == DT_UDF {
            return false;
        }
        if t == DT_STR {
            return self.strings.contains_key(target);
        }
        if t == DT_NUM {
			// println!("IS DT_NUM");
            return self.numbers.contains_key(target);
        }
        if t == DT_LST {
            return self.lists.contains_key(target);
        }
        if t == DT_DCT {
            return self.dicts.contains_key(target);
        }
        return false;
    }
	// checks that the call is valid
	pub fn check_valid (&self, t : &Token, target : &str) -> bool {
		if t.data_type == DT_OBJ {
			return t.dict.as_ref().unwrap().contains_key(target);
		}
		return self.validate(t.data_type, target);
	}
}