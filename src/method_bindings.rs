use crate::statics::*;
use std::collections::HashMap;

// this module provides bindings from types to rust code

// method bindings & list of system objects
pub struct Bindings<'a> {
    lists : HashMap<&'a str, &'a str>,
    dicts : HashMap<&'a str, &'a str>,
    strings : HashMap<&'a str, &'a str>,
    numbers : HashMap<&'a str, &'a str>,
	objects : HashMap<&'a str, &'a str>,
}

impl Bindings<'_> {
    pub fn new () -> Bindings<'static> {
        let lists : [[&str; 2]; 4] = [["method", "push"], ["method", "pop"], ["method", "remove"], ["property", "length"]];
        let dicts : [[&str; 2]; 0] = [];
        let strings : [[&str; 2]; 3] = [["property", "length"], ["method", "is_alpha"], ["method", "is_digit"]];
        let numbers : [[&str; 2]; 1] = [["method", "to_string"]];
		let objects : [[&str; 2]; 1] = [["SYSTEM", "System"]];
        // maps from slices to hashmaps
        let mut ld : HashMap<&str, &str> = HashMap::new();
        let mut dd : HashMap<&str, &str> = HashMap::new();
        let mut sd : HashMap<&str, &str> = HashMap::new();
        let mut nd : HashMap<&str, &str> = HashMap::new();
		let mut od : HashMap<&str, &str> = HashMap::new();
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
		for v in objects.iter() {
			od.insert(v[1], v[0]);
		}
        Bindings {
            lists : ld,
            dicts : dd,
            strings : sd,
            numbers : nd,
			objects : od,
        }
    }
    pub fn get_type (&self, i : u8, t : &str) -> &&str {
		if i == 0 {
			return self.strings.get(t).unwrap();
		} else if i == 1 {
			return self.lists.get(t).unwrap();
		} else if i == 2 {
			return self.dicts.get(t).unwrap();
		} else if i == 3 {
			return self.numbers.get(t).unwrap();
		}
		return &"";
	}
    // checks that the call is valid
    pub fn check_valid (&self, t : &Token, target : &str) -> bool {
        // println!("{}, {}", t.data_type, target);
        if t.data_type == DT_BOL || t.data_type == DT_UDF {
            return false;
        }
        if t.data_type == DT_STR {
            return self.strings.contains_key(target);
        }
        if t.data_type == DT_NUM {
            return self.numbers.contains_key(target);
        }
        if t.data_type == DT_LST {
            return self.lists.contains_key(target);
        }
        if t.data_type == DT_DCT {
            return self.dicts.contains_key(target);
        }
        return false;
    }
	// checks if something is a system object
	pub fn check_object (&self, t : &Token) -> bool {
		return self.objects.contains_key(&t.value[..]);
	}
}

// stores data on individual system objects
struct SystemObject {
	main_dict : HashMap<String, Vec<String>>,
	static_props : HashMap<String, Token>,
}

impl SystemObject {
	fn new (dict : HashMap<String, Vec<String>>, statics : HashMap<String, Token>) -> SystemObject {
		SystemObject {
			main_dict : dict,
			static_props : statics,
		}
	}
}

// actual bindings involving system objects
pub struct SystemObjects {
	objects : Vec<SystemObject>,
}

impl SystemObjects {
	pub fn new () -> SystemObjects {
		SystemObjects {
			objects : Vec::new(),
		}
	}
}