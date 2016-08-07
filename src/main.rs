extern crate sabisabi;
#[macro_use] extern crate lazy_static;

use std::env;
use sabisabi::{Anki, AnkiExport};
use sabisabi::guessing::CardFace;

lazy_static! {
    static ref USAGE: String = format!("Usage: {} [front/back] [path]",
                                       env!("CARGO_PKG_NAME"));
}

fn main() {
    if env::args().len() != 3 {
        panic!(&**USAGE);
    }

    let path = env::args().nth(2)
        .expect(&**USAGE);
    let anki = Anki::from(AnkiExport::PlainText(&*path)).unwrap();

    let side = env::args().nth(1)
        .expect(&**USAGE);
    match &*side.to_lowercase() {
        "front" => anki.guess(CardFace::Front),
        "back"  => anki.guess(CardFace::Back),
        _       => panic!(&**USAGE)
    }
}
