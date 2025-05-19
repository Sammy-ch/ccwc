use clap::Parser;
use std::{
    error::Error,
    fs::File,
    io::{self, BufReader, Read},
};

/// Rust implementation of wc (word count)
#[derive(Parser, Debug)]
#[command(version, about = "Rust version of wc")]
struct Args {
    /// File name (reads from stdin if not provided)
    #[arg()]
    file: Option<String>,

    /// Print the newline count
    #[arg(short = 'l')]
    lines: bool,

    /// Print the word count
    #[arg(short = 'w')]
    words: bool,

    /// Print the byte count
    #[arg(short = 'c')]
    bytes: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let input: Box<dyn Read> = match &args.file {
        Some(filename) => Box::new(File::open(filename)?),
        None => Box::new(io::stdin()),
    };

    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let line_count = buffer.lines().count();
    let word_count = buffer.split_whitespace().count();
    let byte_count = buffer.as_bytes().len();

    let show_all = !args.lines && !args.words && !args.bytes;

    if show_all || args.lines {
        print!("{:>8} ", line_count);
    }
    if show_all || args.words {
        print!("{:>8} ", word_count);
    }
    if show_all || args.bytes {
        print!("{:>8} ", byte_count);
    }

    if let Some(filename) = &args.file {
        println!("{}", filename);
    } else {
        println!(); // reading from stdin
    }

    Ok(())
}
