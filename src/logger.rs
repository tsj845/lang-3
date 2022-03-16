// use crate::static_colors::*;


pub struct Logger {
    lines : Vec<String>,
}

impl Logger {
    pub fn new () -> Logger {
        Logger {
            lines : Vec::new(),
        }
    }
    pub fn log (&mut self, item : String) {
        self.lines.push(item);
    }
    pub fn dump (&self) {
        let mut counter : usize = 0;
        for item in &self.lines {
            counter += 1;
            println!("ITEM {}: {}", counter, item);
        }
    }
}