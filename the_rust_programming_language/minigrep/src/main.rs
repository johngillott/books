use std::{env, fs};
fn main() {
    /*
    Note that std::env::args will panic if any argument contains
    invalid Unicode. If your program needs to accept arguments
    containing invalid Unicode, use std::env::args_os instead.
    That function returns an iterator that produces OsString values
    instead of String values. We’ve chosen to use std::env::args here
    for simplicity, because OsString values differ per platform and
    are more complex to work with than String values.
    */
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("query: {:?}\nfilename: {:?}", config.query, config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{contents}")
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
