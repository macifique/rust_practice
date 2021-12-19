use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};


type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .author("Karoly Molnar")
        .version("0.1.0")
        .about("WC Rust")
        .arg(
            Arg::with_name("count_bytes")
                .short("c")
                .long("bytes")
                .help("print the byte counts")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("count_characters")
                .short("m")
                .long("chars")
                .help("print the character counts")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("count_lines")
                .short("l")
                .long("lines")
                .help("print the newline counts")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("count_words")
                .short("w")
                .long("words")
                .help("print the word counts")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines: false,
        words: false,
        bytes: false,
        chars: false
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}