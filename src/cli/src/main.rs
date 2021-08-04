extern crate clap;

use clap::{App, Arg};
use diagram_base::TransformError;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn read_from_stdin() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_from_path(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    let mut handle = File::open(&path)?;
    let mut buffer = String::new();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn render(input: &str) -> Result<String, TransformError> {
    let output = diagram_seq::transform(input)?;
    Ok(output)
}

fn main() -> io::Result<()> {
    let matches = App::new(PKG_NAME)
        .version(PKG_VERSION)
        .about("Diagrams as code")
        .author(PKG_AUTHORS)
        .arg(
            Arg::with_name("PATH")
                .help("Path to the .diag file to generate diagram for (- for STDIN).")
                .required(false)
                .index(1),
        )
        .get_matches();

    let input = {
        let path = matches.value_of("PATH").unwrap_or("-");
        let (verbose_path, result) = match path {
            "-" => ("STDIN", read_from_stdin()),
            path => (path, read_from_path(path)),
        };
        result.unwrap_or_else(|e| {
            eprintln!("{}: {}", verbose_path, e);
            process::exit(1);
        })
    };

    let output = render(input.as_str());
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
