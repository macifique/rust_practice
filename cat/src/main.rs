fn main() {
    if let Err(b) = cat::get_args().and_then(cat::run) {
        eprintln!("{}", b);
        std::process::exit(1);
    }
}
