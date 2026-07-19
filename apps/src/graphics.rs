use crate::paper::Card;

use derive_builder::Builder;
use macroquad::prelude::*;

pub trait Draw {
    fn draw(&self, layout: Layout);
}

pub trait AutoLayoutDraw {
    fn draw(&self);
}

#[derive(Debug, Clone, Copy)]
pub struct Layout {
    position: Vec2,
    size: Vec2,
}

#[derive(Debug, Builder)]
pub struct Table {
    pub cards: Vec<Card>,
    pub row_count: u8,
    pub column_count: u8,
    thickness: f32,
    color: Color,
}

impl Table {
    fn draw_borders(&self, position: Vec2, bound_size: Vec2, cell_size: Vec2) {
        // vertical lines
        for x in (0..=self.column_count).map(|i| i as f32 * cell_size.x + position.x) {
            let y1 = position.y;
            let y2 = y1 + bound_size.y;
            draw_line(x, y1, x, y2, self.thickness, self.color);
        }

        // horizontal lines
        for y in (0..=self.row_count).map(|i| i as f32 * cell_size.y + position.y) {
            let x1 = position.x;
            let x2 = x1 + bound_size.x;
            draw_line(x1, y, x2, y, self.thickness, self.color);
        }
    }

    fn draw_cards(&self, position: Vec2, cell_size: Vec2) {
        let number_font_size = (cell_size.x * 0.5).round() as u16;
        let suit_font_size = (cell_size.x * 0.8).round() as u16;

        for (i, j) in self.cards.iter().map(Card::as_col_row) {
            let x = i as f32 * cell_size.x + position.x;
            let y = j as f32 * cell_size.y + position.y;

            // draw number
            let number = (i + 1).to_string();
            let number_text_center = get_text_center(&number, None, number_font_size, 1.0, 0.0);
            // slightly above cell center for suits space.
            let cell_center_up = 0.5 * cell_size - vec2(0.0, cell_size.y * 0.3);
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
            let cell_center_down = 0.5 * cell_size + vec2(0.0, cell_size.y * 0.3);
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
    fn draw(&self, bound_layout: Layout) {
        let cell_size = bound_layout.size / vec2(self.column_count as f32, self.row_count as f32);

        self.draw_borders(bound_layout.position, bound_layout.size, cell_size);
        self.draw_cards(bound_layout.position, cell_size);
    }
}

pub struct Dock<'a> {
    pub tables: &'a [Table],
}

impl<'a> Dock<'a> {
    pub fn layout(&self, i: usize) -> Layout {
        let x = (i % self.tables.len()) as f32 * screen_width() / 8.0;
        let y = screen_height() * 0.02;

        let table = &self.tables[i];

        let width = screen_width() / self.tables.len() as f32 * 0.8;
        let height = 2.0 * width * (table.row_count as f32 / table.column_count as f32);

        Layout {
            position: vec2(x, y),
            size: vec2(width, height),
        }
    }
}

impl<'a> AutoLayoutDraw for Dock<'a> {
    fn draw(&self) {
        for (i, table) in self.tables.iter().enumerate() {
            let layout = self.layout(i);
            table.draw(layout);
        }
    }
}

// TODO: rename to Showing
pub struct Asking<'a> {
    pub table: &'a Table,
}

impl<'a> Asking<'a> {
    pub fn layout(&self) -> Layout {
        let height = screen_height() * 0.6;
        let width = height / 2.0 / (self.table.row_count as f32 / self.table.column_count as f32);
        let x = (screen_width() - width) / 2.0;
        let y = screen_height() - height;

        Layout {
            position: vec2(x, y),
            size: vec2(width, height),
        }
    }
}

impl<'a> AutoLayoutDraw for Asking<'a> {
    fn draw(&self) {
        let layout = self.layout();
        self.table.draw(layout);
    }
}

/// A line that connects the asking table in the dock, and that
/// showing in the middle of the screen.
pub struct AskingLine<'a, 'b> {
    pub dock: &'a Dock<'a>,
    pub asking: &'b Table,
}

impl<'a, 'b> AutoLayoutDraw for AskingLine<'a, 'b> {
    fn draw(&self) {
        let index_in_dock = self
            .dock
            .tables
            .iter()
            .enumerate()
            .find_map(|(i, t)| std::ptr::eq(t, self.asking).then_some(i))
            .expect("Cannot find `asking` in `dock`.");

        let layout1 = self.dock.layout(index_in_dock);
        let pos1 = layout1.position + vec2(0.5 * layout1.size.x, layout1.size.y);

        let layout2 = Asking { table: self.asking }.layout();
        let pos2 = layout2.position + vec2(0.5 * layout2.size.x, 0.0);

        draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, WHITE);
    }
}
