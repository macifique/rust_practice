use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use std::path::Path;
use std::str::FromStr;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
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
    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|x|{
                    EntryType::from_str(&x)
                        .map_err(|_| format!("Invalid --type \"{}\"", x))
                })
                .collect::<Result<Vec<_>, _>>()
            })
        .transpose()?
        .unwrap_or_default();

    Ok(Config {
        paths,
        names,
        entry_types
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(ex) => {
                    let path = ex.path();
                    if fits(&config.entry_types, path) {
                        let filename = path.file_name().unwrap().to_str().unwrap();
                        if matches(&config.names, filename) {
                            println!("{}", ex.path().display())
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


fn matches(reg_list: &Vec<Regex>, attr: &str) -> bool {
    let mut ret = if reg_list.len() > 0 {
        false
    }else {
        true
    };
    for r in reg_list {
        if r.is_match(attr) {
            ret = true;
            break;
        }
    }
    ret
}

fn fits(entry_list: &Vec<EntryType>, path: &Path) -> bool {
    let mut ret = false;

    if path.is_dir() && entry_list.contains(&EntryType::Dir) {
        ret = true;
    }else if path.is_file() && entry_list.contains(&EntryType::File) {
        ret = true;
    }else if !path.is_file() && !path.is_dir() && entry_list.contains(&EntryType::Link) {
        ret = true;
    }else if entry_list.len() == 0 {
        ret = true
    }
    ret
}