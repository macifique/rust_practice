use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

impl FileInfo {
    fn add(&mut self, other: &FileInfo) {
        self.num_lines += other.num_lines;
        self.num_chars += other.num_chars;
        self.num_bytes += other.num_bytes;
        self.num_words += other.num_words;
    }
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
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count_characters")
                .short("m")
                .long("chars")
                .help("print the character counts")
                .takes_value(false)
                .conflicts_with("count_bytes"),
        )
        .arg(
            Arg::with_name("count_lines")
                .short("l")
                .long("lines")
                .help("print the newline counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("count_words")
                .short("w")
                .long("words")
                .help("print the word counts")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .default_value("-")
                .multiple(true),
        )
        .get_matches();

    let mut cfg = Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: matches.is_present("count_lines"),
        words: matches.is_present("count_words"),
        bytes: matches.is_present("count_bytes"),
        chars: matches.is_present("count_characters"),
    };
    // if !(cfg.lines || cfg.words || cfg.bytes || cfg.chars) {
    if [cfg.lines, cfg.bytes, cfg.words, cfg.chars]
        .iter()
        .all(|val| val == &false)
    {
        cfg.lines = true;
        cfg.words = true;
        cfg.bytes = true;
    }
    Ok(cfg)
}

pub fn run(config: Config) -> MyResult<()> {
    let mut info_sum = FileInfo {
        num_lines: 0,
        num_words: 0,
        num_bytes: 0,
        num_chars: 0,
    };
    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(f) => {
                let info = count(f)?;
                info_sum.add(&info);
                print_statistics(&config, &info, filename);
            }
        }
    }
    if config.files.len() > 1 {
        print_statistics(&config, &info_sum, "total");
    }
    Ok(())
}

fn print_statistics(config: &Config, info: &FileInfo, filename: &str) {
    if config.lines {
        print!("{:>8}", info.num_lines);
    }
    if config.words {
        print!("{:>8}", info.num_words);
    }
    if config.bytes {
        print!("{:>8}", info.num_bytes);
    }
    if config.chars {
        print!("{:>8}", info.num_chars);
    }
    if filename.ne("-") {
        println!(" {}", filename);
    }else {
        println!();
    }
    // println!("{}", if filename.ne("-") {filename}else{""});
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    // let mut buffer = Vec::new();
    // let result_bytes = file.read_to_end(&mut buffer);
    // let char_string = String::from_utf8_lossy(&buffer).to_string();
    //
    // num_bytes = result_bytes?;
    // num_chars = char_string.len();
    // num_lines = char_string.lines().count();
    // num_words = char_string.split_whitespace().count();

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }


    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

#[cfg(test)]
mod tests {
    use super::count;
    use super::FileInfo;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_bytes: 48,
            num_chars: 48,
        };
        assert_eq!(expected, info.unwrap());
    }
}
