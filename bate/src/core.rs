use std::{collections::HashMap, str};
use log::{info, warn};

pub use crate::gate;

pub struct Core
{
    pub minimum: u64,
    pub maximum: u64,
    pub score: f64,
    pub size: u64,
}

impl Core {
    pub fn new(minimum: u64, maximum: u64, score: f64) -> Core {
        Core {
            minimum,
            maximum,
            score,
            size: 0
        }
    }

    pub fn run(&mut self, content: Vec<u8>, keys: Vec<String>) -> () {
        let mut processed: Vec<u8> = Vec::new();
        let operations: HashMap<&str, fn(u8, u8) -> u8> = HashMap::from([
            ("AND", gate::and as fn(u8, u8) -> u8),
            ("OR", gate::or as fn(u8, u8) -> u8),
            ("XOR", gate::xor as fn(u8, u8) -> u8),
            ("NAND", gate::nand as fn(u8, u8) -> u8),
            ("NOR", gate::nor as fn(u8, u8) -> u8),
            ("NXOR", gate::nxor as fn(u8, u8) -> u8)
        ]);

        for (operation_title, operation) in operations.iter() {
            for key in keys.iter() {
                processed = self.process(operation, &content, &key.as_bytes().to_owned());
                self.check(operation_title, &key, &processed);
            }
        }
    }

    fn process(&mut self, operation: &fn(u8, u8) -> u8, content: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
        let mut processed: Vec<u8> = Vec::new();
        let mut sub_offset: usize = 0;
        let short = key;
        let long = content;
    
        for index in 0..long.len() {
            if sub_offset == short.len() {
                sub_offset = 0;
            }
            processed.push(operation(long[index], short[sub_offset]));
            sub_offset += 1;
        }

        processed
    }

    fn render(&self, operation_title: &&str, key: &String, score: f64) -> () {
        let total_width: usize = 40;
        let dot_count: usize = total_width.saturating_sub(key.len() + operation_title.len() + 5);
        let dots: String = ".".repeat(dot_count);

        info!("{}({}){}: {:.1}%", operation_title, key, dots, score * 100.0);
    }

    fn check(&self, operation_title: &&str, key: &String, decrypted: &Vec<u8>) -> () {
        let ascii_score = decrypted
            .iter()
            .filter(|&&byte| byte.is_ascii_alphanumeric() || byte.is_ascii_whitespace())
            .count() as f64;
        let score: f64 = ascii_score / decrypted.len() as f64;

        if score >= self.score {
            self.render(operation_title, key, score);
        }
    }
}
