use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
    None
}

impl EntryType {
    fn convert(s: &String) -> EntryType {
        match s.as_str() {
            "d" => EntryType::Dir,
            "f" => EntryType::File,
            "l" => EntryType::Link,
            _ => EntryType::None
        }
    }
}
impl FromStr for EntryType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "d" => Ok(EntryType::Dir),
            "f" => Ok(EntryType::File),
            "l" => Ok(EntryType::Link),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .about("Rust find")
        .version("0.1.0")
        .author("Karoly Molnar")
        .arg(
            Arg::with_name("dirs")
                .takes_value(true)
                .multiple(true)
                .default_value(".")
                .help("Search paths ")
                .value_name("PATH")
        )
        .arg(
            Arg::with_name("names")
                .takes_value(true)
                .short("n")
                .long("name")
                .help("Name")
                .value_name("NAME")
                .multiple(true)
                .number_of_values(1)
        )
        .arg(
            Arg::with_name("types")
                .takes_value(true)
                .short("t")
                .long("type")
                .help("Entry type")
                .value_name("TYPE")
                .multiple(true)
                .possible_values(&vec!["f", "d", "l"])
                .number_of_values(1)
        )
        .get_matches();

    let paths = matches.values_of_lossy("dirs").unwrap();
    let types : Result<Vec<EntryType>,()> = match matches.values_of_lossy("types") {
        Option::None => Ok(vec!()),
        Some(t) => t.iter()
            .map(String::as_str)
            .map(EntryType::from_str)
            .collect()
    };



    Ok(Config {
        paths,
        names: vec![],
        entry_types : match types {
            Ok(t) => t,
            Err(_) => vec![]
        }
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}
