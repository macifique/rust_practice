use crate::Extract::*;
use clap::{App, Arg};
use std::{error::Error, ops::Range};
use std::borrow::Borrow;
use regex::{Captures, Regex, RegexSet};

type MyResult<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<Range<usize>>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList)
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Karoly Molnar")
        .about("Rust cut")
        .arg(
            Arg::with_name("files")
                .takes_value(true)
                .default_value("-")
                .help("Input file(s)")
                .value_name("FILE")
                .multiple(true)
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .help("Selected bytes")
                .takes_value(true)
                .long("bytes")
                .short("b")
                .conflicts_with("chars")
                .conflicts_with("fields")
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .help("Selected characters")
                .takes_value(true)
                .long("chars")
                .short("c")
                .conflicts_with("bytes")
                .conflicts_with("fields")
        )
        .arg(
            Arg::with_name("fields")
                .value_name("FIELDS")
                .help("Selected fields")
                .takes_value(true)
                .long("fields")
                .short("f")
                .conflicts_with("bytes")
                .conflicts_with("chars")
        )
        .arg(
            Arg::with_name("delimiter")
                .value_name("DELIMITER")
                .help("Field delimiter")
                .short("d")
                .long("delim")
                .default_value("\t")
                .takes_value(true)
                .validator(valid_delimiter)
        )
        .get_matches();

    fn valid_delimiter(v: String) -> Result<(), String> {
        if v.len() >1 {
            return Err(String::from("the delimiter must be a single character"));
        }
        return Ok(());
    }

    //
    // fn valid_range(v: String) -> Result<(), String> {
    //     let set = RegexSet::new(&[
    //         r"\d+-{1}\d+",
    //         r"-{1}\d+",
    //         r"\d+-{1}",
    //         r"\d+",
    //     ]).unwrap();
    //     let matches = set.matches(&v);
    //     if matches.matched_any() {
    //         return Ok(());
    //     }
    //     return Err(String::from("Invalid range"));
    // }


    // fn parse_pos(range: &str) -> MyResult<PositionList> {
    //     let ranges = range.split(",").collect_vec();
    //
    //     let mut rangevec : PositionList = vec![];
    //
    //     for r in ranges {
    //
    //     }
    //
    //     Ok(PositionList(vec![]))
    //
    // }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        delimiter: matches.value_of("delimiter").unwrap().as_bytes()[0],
        extract: Extract::Chars(vec![])
    })
}

fn parse_pos(range: &str) -> MyResult<PositionList> {
    let regex_hi_lo = Regex::new(r"^(\d+)-(\d+)$").unwrap();
    let regex_no = Regex::new(r"^\d+$").unwrap();
    let range_set: Vec<_>= range.split(",").collect();
    let mut ret: PositionList = vec![];
    for r in range_set {
        let mut caps = regex_hi_lo.captures(r);
        if caps.is_some() {
            let ucap = caps.unwrap();
            let lotxt = ucap.get(1).map_or("", |m| m.as_str());
            let hitxt = ucap.get(2).map_or("", |m| m.as_str());
            let lo = get_num(lotxt)?;
            let hi = get_num(hitxt)?;
            if lo < hi {
                ret.push(Range{
                    start: lo-1,
                    end: hi
                });
            }else {
                return Err(format!("First number in range ({}) must be lower than second number ({})",lotxt, hitxt).into());
            }

        }else {
            caps = regex_no.captures(r);
            if caps.is_some() {
                let numtxt = caps.unwrap().get(0).map_or("", |m| m.as_str());
                let val = get_num(numtxt)?;
                ret.push(Range{
                    start: val-1,
                    end: val
                });
            }else {
                return Err(format!("illegal list value: \"{}\"",r).into());
            }
        }
    }
    if ret.len() > 0 {
        Ok(ret)
    }else {
        Err(format!("illegal list value: \"{}\"",range).into())
    }

}


fn get_num(nstr: &str) ->MyResult<usize> {
    let n = nstr.parse::<usize>().expect(&format!("illegal list value: {}", nstr));
    if n > 0 {
        Ok(n)
    }else {
        Err(format!("illegal list value: \"{}\"", nstr).into())
    }
}

pub fn run(config: Config) -> MyResult<()>{
    dbg!(config);
    Ok(())
}


#[cfg(test)]
mod unit_tests {
    use super::parse_pos;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("").is_err());

        // Zero is an error
        let res = parse_pos("0");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        let res = parse_pos("0-1");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"0\"",);

        // A leading "+" is an error
        let res = parse_pos("+1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1\"",
        );

        let res = parse_pos("+1-2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"+1-2\"",
        );

        let res = parse_pos("1-+2");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-+2\"",
        );

        // Any non-number is an error
        let res = parse_pos("a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1,a");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "illegal list value: \"a\"",);

        let res = parse_pos("1-a");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"1-a\"",
        );

        let res = parse_pos("a-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "illegal list value: \"a-1\"",
        );

        // Wonky ranges
        let res = parse_pos("-");
        assert!(res.is_err());

        let res = parse_pos(",");
        assert!(res.is_err());

        let res = parse_pos("1,");
        assert!(res.is_err());

        let res = parse_pos("1-");
        assert!(res.is_err());

        let res = parse_pos("1-1-1");
        assert!(res.is_err());

        let res = parse_pos("1-1-a");
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1");
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20");
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
