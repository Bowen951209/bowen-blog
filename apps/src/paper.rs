use combination::v2::{Combine, Select};
use macroquad::rand::{ChooseRandom, RandGenerator};

pub type Paper = Vec<Card>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
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
    let paper_indices: [usize; 8] = std::array::from_fn(|i| i);
    let mut included_paper_indices = paper_indices.try_select(&Combine::new(8, 5)).unwrap();
    included_paper_indices.shuffle_with_state(rng);

    for (card, included_paper_indices) in Card::all().zip(included_paper_indices) {
        for included_paper_index in included_paper_indices {
            papers[*included_paper_index].push(card);
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
