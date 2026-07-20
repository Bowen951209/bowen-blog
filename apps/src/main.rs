mod graphics;
mod paper;

use std::iter::Peekable;

use arrayvec::ArrayVec;
use macroquad::{prelude::*, rand::RandGenerator};

use crate::{
    graphics::{
        AnimateState, Animator, AutoLayoutDraw, Dock, Dock2ShowLine, Draw, Hint, Layout, Showing,
        Table, TableBuilder,
    },
    paper::{Card, Paper},
};

// TODO: name 'a 'tables
struct Game<'a, I: Iterator<Item = &'a Table>> {
    tables: &'a [Table; 8],
    table_textures: &'a [Texture2D; 8],
    state: InteractState<'a>,
    asker: Asker<'a, I>,
}

impl<'a, I: Iterator<Item = &'a Table>> Game<'a, I> {
    fn new(
        tables: &'a [Table; 8],
        table_textures: &'a [Texture2D; 8],
        asker: Asker<'a, I>,
    ) -> Self {
        Self {
            tables,
            table_textures,
            asker,
            state: InteractState::Asking,
        }
    }

    fn update(&mut self) {
        match self.state {
            InteractState::Asking => self.handle_ask(),
            InteractState::PlayingAnimation(_) => self.handle_animation(),
            InteractState::ShowAnswer => self.handle_show_answer(),
        }
    }

    fn handle_ask(&mut self) {
        Self::draw_hint();
        self.draw_dock();
        self.draw_ask();
        self.keyboard_answer();

        // If we got the answer, switch state to `PlayingAnimation`.
        if let Some(answer) = self.asker.answer() {
            println!("Magic energy reached. Your card is `{answer:?}`!");

            let textures = self.excluded_table_textures();

            let animator = Animator::new(180, textures, Dock::new(self.table_textures));
            self.state = InteractState::PlayingAnimation(animator);
        }
    }

    fn handle_animation(&mut self) {
        if let InteractState::PlayingAnimation(ref mut animator) = self.state {
            if matches!(animator.draw_next_frame(), AnimateState::Finished) {
                self.state = InteractState::ShowAnswer;
            }
        } else {
            panic!("It is not in PlayingAnimation state.")
        }

        self.draw_dock();
    }

    fn handle_show_answer(&self) {
        // draw the asnwer text
        const TEXT: &str = "Magic! Your card is:";
        const FONT_SIZE: u16 = 20;
        const CARD_WIDTH: f32 = 30.0;
        const CARD_HEIGHT: f32 = 50.0;
        let answer = self.asker.answer().expect("Answer is not found.");

        Hint::new(TEXT, FONT_SIZE).draw();

        let dim = measure_text(TEXT, None, FONT_SIZE, 1.0);
        let x = (dim.width + screen_width()) * 0.5;
        let y = screen_height() * 0.2 - CARD_HEIGHT * 0.5;

        answer.draw(Layout::new(vec2(x, y), vec2(CARD_WIDTH, CARD_HEIGHT)));

        // remeber to draw the tables
        for texture in self.excluded_table_textures() {
            Showing::new(texture).draw();
        }
    }

    fn draw_dock(&self) {
        Dock::new(self.table_textures).draw();
    }

    /// Draw `asker`'s asking table in the middle of the screen, and
    /// draw a line connecting it and that copy in the `dock`.
    fn draw_ask(&self) {
        let asking = self
            .asker
            .maybe_asking
            .expect("There's nothing left to ask");
        let texture = self.texture_from_table(asking);

        // draw the asking table in the middle of the screen
        Showing::new(texture).draw();

        // draw the line that connects the asking table in the
        // dock, and that in the middle of the screen.
        Dock2ShowLine::new(Dock::new(self.table_textures), texture).draw();
    }

    /// Handle keyboard input. User press `Y` if his card is contained
    /// in `asker`'s asking table. Otherwise, he press `N`.
    fn keyboard_answer(&mut self) {
        if is_key_pressed(KeyCode::Y) {
            self.asker.is_included_in_asking(true);
        } else if is_key_pressed(KeyCode::N) {
            self.asker.is_included_in_asking(false);
        }
    }

    fn draw_hint() {
        Hint::new("Is your card in the table? (y/n)", 20).draw();
    }

    /// Return `excludeds` textures. Call this after `excludeds` has
    /// len of 3. Otherwise it panics.
    fn excluded_table_textures(&self) -> [&'a Texture2D; 3] {
        self.asker
            .excludeds
            .clone()
            .into_inner()
            .expect("`excludeds` len is not 3")
            .map(|table| self.texture_from_table(table))
    }

    fn texture_from_table(&self, table: &Table) -> &'a Texture2D {
        let index = self
            .tables
            .iter()
            .position(|t| std::ptr::eq(t, table))
            .expect("Cannot find the excluded table in all 8 tables");
        &self.table_textures[index]
    }
}

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

enum InteractState<'a> {
    Asking,
    PlayingAnimation(Animator<'a>),
    ShowAnswer,
}

#[macroquad::main("MyGame")]
async fn main() -> anyhow::Result<()> {
    setup_font()?;

    let tables = setup_tables();
    let targets = render_tables(&tables).await;
    let table_textures = std::array::from_fn(|i| targets[i].texture.weak_clone());

    let asker = Asker::new(tables.iter().peekable());
    let mut game = Game::new(&tables, &table_textures, asker);

    loop {
        // clear_background(BLACK);
        game.update();
        next_frame().await;
    }
}

fn setup_font() -> Result<(), macroquad::Error> {
    let font = load_ttf_font_from_bytes(include_bytes!("/usr/share/fonts/TTF/DejaVuSerif.ttf"))?;
    // let font = load_ttf_font_from_bytes(include_bytes!("poker_dejavu.ttf"))?;
    set_default_font(font);

    Ok(())
}

fn setup_tables() -> [Table; 8] {
    // TODO: let user set seed
    let papers = paper::setup_papers(&RandGenerator::new());
    tables_from_papers(papers)
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

async fn render_tables<const N: usize>(tables: &[Table; N]) -> [RenderTarget; N] {
    const WIDTH: u32 = 650;
    const HEIGHT: u32 = 400;

    let targets = std::array::from_fn(|_| render_target(WIDTH, HEIGHT));

    for (target, table) in targets.iter().zip(tables) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / WIDTH as f32, 2.0 / HEIGHT as f32),
            target: vec2(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5),
            render_target: Some(target.clone()),

            ..Default::default()
        });

        table.draw(Layout::new(Vec2::ZERO, vec2(WIDTH as f32, HEIGHT as f32)));
        next_frame().await; // this is neccessary. make sure things are drawn
        // on the texture before generating mipmap.
        target.texture.generate_mipmap();
    }

    // remember to reset the default camera
    set_default_camera();

    targets
}

trait MipMap {
    fn generate_mipmap(&self);
}

impl MipMap for Texture2D {
    fn generate_mipmap(&self) {
        use miniquad::MipmapFilterMode;

        let texture_id = self.raw_miniquad_id();
        let gl_ctx = unsafe { get_internal_gl().quad_context };

        gl_ctx.texture_generate_mipmaps(texture_id);
        gl_ctx.texture_set_min_filter(texture_id, FilterMode::Linear, MipmapFilterMode::Linear);
        gl_ctx.texture_set_mag_filter(texture_id, FilterMode::Linear);
    }
}
