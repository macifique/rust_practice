use std::borrow::Borrow;


#[derive(Debug, Default)]
struct Ps {
    x: i32,
    y: usize,
}

impl Ps {
    fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }
    fn y(mut self, y: usize) -> Self {
        self.y = y;
        self
    }
    fn new() -> Self {
        let ret : Ps = Default::default();
        ret
    }
}


fn main() {
    // let optar : [Option<i32>; 12] = [Option::None; 12];
    // println!("{:?}", optar);
    // println!("{:?}", optar);

    let mut mystr = String::from("This is my string");
    println!("{}", mystr);
    mystr.push_str(" ahy");
    myprint(mystr.borrow());
    mystr.push_str(" woo");
    myprint(mystr.borrow());
    // myprint(mystr.clone());


    // let ps2 = psps.set_x(1).set_y(2);
    let ps2 = Ps::new().x(3).y(10);
    dbg!(ps2);
    // let xpsps = psps.copy();

}

fn myprint(val: &str) {
    println!("{}", val);
}
