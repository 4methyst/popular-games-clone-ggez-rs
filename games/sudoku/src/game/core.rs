use ggez::{
    event, Context, GameError
};

use crate::game::entity::*;

const SCREEN_SIZE: (f32, f32) = (720., 480.);

pub fn run() {
    let (ctx, events_loop) = 
        ggez::ContextBuilder::new(
            "Sudoku", 
            "alimulap")
        .window_setup(
            ggez::conf::WindowSetup::default()
            .title("Sudoku"))
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build().unwrap();

    let state = App::new(&ctx);
    event::run(ctx, events_loop, state);
}

struct App {
    board: Board,
}

impl App {
    fn new(ctx: &Context) -> Self {
        App {
            board: Board::init(&ctx),
        }
    }
}

impl event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }
}