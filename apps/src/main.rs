mod graphics;
mod paper;

use macroquad::{prelude::*, rand::RandGenerator};

use crate::{
    graphics::{Draw, Table, TableBuilder},
    paper::Paper,
};

#[macroquad::main("MyGame")]
async fn main() -> anyhow::Result<()> {
    let font = load_ttf_font_from_bytes(include_bytes!("poker_dejavu.ttf"))?;
    set_default_font(font);

    // TODO: let user set seed
    let papers = paper::setup_papers(&RandGenerator::new());
    let tables = tables_from_papers(papers);

    loop {
        clear_background(BLACK);

        for (i, table) in tables.iter().enumerate() {
            let x = (i % 2 * 300) as f32;
            let y = (i / 2 * 180) as f32;
            table.draw(vec2(x, y), vec2(20.0 * 13.0, 40.0 * 4.0));
        }

        next_frame().await;
    }
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
