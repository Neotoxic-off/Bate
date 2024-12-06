use indicatif::ProgressBar;

pub use crate::gate;

pub struct Core
{
    pub minimum: u64,
    pub maximum: u64,
    pub score: f64,
    pub size: u64,
    pub value: Vec<u8>,
    pub key: Vec<u8>,
    pub current_keys: Vec<Vec<u8>>,
    pub keys: Vec<Vec<u8>>,
    
    progress: ProgressBar
}

impl Core {
    pub fn new(minimum: u64, maximum: u64, score: f64) -> Core {
        Core {
            minimum,
            maximum,
            score,
            size: 0,
            value: Vec::new(),
            key: Vec::new(),
            keys: Vec::new(),
            progress: ProgressBar::new(maximum - minimum)
        }
    }

    pub fn run(&mut self, content: Vec<u8>) -> () {
        for key_size in self.minimum..self.maximum {
            self.build_key(key_size);
            self.progress.inc(1);
            self.value = self.process(gate::xor);
            self.check();
        }

        self.progress.finish();
    }

    fn build_key(&mut self, size: u64) -> Vec<u8> {
        self.key.clear();

        for i in 0..size {
            self.key.push();
        }
    }

    fn process(&mut self, operation: fn(u8, u8) -> u8) -> Vec<u8> {
        let mut processed: Vec<u8> = Vec::new();

        for (item) in self.value.iter() {
            processed.push(operation());
        }

        processed
    }

    fn resume(&self) -> () {

    }

    fn check(&self) -> f64 {
        0.0
    }
}
