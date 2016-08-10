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
    back:  String,
    tag:   Option<String>
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.front == other.front &&
            self.back == other.back
    }
}

impl<'a> Card {
    fn new(front: &str, back: &str, tag: Option<String>) -> Card {
        Card {
            front: front.to_owned(),
            back:  back.to_owned(),
            tag:   tag
        }
    }

    pub fn get_front(&'a self) -> &'a str {
        &*self.front
    }

    pub fn get_back(&'a self) -> &'a str {
        &*self.back
    }

    pub fn get_tag(&'a self) -> &'a Option<String> {
        &self.tag
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
    AnkiPackage(&'a str),
    PlainTextNotes(&'a str),
    PlainTextCards(&'a str)
}

impl<'a> AnkiExport<'a> {
    pub fn from(string: &'a str, path: &'a str) -> AnkiExport<'a> {
        match &*string.to_lowercase() {
            "ankipackage" => AnkiExport::AnkiPackage(path),
            "notes"       => AnkiExport::PlainTextNotes(path),
            "cards"       => AnkiExport::PlainTextCards(path),
            _             => panic!("Format not recognized.")
        }
    }
}

pub trait AnkiExporter<'a> {
    fn from_anki_package(&'a str)     -> Result<Anki, io::Error>;
    fn from_plain_text_cards(&'a str) -> Result<Anki, io::Error>;
    fn from_plain_text_notes(&'a str) -> Result<Anki, io::Error>;
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
    fn from_anki_package(_: &'a str) -> Result<Anki, io::Error> {
        panic!("Not implemented yet.");
    }

    fn from_plain_text_cards(source: &'a str) -> Result<Anki, io::Error> {
        let file  = try!(File::open(source));
        let cards = BufReader::new(&file).lines()
            .map(|line| {
                let parts = line.unwrap()
                    .split("\t")
                    .map(|s| strip_html(s))
                    .collect::<Vec<String>>();
                Card::new(&parts[0], &parts[1], None)
            })
        .collect::<Vec<Card>>();

        Ok(Anki::new(cards))
    }

    fn from_plain_text_notes(source: &'a str) -> Result<Anki, io::Error> {
        let file  = try!(File::open(source));
        let cards = BufReader::new(&file).lines()
            .map(|line| {
                let parts = line.unwrap()
                    .split("\t")
                    .map(|s| strip_html(s))
                    .collect::<Vec<String>>();
                Card::new(&parts[0], &parts[1], Some(parts[2].to_owned()))
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

    pub fn select_tag(&'a mut self, tag: &'a str) {
        let some_tag = Some(tag.to_owned());
        self.cards.retain(|c| c.tag == some_tag);
    }

    pub fn from(source: AnkiExport<'a>) -> Result<Anki, io::Error> {
        match source {
            AnkiExport::AnkiPackage(source)    => AnkiExport::from_anki_package(source),
            AnkiExport::PlainTextNotes(source) => AnkiExport::from_plain_text_notes(source),
            AnkiExport::PlainTextCards(source) => AnkiExport::from_plain_text_cards(source)
        }
    }
}

pub mod guessing;
