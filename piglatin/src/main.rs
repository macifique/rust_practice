use std::io;

fn main() {
    let mut instr = String::new();

    match io::stdin().read_line(&mut instr) {
        Ok(_n) => {
            println!("Input text: {}", instr);
            println!("Converted text: {}", to_pig_latin(&instr));
        }
        Err(error) => println!("error: {}", error),
    }
}

fn to_pig_latin(val: &str) -> String {
    val
        .split_whitespace()
        .map(pigize)
        .map(|word| word+" ")
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn pigize(val: &str) -> String {
    let valstr = val.to_string();
    if valstr.is_empty() {
        return valstr;
    }
    let mut ret = String::new();
    let mut first_consonant :Option<char> = Option::None;
    let mut first = true;
    for letter in val.to_string().chars() {
        if first {
            first = false;
            if !is_vowel(letter) {
                first_consonant = Some(letter);
                continue;
            }
        }
        ret += &letter.to_string();
    }
    ret += "-";
    match first_consonant {
        None => ret + "hay",
        Some(val) => ret + &val.to_string() + "ay"
    }
}

fn is_vowel(val: char) -> bool {
    match val {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'A' => true,
        'E' => true,
        'I' => true,
        'O' => true,
        'U' => true,
        _ => false
    }
}

#[test]
fn test_pigize() {
    assert_eq!("ello-hay", pigize("hello"));
    assert_eq!("anana-bay", pigize("banana"));
    assert_eq!("apple-hay", pigize("apple"));
    assert_eq!("Apple-hay", pigize("Apple"));
    assert_eq!("", pigize(""));
    assert_eq!("", pigize(""));
    // assert_eq!("Ear-pay", pigize("Pear"));
}

#[test]
fn test_piglatin() {
    assert_eq!("itten-kay eel-hay", to_pig_latin("kitten   eel   "));
}