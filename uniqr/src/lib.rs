use std::borrow::Borrow;
use clap::{App, Arg};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Write};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

struct Out {
    out_file: Option<File>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.1.0")
        .author("Karoly Molnar")
        .about("Rust uniq")
        .arg(
            Arg::with_name("input_file")
                .takes_value(true)
                .default_value("-")
                .value_name("IN_FILE")
                .help("Input file")
                .required(false)
        )
        .arg(
            Arg::with_name("output_file")
                .takes_value(true)
                .value_name("OUT_FILE")
                .help("Output file")
                .required(false)
        )
        .arg(
            Arg::with_name("count")
                .takes_value(false)
                .short("c")
                .long("count")
                .help("Show counts")
                .required(false)
        )

        .get_matches();

    Ok(Config {
        in_file: matches.value_of("input_file").unwrap().parse()?,
        // out_file: matches.value_of("output_file").map(String::from),
        out_file: matches.value_of("output_file").map(String::from),
        count: matches.is_present("count")
    })
}

impl Out {
    fn init(&mut self, out_file: &Option<String>) -> MyResult<()> {
        self.out_file = match out_file {
            None => {
                Option::None
            }
            Some(f) => {
                Option::Some(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(f)?)
            }
        };
        Ok(())
    }

    fn write(&mut self, out_line: &str) -> MyResult<()> {
        match &self.out_file {
            None => {
                print!("{}", out_line);
            }
            Some( f) => {
                f.borrow().write_all(out_line.as_bytes())?;
            }
        }
        Ok(())
    }
}

pub fn run(config: Config) -> MyResult<()> {

    let mut out = Out {
        out_file: None
    };
    out.init(&config.out_file)?;


    let mut print = |count: u64, text: &str| -> MyResult<()>{
        if count > 0 {
            let out_str: String;
            if config.count {
                out_str = format!("{:>4} {}", count, text);
            } else {
                out_str = format!("{}", text);
            }
            out.write(&out_str)?;
        }
        Ok(())
    };

    let mut file = open(&config.in_file)
        .map_err(|e| format!("{}: {}", config.in_file, e))?;


    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            if count > 0 {
                print(count, &previous)?;
            }
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }

    if count > 0 {
        print(count, &previous)?;
    }

    Ok(())
}


fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

