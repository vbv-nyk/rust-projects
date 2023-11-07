use std::{env, error::Error, fs, process};

struct Args {
    filename: String,
    query: String,
}

impl Args {
    fn newConfig(filename: String, query: String) -> Args {
        Args { filename, query }
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    Ok(fs::read_to_string(path)?.to_string())
}

fn initialise(args: Vec<String>) -> Result<Args, &'static str> {
    if args.len() < 3 {
        return Err("Not enough arguments");
    }
    Ok(Args::newConfig(args[1].to_string(), args[2].to_string()))
}

fn search(text: String, query: String) {
    let lines = text.lines();
    for line in lines {
        if line.contains(&query) {
            println!("{}", line);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let args = initialise(args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    let file_content = read_file(&args.filename).unwrap_or_else(|err| {
        println!("Could not convert text to string");
        process::exit(1)
    });

    search(file_content, args.query);
}
