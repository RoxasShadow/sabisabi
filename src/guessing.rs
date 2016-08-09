use std::io::stdin;
use rand::{thread_rng, Rng};
use ansi_term::Style;

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

    pub fn guess(&self, card_face: CardFace) {
        let mut rng   = thread_rng();
        let mut cards = self.get_cards();
        let     style = Style::new().bold();

        while !cards.is_empty() {

            let correct = Self::select(cards.to_owned(), 1).get(0)
                .unwrap()
                .to_owned();
            let wrongs  = Self::select(cards.to_owned(), 3);

            let mut options = vec![];
            options.extend(wrongs.iter().cloned());
            options.extend(vec![correct.to_owned()]
                           .iter()
                           .cloned()
                           );
            rng.shuffle(&mut options);

           loop {
                match card_face {
                    CardFace::Front => {
                        println!("{}\n  1) {}\n  2) {}\n  3) {}\n  4) {}",
                            style.paint(correct.get_back()),
                            options[0].get_front(),
                            options[1].get_front(),
                            options[2].get_front(),
                            options[3].get_front()
                        );
                   },
                    _ => {
                        println!("{}\n  1) {}\n  2) {}\n  3) {}\n  4) {}",
                            style.paint(correct.get_front()),
                            options[0].get_back(),
                            options[1].get_back(),
                            options[2].get_back(),
                            options[3].get_back()
                        );
                    }
                }

                let mut answer = String::new();
                let     result = stdin().read_line(&mut answer);
                if result.is_err() {
                    println!("Your answer is invalid. Please retry.");
                    continue;
                }

                let n = answer.trim().parse::<usize>();
                if n.is_err() {
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
                        continue;
                    }
                }

                if correct.index == options[n.unwrap() - 1].index {
                    println!("Your answer is correct!\n");
                    cards.remove(correct.index);
                    break;
                }
                else {
                    println!("Your answer is wrong.\n");
                    continue;
                }
            }
        }
    }
}
