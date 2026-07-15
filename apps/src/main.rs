mod graphics;
mod paper;

use std::iter::Peekable;

use arrayvec::ArrayVec;
use macroquad::{prelude::*, rand::RandGenerator};

use crate::{
    graphics::{Draw, Table, TableBuilder},
    paper::{Card, Paper},
};

#[derive(Debug)]
struct Asker<'a, I: Iterator<Item = &'a Table>> {
    maybe_asking: Option<&'a Table>,
    includeds: ArrayVec<&'a Table, 5>,
    excludeds: ArrayVec<&'a Table, 3>,
    to_asks: Peekable<I>,
}

impl<'a, I: Iterator<Item = &'a Table>> Asker<'a, I> {
    fn new(mut to_asks: Peekable<I>) -> Self {
        Self {
            maybe_asking: to_asks.next(),
            includeds: ArrayVec::new(),
            excludeds: ArrayVec::new(),
            to_asks,
        }
    }

    fn is_included_in_asking(&mut self, is_included: bool) {
        let asking = self
            .maybe_asking
            .expect("There is noting to ask, so there should be nothing to answer.");

        if is_included {
            self.includeds.push(asking);
        } else {
            self.excludeds.push(asking);
        }

        // check if we can get the result early
        if self.includeds.len() == 5 {
            for table in self.to_asks.by_ref() {
                self.excludeds.push(table);
            }

            return;
        }
        if self.excludeds.len() == 3 {
            for table in self.to_asks.by_ref() {
                self.includeds.push(table);
            }

            return;
        }

        // remember to ask the next one
        self.maybe_asking = self.to_asks.next();
    }

    fn answer(&self) -> Option<Card> {
        if self.excludeds.len() < 3 {
            return None;
        }

        let answer = Card::all()
            .find(|card| {
                self.excludeds
                    .iter()
                    .all(|table| !table.cards.contains(card))
            })
            .expect("Cannot find answer.");

        Some(answer)
    }
}

#[macroquad::main("MyGame")]
async fn main() -> anyhow::Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("poker_dejavu.ttf"))?;
    set_default_font(font);

    // TODO: let user set seed
    let papers = paper::setup_papers(&RandGenerator::new());
    let tables = tables_from_papers(papers);

    let mut asker = Asker::new(tables.iter().peekable());

    loop {
        clear_background(BLACK);
        draw_dock(&tables);

        if let Some(asking) = asker.maybe_asking {
            // interactively ask the user if there card is in the shown table

            // show the asking card
            draw_asking(asking);

            // user answer
            if is_key_pressed(KeyCode::Y) {
                asker.is_included_in_asking(true);
            } else if is_key_pressed(KeyCode::N) {
                asker.is_included_in_asking(false);
            }

            // check if we got the answer
            if let Some(answer) = asker.answer() {
                println!("Magic energy reached. Your card is `{answer:?}`!");
            }
        }

        next_frame().await;
    }
}

fn draw_dock(tables: &[Table]) {
    for (i, table) in tables.iter().enumerate() {
        let x = (i % tables.len()) as f32 * screen_width() / 8.0;
        let y = screen_height() * 0.02;

        let width = screen_width() / 8.0 * 0.8;
        let height = 2.0 * width * (table.row_count as f32 / table.column_count as f32);
        table.draw(vec2(x, y), vec2(width, height));
    }
}

fn draw_asking(table: &Table) {
    let height = screen_height() * 0.6;
    let width = height / 2.0 / (table.row_count as f32 / table.column_count as f32);

    let x = (screen_width() - width) / 2.0;
    let y = screen_height() - height;

    table.draw(vec2(x, y), vec2(width, height));
}

fn tables_from_papers<const N: usize>(papers: [Paper; N]) -> [Table; N] {
    let mut temp_array = papers.map(Some);

    std::array::from_fn(|i| {
        TableBuilder::default()
            .column_count(13)
            .row_count(4)
            .thickness(2.0)
            .color(WHITE)
            .cards(temp_array[i].take().unwrap())
            .build()
            .unwrap()
    })
}
