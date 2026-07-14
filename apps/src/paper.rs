use combination::v2::{Combine, Select};

pub type Paper = Vec<Card>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card(u8);

impl Card {
    pub fn as_col_row(&self) -> (u8, u8) {
        let row = self.0 / 13;
        let col = self.0 % 13;

        (col, row)
    }
}

pub fn setup_papers() -> [Paper; 8] {
    let mut papers: [Paper; 8] = std::array::from_fn(|_| Vec::new());
    let paper_indices: [usize; 8] = std::array::from_fn(|i| i);
    let included_paper_indices = paper_indices.try_select(&Combine::new(8, 5)).unwrap();

    let cards = (0..52).map(Card);
    for (card, included_paper_indices) in cards.zip(included_paper_indices) {
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
        let papers = setup_papers();
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

        for i in 0..52 {
            if i == my_card.0 {
                continue;
            }

            assert!(
                not_contained_papers
                    .iter()
                    .any(|paper| paper.contains(&Card(i)))
            );
        }
    }
}
