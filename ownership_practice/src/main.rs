use std::borrow::Borrow;

fn main() {
    println!("Hello, world!");
    let mut s1 = String::new();
    s1.push_str("aaa - ");
    let mut s2 = valstr(s1);
    // let mut borrowed_s2 = s2.borrow();
    let s2ref2 = &mut s2;
    println!("Size: {}", slen(s2ref2));
    let s2ref3 = &mut s2;
    s2ref3.push_str(" ftftfft");
    // let s2ref = &s2;

    // println!("{}", s2ref3);
    // println!("{}", s2ref3);

    let s2ref4 = & mut s2;
    s2ref4.push_str(" 222222");
    // s2ref3.push_str(" ftftfft");
    println!("{}", s2ref4);
    // let s2ref5 = s2ref3;

    // println!("{}", s2ref5);
    // let reference_to_nothing = dangle();

    let mut s = String::from("hello world");
    let uu = s.borrow();
    let word = first_word(&s);

    println!("the first word is: {}", word);
    s.clear(); // error!


}

fn valstr(mut value: String) -> String {
    value.push_str("update update");
    value
}

fn slen(val: &mut String) -> usize {
    val.push_str(" ooo ");
    val.len()
}

fn dangle() -> Box<String> {
    let s = Box::from(String::from("hello"));

    s
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}