extern crate clap;
extern crate sabisabi;
#[macro_use] extern crate lazy_static;

use clap::{App, Arg};
use sabisabi::{Anki, AnkiExport};
use sabisabi::guessing::CardFace;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("side")
             .short("s")
             .long("side")
             .value_name("front/side")
             .takes_value(true)
             .help("which side of the cards you want to guess")
             )
        .arg(Arg::with_name("path")
             .short("p")
             .long("path")
             .value_name("PATH")
             .takes_value(true)
             .help("path to the Anki deck")
             )
        .arg(Arg::with_name("format")
             .short("f")
             .long("format")
             .value_name("AnkiPackage/Notes/Cards")
             .takes_value(true)
             .help("format of the exported Anki deck")
             )
        .arg(Arg::with_name("tag")
             .short("t")
             .long("tag")
             .value_name("TAG")
             .takes_value(true)
             .help("only cards with given tag will be considered")
             )
        .arg(Arg::with_name("strip_parents")
             .long("strip-parents")
             .help("remove all the parenthesis from the Q&As")
             )

        .get_matches();

    if let Some(side) = matches.value_of("side") {
        if let Some(path) = matches.value_of("path") {
            if let Some(format) = matches.value_of("format") {
                let     format = AnkiExport::from(format, path);
                let mut anki   = Anki::from(format).unwrap();

                if let Some(tag) = matches.value_of("tag") {
                    anki.select_tag(&*tag);
                }

                let strip_parents = matches.is_present("strip_parents");
                match &*side.to_lowercase() {
                    "front" => anki.guess(CardFace::Front, strip_parents),
                    "back"  => anki.guess(CardFace::Back, strip_parents),
                    _       => println!("Invalid side given.")
                }
            }
            else {
                println!("Format not provided. Run with --help for more instructions.");
            }
        }
        else {
            println!("Path not provided. Run with --help for more instructions.");
        }
    }
    else {
        println!("Side not provided. Run with --help for more instructions.");
    }
}
