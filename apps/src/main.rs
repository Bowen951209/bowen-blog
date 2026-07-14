use core::panic;

use combination::v2::{Combine, Select};
use macroquad::prelude::*;

trait Draw {
    fn draw(&self);
}

// TODO: make a Card struct and use u8 instead of coordinate
struct Table {
    // The position of top-left corner.
    position: Vec2,
    // Size of each cell.
    cell_size: Vec2,
    row_count: u8,
    column_count: u8,
    thickness: f32,
    color: Color,
    cards: Vec<Card>,
}

impl Table {
    fn draw_borders(&self) {
        let border_size = self.cell_size * vec2(self.column_count as f32, self.row_count as f32);

        // vertical lines
        for x in (0..=self.column_count).map(|i| i as f32 * self.cell_size.x + self.position.x) {
            let y1 = self.position.y;
            let y2 = y1 + border_size.y;
            draw_line(x, y1, x, y2, self.thickness, self.color);
        }

        // horizontal lines
        for y in (0..=self.row_count).map(|i| i as f32 * self.cell_size.y + self.position.y) {
            let x1 = self.position.x;
            let x2 = x1 + border_size.x;
            draw_line(x1, y, x2, y, self.thickness, self.color);
        }
    }

    fn draw_cards(&self) {
        let number_font_size = (self.cell_size.x * 0.5).round() as u16;
        let suit_font_size = (self.cell_size.x * 0.8).round() as u16;

        for (i, j) in self.cards.iter().map(Card::as_col_row) {
            let x = i as f32 * self.cell_size.x + self.position.x;
            let y = j as f32 * self.cell_size.y + self.position.y;

            // draw number
            let number = (i + 1).to_string();
            let number_text_center = get_text_center(&number, None, number_font_size, 1.0, 0.0);
            // slightly above cell center for suits space.
            let cell_center_up = 0.5 * self.cell_size - vec2(0.0, self.cell_size.y * 0.3);
            let offset = cell_center_up - number_text_center;

            draw_text(
                &number,
                x + offset.x,
                y + offset.y,
                number_font_size as f32,
                self.color,
            );

            // draw suit
            let suit = Self::suit_from_row(j);
            let suit_text_center = get_text_center(suit, None, suit_font_size, 1.0, 0.0);
            // slightly below cell center.
            let cell_center_down = 0.5 * self.cell_size + vec2(0.0, self.cell_size.y * 0.3);
            let offset = cell_center_down - suit_text_center;

            Self::draw_suit(suit, x + offset.x, y + offset.y, suit_font_size as f32);
        }
    }

    fn draw_suit(suit: &str, x: f32, y: f32, font_size: f32) {
        let color = match suit {
            "♣" => WHITE,
            "♥" => RED,
            "♠" => WHITE,
            "♦" => RED,
            _ => panic!(r#"Can only handle "♣", "♥", "♠", and "♦"."#),
        };

        draw_text(suit, x, y, font_size, color);
    }

    fn suit_from_row(i: u8) -> &'static str {
        match i {
            0 => "♣",
            1 => "♥",
            2 => "♠",
            3 => "♦",
            _ => panic!("Can only handle row 0, 1, 2, and 3."),
        }
    }
}

impl Draw for Table {
    fn draw(&self) {
        self.draw_borders();
        self.draw_cards();
    }
}

#[macroquad::main("MyGame")]
async fn main() -> anyhow::Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("poker_dejavu.ttf"))?;
    set_default_font(font);

    let papers = setup_papers();
    let tables = tables_from_papers(papers);

    loop {
        clear_background(BLACK);

        tables.iter().for_each(Table::draw);

        next_frame().await;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Card(u8);

impl Card {
    fn as_col_row(&self) -> (u8, u8) {
        let row = self.0 / 13;
        let col = self.0 % 13;

        (col, row)
    }
}

type Paper = Vec<Card>;
fn setup_papers() -> [Paper; 8] {
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

fn tables_from_papers<const N: usize>(papers: [Vec<Card>; N]) -> [Table; N] {
    let mut tables = papers.map(|paper| Table {
        position: vec2(0.0, 0.0),
        cell_size: vec2(20.0, 40.0),
        column_count: 13,
        row_count: 4,
        thickness: 2.0,
        color: WHITE,
        cards: paper,
    });

    for (i, table) in tables.iter_mut().enumerate() {
        table.position.x = (i % 2 * 300) as f32;
        table.position.y = (i / 2 * 180) as f32;
    }

    tables
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
