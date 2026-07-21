use crate::paper::{Card, Number, Suit};

use derive_builder::Builder;
use macroquad::{
    prelude::*,
    ui::{Skin, root_ui},
};

pub static FONT: &[u8] = include_bytes!("/usr/share/fonts/TTF/DejaVuSerif.ttf");

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

impl Layout {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn lerp(self, rhs: Self, t: f32) -> Self {
        Self {
            position: self.position.lerp(rhs.position, t),
            size: self.size.lerp(rhs.size, t),
        }
    }
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
        for card in self.cards.iter() {
            let (i, j) = card.as_col_row();
            let x = i as f32 * cell_size.x + position.x;
            let y = j as f32 * cell_size.y + position.y;
            let layout = Layout::new(vec2(x, y), cell_size);
            card.draw(layout);
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

#[derive(Debug, Clone, Copy)]
pub struct Dock<'a> {
    table_textures: &'a [Texture2D],
}

impl<'a> Dock<'a> {
    pub fn new(table_textures: &'a [Texture2D]) -> Self {
        Self { table_textures }
    }

    pub fn layout_from_index(&self, i: usize) -> Layout {
        let len = self.table_textures.len();
        let x = (i % len) as f32 * screen_width() / len as f32;
        let y = screen_height() * 0.02;

        let texture = &self.table_textures[i];

        let width = screen_width() / self.table_textures.len() as f32 * 0.8;
        let height = width * (texture.height() / texture.width());

        Layout {
            position: vec2(x, y),
            size: vec2(width, height),
        }
    }

    pub fn layout_from_ptr(&self, ptr: &Texture2D) -> Layout {
        let index = self
            .table_textures
            .iter()
            .enumerate()
            .find_map(|(i, t)| std::ptr::eq(t, ptr).then_some(i))
            .expect("Cannot find the table.");

        self.layout_from_index(index)
    }
}

impl<'a> AutoLayoutDraw for Dock<'a> {
    fn draw(&self) {
        for (i, table) in self.table_textures.iter().enumerate() {
            let layout = self.layout_from_index(i);
            table.draw(layout);
        }
    }
}

pub struct Showing<'a> {
    texture: &'a Texture2D,
}

impl<'a> Showing<'a> {
    pub fn new(texture: &'a Texture2D) -> Self {
        Self { texture }
    }

    pub fn layout(&self) -> Layout {
        let height = screen_height() * 0.6;
        let width = height * (self.texture.width() / self.texture.height());
        let x = (screen_width() - width) / 2.0;
        let y = screen_height() - height;

        Layout {
            position: vec2(x, y),
            size: vec2(width, height),
        }
    }
}

impl<'a> AutoLayoutDraw for Showing<'a> {
    fn draw(&self) {
        let layout = self.layout();
        self.texture.draw(layout);
    }
}

/// A line that connects `table` in the dock, and that
/// showing in the middle of the screen.
pub struct Dock2ShowLine<'a> {
    dock: Dock<'a>,
    table_texture: &'a Texture2D,
}

impl<'a> Dock2ShowLine<'a> {
    pub fn new(dock: Dock<'a>, table_texture: &'a Texture2D) -> Self {
        Self {
            dock,
            table_texture,
        }
    }
}

impl<'a> AutoLayoutDraw for Dock2ShowLine<'a> {
    fn draw(&self) {
        let layout1 = self.dock.layout_from_ptr(self.table_texture);
        let pos1 = layout1.position + vec2(0.5 * layout1.size.x, layout1.size.y);

        let layout2 = Showing::new(self.table_texture).layout();
        let pos2 = layout2.position + vec2(0.5 * layout2.size.x, 0.0);

        draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, WHITE);
    }
}

pub struct Animator<'a> {
    total_frame: u32,
    frame: u32,
    table_textures: [&'a Texture2D; 3],
    dock: Dock<'a>,
}

impl<'a> Animator<'a> {
    pub fn new(total_frame: u32, tables: [&'a Texture2D; 3], dock: Dock<'a>) -> Self {
        Self {
            total_frame,
            frame: 0,
            table_textures: tables,
            dock,
        }
    }

    pub fn draw_next_frame(&mut self) -> AnimateState {
        self.frame += 1;

        let t = self.frame as f32 / self.total_frame as f32;
        // The index of the table we are playing
        let index = (t * 3.0).floor();

        if index < 3.0 {
            // The percentage of THIS table animation.
            let this_percentage = t * 3.0 - index;

            let texture = &self.table_textures[index as usize];

            let start = self.dock.layout_from_ptr(texture);
            let end = Showing { texture }.layout();

            let layout = start.lerp(end, this_percentage);
            texture.draw(layout);

            // draw the showing line
            Dock2ShowLine {
                dock: self.dock,
                table_texture: texture,
            }
            .draw();
        }

        // remember to draw the in-position tables
        for texture in self.table_textures.iter().take(index as usize) {
            let layout = Showing { texture }.layout();
            texture.draw(layout);
        }

        if index < 3.0 {
            AnimateState::Playing
        } else {
            AnimateState::Finished
        }
    }
}

pub struct Hint<'a> {
    text: &'a str,
    font_size: u16,
}

impl<'a> Hint<'a> {
    pub fn new(text: &'a str, font_size: u16) -> Self {
        Self { text, font_size }
    }
}

impl<'a> AutoLayoutDraw for Hint<'a> {
    fn draw(&self) {
        let center = get_text_center(self.text, None, self.font_size, 1.0, 0.0);
        let x = screen_width() * 0.5 - center.x;
        let y = screen_height() * 0.2 - center.y;

        draw_text(self.text, x, y, self.font_size as f32, WHITE);
    }
}

pub enum AnimateState {
    Playing,
    Finished,
}

impl Draw for Card {
    fn draw(&self, layout: Layout) {
        self.draw_suit_and_number(layout);
        draw_border(layout);
    }
}

trait CardDraw {
    fn draw_suit_and_number(&self, layout: Layout);
}

impl CardDraw for Card {
    fn draw_suit_and_number(&self, layout: Layout) {
        let (suit, number) = self.as_suit_and_number();
        draw_suit(suit, layout);
        draw_number(number, layout);
    }
}

impl Draw for Texture2D {
    fn draw(&self, layout: Layout) {
        let params = DrawTextureParams {
            dest_size: Some(layout.size),
            ..Default::default()
        };
        draw_texture_ex(self, layout.position.x, layout.position.y, WHITE, params);
    }
}

pub fn get_skins() -> [Skin; 2] {
    const TRANS: Color = color_u8!(0, 0, 0, 0);
    const FONT_SIZE: u16 = 30;
    let style1 = root_ui()
        .style_builder()
        .color(TRANS)
        .text_color(GREEN)
        .font_size(FONT_SIZE)
        .font(FONT)
        .unwrap()
        .build();
    let skin1 = Skin {
        button_style: style1,
        ..root_ui().default_skin()
    };
    let style2 = root_ui()
        .style_builder()
        .color(TRANS)
        .text_color(RED)
        .font_size(FONT_SIZE)
        .font(FONT)
        .unwrap()
        .build();
    let skin2 = Skin {
        button_style: style2,
        ..root_ui().default_skin()
    };

    [skin1, skin2]
}

pub fn setup_font() -> Result<(), macroquad::Error> {
    // let font = load_ttf_font_from_bytes(include_bytes!("poker_dejavu.ttf"))?;
    set_default_font(load_ttf_font_from_bytes(FONT).unwrap());

    Ok(())
}

fn draw_suit(suit: Suit, layout: Layout) {
    let suit_font_size = layout.size.x.round() as u16;

    let color = match suit {
        Suit::Club => WHITE,
        Suit::Heart => RED,
        Suit::Spade => WHITE,
        Suit::Diamond => RED,
    };
    let suit = suit.to_string();
    let suit_text_center = get_text_center(&suit, None, suit_font_size, 1.0, 0.0);
    // slightly below cell center.
    let cell_center_down = 0.5 * layout.size + vec2(0.0, layout.size.y * 0.3);
    let offset = cell_center_down - suit_text_center;
    let position = layout.position + offset;

    draw_text(&suit, position.x, position.y, suit_font_size as f32, color);
}

fn draw_number(number: Number, layout: Layout) {
    let number_font_size = (layout.size.x * 0.7).round() as u16;

    let number = number.to_string();
    let center = get_text_center(&number, None, number_font_size, 1.0, 0.0);
    // slightly above cell center for suits space.
    let cell_center_up = 0.5 * layout.size - vec2(0.0, layout.size.y * 0.3);
    let offset = cell_center_up - center;
    let position = layout.position + offset;

    draw_text(
        &number,
        position.x,
        position.y,
        number_font_size as f32,
        WHITE,
    );
}

fn draw_border(layout: Layout) {
    draw_rectangle_lines(
        layout.position.x,
        layout.position.y,
        layout.size.x,
        layout.size.y,
        2.0,
        WHITE,
    );
}
