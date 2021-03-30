extern crate clap;

use clap::App;
use diagram_base::TransformError;
use std::io::{self, Read};
use std::process;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn render(input: String) -> Result<String, TransformError> {
    let output = diagram_seq::transform(input)?;
    Ok(output)
}

fn main() -> io::Result<()> {
    App::new(PKG_NAME)
        .version(PKG_VERSION)
        .about("Diagrams as code")
        .author(PKG_AUTHORS)
        .get_matches();

    let input = read_from_stdin()?;
    let output = render(input);
    match output {
        Ok(repr) if !repr.is_empty() => println!("{}", repr),
        Ok(_) => eprintln!("Warning: No diagram was generated"),
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
    Ok(())
}
