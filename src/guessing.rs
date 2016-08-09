use std::io::stdin;
use rand::{thread_rng, Rng};
use ansi_term::Style;
use regex::Regex;

use super::{Anki, Card};

pub enum CardFace {
    Front,
    Back
}

#[derive(Debug, Clone)]
pub struct SelectedCard {
    index: usize,
    card:  Card,
}

impl SelectedCard {
    fn new(index: usize, card: Card) -> SelectedCard {
        SelectedCard { index: index, card: card }
    }

    fn get_front(&self) -> &str {
        self.card.get_front()
    }

    fn get_back(&self) -> &str {
        self.card.get_back()
    }
}

impl<'a> Anki {
    fn select(cards: Vec<Card>, count: i32) -> Vec<SelectedCard> {
        let     len = cards.len();
        let mut rng = thread_rng();
        let mut selections: Vec<SelectedCard> = vec![];

        for _ in 0..count {
            let n = rng.gen_range(0, len);

            selections.push(
                SelectedCard::new(n, cards.get(n)
                                  .unwrap()
                                  .to_owned())
                );
        }

        selections
    }

    pub fn guess(&self, card_face: CardFace, strip_parents: bool) {
        let mut rng   = thread_rng();
        let mut cards = self.get_cards();
        let     style = Style::new().bold();

        while !cards.is_empty() {
            let correct = Self::select(cards.to_owned(), 1).get(0)
                .unwrap()
                .to_owned();
            let wrongs = Self::select(cards.to_owned(), 3);

            let mut options = vec![];
            options.extend(wrongs.iter().cloned());
            options.extend(vec![correct.to_owned()]
                           .iter()
                           .cloned()
                           );
            rng.shuffle(&mut options);

           loop {
                let question = match card_face {
                    CardFace::Front => {
                        format!("{}\n  1) {}\n  2) {}\n  3) {}\n  4) {}",
                            style.paint(correct.get_back()),
                            options[0].get_front(),
                            options[1].get_front(),
                            options[2].get_front(),
                            options[3].get_front()
                        )
                   },
                    _ => {
                        format!("{}\n  1) {}\n  2) {}\n  3) {}\n  4) {}",
                            style.paint(correct.get_front()),
                            options[0].get_back(),
                            options[1].get_back(),
                            options[2].get_back(),
                            options[3].get_back()
                        )
                    }
                };

                lazy_static! {
                    static ref PARENTS_RE: Regex = Regex::new(r"\([^)]*\)").unwrap();
                }

                if strip_parents {
                    println!("{}", PARENTS_RE.replace_all(
                            &*question, ""));
                }
                else {
                    println!("{}", question);
                }

                let mut answer = String::new();
                let     result = stdin().read_line(&mut answer);
                if result.is_err() {
                    println!("Your answer is invalid. Please retry.");
                    continue;
                }

                match answer.trim().parse::<usize>() {
                    Ok(n) => {
                        if n > 0 && n < 5 && correct.index == options[n - 1].index {
                            println!("Your answer is correct!\n");
                            cards.remove(correct.index);
                            break;
                        }
                        else {
                            println!("Your answer is wrong.\n");
                        }
                    },
                    Err(_) => {
                        let side = match card_face {
                            CardFace::Front => correct.card.get_front(),
                            _               => correct.card.get_back()
                        };

                        if answer.trim() == side {
                            println!("Your answer is correct!\n");
                            break;
                        }
                        else {
                            println!("Your answer is invalid. Please retry.\n");
                        }
                    }
                }
           }
        }
    }
}
