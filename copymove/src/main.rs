use std::borrow::{Borrow, BorrowMut};

struct Massive {
    a: [i128; 10000],
}

impl Massive {
    #[inline(always)]
    fn stupid(mut self) -> Self {
        println!("inside stupid {:?}", &mut self.a[1] as *mut i128);

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;

        //return the same object
        self
    }

    fn stupidref(& mut self) -> &Massive {
        println!("inside stupidref: {:?}", self.a[1] as *mut i128);

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;

        //return the same object
        self
    }

    fn stupidref2(& mut self){
        println!("inside stupidref: {:?}", self.a[1] as *mut i128);

        //do some stuff to alter it
        self.a[1] += 23;
        self.a[4] += 24;
    }


}

fn main() {
    let mut f = Box::new(Massive { a: [10i128; 10000] });

    println!("orig: {:?}", &mut f.a[1] as *mut i128);

    let mut f2 = f.stupid();
    println!("ret from stupid: {:?}", &mut f2.a[1] as *mut i128);

    // f.stupidref2();
    // let _f3 = f.stupidref().borrow();
    // println!("ret from stupidref {:?}", f3.a[1] as *mut i128);

}
