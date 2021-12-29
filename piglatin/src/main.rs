use std::io;

fn main() {
    let mut instr = String::new();

    match io::stdin().read_line(&mut instr) {
        Ok(_n) => {
            println!("Input text: {}", instr);
            println!("Converted text: {}", to_pig_latin(&mut instr));
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
    let str_val = val.to_string();
    let mut ret = String::new();
    let mut first_consonant :Option<char> = Option::None;
    let mut first = true;
    for letter in str_val.chars() {
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