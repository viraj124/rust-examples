use std::env;
use std::fs;
use std::process;
use ch12_minigrep_project::{search_case_insensitive, search};

fn main() {
    let finderConfig = Finder::new(env::args());

    match finderConfig {
        Ok(config) => {
            println!("query is {}", config.query);
            println!("file is {}", config.file);

            if let Err(e) = run(config){
                eprintln!("error is {}", e);
                process::exit(1);
            }
        },
        Err(e) => eprintln!("error is {}", e)
    }
}

fn run(finder: Finder) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(finder.file)?;
    println!("contents is {}", contents);

    if (finder.isSensitive) {
        println!("case sensitive");
        for line in search(&contents, &finder.query) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&contents, &finder.query) {
        println!("{}", line);
    }
    }
    Ok(())

}
struct Finder{
    query: String,
    file: String,
    isSensitive: bool
}

impl Finder{
    fn new(mut args: env::Args) -> Result<Finder, String> {
        args.next();

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no file"))
        };
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(String::from("no query"))
        };

        let isSensitive = env::var("IGNORE_CASE").is_ok();

        Ok(Finder {
            query: query,
            file: file,
            isSensitive: isSensitive
        })
    }
}