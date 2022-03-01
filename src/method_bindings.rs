use crate::statics::*;
use std::collections::HashMap;

// this module provides bindings from types to rust code

pub struct Bindings<'a> {
    lists : HashMap<&'a str, &'a str>,
    dicts : HashMap<&'a str, &'a str>,
    strings : HashMap<&'a str, &'a str>,
    numbers : HashMap<&'a str, &'a str>,
}

impl Bindings<'_> {
    pub fn new () -> Bindings<'static> {
        let lists : [[&str; 2]; 3] = [["method", "push"], ["method", "pop"], ["property", "length"]];
        let dicts : [[&str; 2]; 0] = [];
        let strings : [[&str; 2]; 1] = [["property", "length"]];
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
        println!("{}, {}", t.data_type, target);
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
}