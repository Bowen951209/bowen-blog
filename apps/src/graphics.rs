use crate::paper::Card;

use derive_builder::Builder;
use macroquad::prelude::*;

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug, Builder)]
pub struct Table {
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
