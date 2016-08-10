#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rand;
extern crate ansi_term;

use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::fmt::{self, Display};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Card {
    front: String,
    back:  String
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.front == other.front &&
            self.back == other.back
    }
}

impl<'a> Card {
    fn new(front: &str, back: &str) -> Card {
        Card { front: front.to_owned(), back: back.to_owned() }
    }

    pub fn get_front(&'a self) -> &'a str {
        &*self.front
    }

    pub fn get_back(&'a self) -> &'a str {
        &*self.back
    }
}

#[derive(Debug)]
pub struct Anki {
    cards: Vec<Card>
}

impl Display for Anki {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.cards.iter()
            .map(|c| format!("{0: <15} \t {1}",
                             c.get_front(),
                             c.get_back()
                             )
                 )
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", string)
    }
}

pub enum AnkiExport<'a> {
    PlainText(&'a str)
}

pub trait AnkiExporter<'a> {
    fn from_plain_text(&'a str) -> Result<Anki, io::Error>;
}

fn strip_html(s: &str) -> String {
    lazy_static! {
        static ref HTML_RE: Regex = Regex::new(r"</?[^>]+?>").unwrap();
    }

    HTML_RE.replace_all(
        &*s.replace("&nbsp;", " "), ""
        )
}

impl<'a> AnkiExporter<'a> for AnkiExport<'a> {
    fn from_plain_text(source: &'a str) -> Result<Anki, io::Error> {
        let file  = try!(File::open(source));
        let cards = BufReader::new(&file).lines()
            .map(|line| {
                let parts = line.unwrap()
                                .split("\t")
                                .map(|s| strip_html(s))
                                .collect::<Vec<String>>();
                Card::new(&parts[0], &parts[1])
            })
        .collect::<Vec<Card>>();

        Ok(Anki::new(cards))
    }
}

impl<'a> Anki {
    fn new(cards: Vec<Card>) -> Anki {
        Anki { cards: cards }
    }

    pub fn get_cards(&self) -> Vec<Card> {
        self.cards.to_owned()
    }

    pub fn from(source: AnkiExport<'a>) -> Result<Anki, io::Error> {
        match source {
            AnkiExport::PlainText(source) => AnkiExport::from_plain_text(source)
        }
    }
}

pub mod guessing;
