use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    n_lines: usize,
    c_bytes: Option<usize>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("head")
        .version("0.1.0")
        .author("Karoly Molnar")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .takes_value(true)
                .default_value("10")
                // .conflicts_with("number_bytes")
        )
        .arg(
            Arg::with_name("number_bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .help("Number of bytes")
                .takes_value(true)
                .conflicts_with("number_lines")
        )
        .get_matches();

    let lines = matches
        .value_of("number_lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("number_bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        n_lines: lines.unwrap(),
        c_bytes: bytes
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {
                if let Some(num_bytes) = config.c_bytes {
                    // let mut handle = file.take(num_bytes as u64);
                    // let mut buffer = vec![0; num_bytes];
                    // let bytes_read = handle.read(&mut buffer)?;
                    let bb = file.bytes().take(num_bytes).collect::<Result<Vec<_>, _>>()?;
                    let bb3 = file.bytes().take(4).collect::<Result<Vec<_>,_>>()?;
                    io::stdout().write(bb.as_slice())?;

                    // print!(
                    // "{}",
                    // String::from_utf8_lossy(&buffer[..bytes_read])
                // );
                } else {
                    let mut line = String::new();
                    for _ in 0..config.n_lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        };
    }
    Ok(())
}


pub fn run3(config: Config) -> MyResult<()> {
    let more_files = config.files.len() > 1;
    let mut files_iterator = config.files.iter();
    let mut filename = files_iterator.next();
    while filename.is_some(){
        match open(&filename.unwrap()) {
            Err(err) => eprintln!("{}: {}", filename.unwrap(), err),
            Ok(reader) => {
                if more_files {
                    println!("==> {} <==", filename.unwrap());
                }
                let mut n_lines = config.n_lines;
                let mut c_bytes: usize = 0;
                let mut line_vec: Vec<u8> = Vec::new();

                for byte in reader.bytes() {
                    let this_byte = byte?;
                    // line_vec.push(byte?);
                    line_vec.push(this_byte);

                    if config.c_bytes.is_some() {
                        c_bytes += 1;
                        if c_bytes >= config.c_bytes.unwrap() {
                            io::stdout().write(line_vec.as_slice())?;
                            line_vec.clear();
                            break;
                        }

                    }else {
                        if this_byte == 10 {
                            n_lines -= 1;
                            io::stdout().write(line_vec.as_slice())?;
                            line_vec.clear();
                        }
                        if n_lines == 0 {
                            break;
                        }
                    }
                }
                io::stdout().write(line_vec.as_slice())?;
                io::stdout().flush()?;
            }
        }
        filename = files_iterator.next();
        if filename.is_some(){
            println!();
        }
    }
    Ok(())
}

fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse::<usize>() {
        Ok(n) if n> 0 => Ok(n),
        _ => Err(val.into())
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}