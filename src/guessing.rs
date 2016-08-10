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

impl PartialEq for SelectedCard {
    fn eq(&self, other: &SelectedCard) -> bool {
        self.card == other.card
    }
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
    fn get_random_pair(cards: Vec<Card>, cards_number: usize) -> (SelectedCard, Vec<SelectedCard>) {
        let len = cards.len();

        if len < cards_number + 1 {
            panic!("There are not enough cards in the deck.");
        }

        let mut rng = thread_rng();
        let mut found_n: Vec<usize> = Vec::new();

        loop {
            let n = rng.gen_range(0, len);
            if !found_n.contains(&n) {
                found_n.push(n);
            }
            if found_n.len() == cards_number + 1 {
                break;
            }
        }

        let p1 = {
            let n = found_n.pop().unwrap();
            SelectedCard::new(n, cards.get(n)
                              .unwrap()
                              .to_owned()
                              )
        };

        let p2 = found_n.into_iter().map(|n| {
            SelectedCard::new(n, cards.get(n)
                              .unwrap()
                              .to_owned()
                              )
        }).collect::<Vec<SelectedCard>>();

        (p1, p2)
    }

    pub fn guess(&self, card_face: CardFace, strip_parents: bool) {
        let mut rng   = thread_rng();
        let mut cards = self.get_cards();
        let     style = Style::new().bold();

        while !cards.is_empty() {
            let (correct, wrongs) = Self::get_random_pair(cards.to_owned(), 3 as usize);

            let mut options = vec![];
            options.extend(wrongs.into_iter()
                           .to_owned());
            options.extend(vec![correct.to_owned()].into_iter()
                           .to_owned());
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
