use std::env::args;

use muslib::mixer::Loader;

fn main() {
    let f = args().last().unwrap();

    let loader = &mut Loader::<f64>::new();
    let _ = loader.file(f.into()).mono().load();

    let audio = loader.data();
    let outstr = audio
        .iter()
        .map(|x| (*x * 100_f64).round().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", outstr);
}
