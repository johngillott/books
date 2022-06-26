use std::{env, error::Error, fs, process};
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

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("query: {:?}\nfilename: {:?}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{contents}");

    Ok(())
}
