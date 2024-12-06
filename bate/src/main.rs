use core::Core;

use env_logger;
use clap::Parser;

pub mod io;
pub mod gate;
pub mod core;
pub mod models;

fn setup() -> () {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn load_argument() -> models::arguments::Arguments {
    let arguments: models::arguments::Arguments = models::arguments::Arguments::parse();

    arguments
}

fn read_source_file(path: String) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    if io::file::File::exists(&path) == true {
        result = io::file::File::open(&path).unwrap();
    }

    result
}

fn main() -> () {
    setup();
    
    let arguments: models::arguments::Arguments = load_argument();
    let mut core: Core = Core::new(arguments.minimum, arguments.maximum, arguments.score);
    let content: Vec<u8> = read_source_file(arguments.file);

    core.run(content);
}
