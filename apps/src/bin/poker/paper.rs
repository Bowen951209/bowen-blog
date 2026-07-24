use itertools::Itertools;
use macroquad::rand::{ChooseRandom, RandGenerator};

pub type Paper = Vec<Card>;

pub enum Suit {
    Club,
    Heart,
    Spade,
    Diamond,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Suit::Club => "♣",
            Suit::Heart => "♥",
            Suit::Spade => "♠",
            Suit::Diamond => "♦",
        };

        write!(f, "{s}")
    }
}

pub struct Number(u8);

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            1 => write!(f, "A"),
            11 => write!(f, "J"),
            12 => write!(f, "Q"),
            13 => write!(f, "K"),
            v => write!(f, "{v}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    pub fn as_suit_and_number(&self) -> (Suit, Number) {
        let (col, row) = self.as_col_row();

        let suit = match row {
            0 => Suit::Club,
            1 => Suit::Heart,
            2 => Suit::Spade,
            3 => Suit::Diamond,
            _ => panic!("Can only handle row 0, 1, 2, and 3."),
        };

        let number = Number(col + 1);

        (suit, number)
    }

    pub fn as_col_row(&self) -> (u8, u8) {
        let row = self.0 / 13;
        let col = self.0 % 13;

        (col, row)
    }

    pub fn all() -> impl Iterator<Item = Card> {
        (0..52).map(Card)
    }
}

pub fn setup_papers(rng: &RandGenerator) -> [Paper; 8] {
    let mut papers: [Paper; 8] = std::array::from_fn(|_| Vec::new());
    let mut included_paper_indices = (0usize..8).combinations(5).collect_array::<56>().unwrap();
    included_paper_indices.shuffle_with_state(rng);

    for (card, included_paper_indices) in Card::all().zip(included_paper_indices) {
        for included_paper_index in included_paper_indices {
            papers[included_paper_index].push(card);
        }
    }

    papers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn paper_setup() {
        let rng = RandGenerator::new();
        rng.srand(1122);

        let papers = setup_papers(&rng);
        let my_card = Card(21);

        let mut contained_papers = Vec::new();
        let mut not_contained_papers = Vec::new();

        for paper in papers {
            if paper.contains(&my_card) {
                contained_papers.push(paper);
            } else {
                not_contained_papers.push(paper);
            }
        }

        assert_eq!(contained_papers.len(), 5);
        assert_eq!(not_contained_papers.len(), 3);

        for card in Card::all() {
            if card == my_card {
                continue;
            }

            assert!(
                not_contained_papers
                    .iter()
                    .any(|paper| paper.contains(&card))
            );
        }
    }
}
