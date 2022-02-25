use crate::statics::*;
use std::collections::HashMap;

// this module provides bindings from types to rust code

pub struct Bindings<'a> {
    lists : HashMap<&'a str, &'a str>,
    dicts : HashMap<&'a str, &'a str>,
    strings : HashMap<&'a str, &'a str>,
    numbers : HashMap<&'a str, &'a str>,
    UDFTOK : Token,
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
        for v in lists {
            ld.insert(v[1], v[0]);
        }
        for v in dicts {
            dd.insert(v[1], v[0]);
        }
        for v in strings {
            sd.insert(v[1], v[0]);
        }
        for v in numbers {
            nd.insert(v[1], v[0]);
        }
        Bindings {
            lists : ld,
            dicts : dd,
            strings : sd,
            numbers : nd,
            UDFTOK : Token::news(UDF, "UDF", BASE_TOKEN),
        }
    }
    fn empty (&self) -> Token {
        return self.UDFTOK.clone();
    }
    fn get_value (&self, tokens : &Vec<Token>, i : usize) -> (usize, Token) {
        if tokens.len() > 0 {
            return (i, tokens[i+1].clone());
        }
        return (i+2, self.empty());
    }
    pub fn execute (&self, mut  tokens : Vec<Token>, mut i : usize) -> (usize, Token, Vec<Token>) {
        let target : &str = &tokens[i+1].value.clone();
        let t = &tokens[i-1];
        if t.data_type == DT_STR {
            let btype : &&str = self.strings.get(target).unwrap();
            if btype == &"method" {
                return (i, self.empty(), tokens);
            } else if btype == &"property" {
                if target == "length" {
                    return (i, Token::new(LIT, (t.value.len()-2).to_string(), BASE_TOKEN), tokens);
                }
            }
        }
        if t.data_type == DT_LST {
            let btype : &&str = self.lists.get(target).unwrap();
            if btype == &"method" {
                if target == "push" {
                    let x : (usize, Token) = self.get_value(&tokens, i);
                    i = x.0;
                    tokens[i-1].push(x.1);
                }
                return (i, self.empty(), tokens);
            } else if btype == &"property" {
                if target == "length" {
                    return (i, Token::new(LIT, t.length.to_string(), BASE_TOKEN), tokens);
                }
            }
        }
        return (i, self.empty(), tokens);
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