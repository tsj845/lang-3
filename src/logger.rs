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
    pub fn logs (&mut self, item : &str) {
        self.lines.push(item.to_owned());
    }
    pub fn dump (&self) {
        println!("\n\x1b[38;2;255;255;0mLOG DUMP\x1b[0m");
        let mut counter : usize = 0;
        for item in &self.lines {
            counter += 1;
            println!("ITEM {}: {}", counter, item);
        }
    }
}