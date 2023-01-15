use ggez::{
    event, Context, GameResult,
    graphics::{ self }
};

// use crate::game::entity::*;
use crate::game::game_states::{
    GameState, StateTrait, main_menu::MainMenu
};

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
    states: Vec<Box<dyn StateTrait>>,
}

impl App {
    fn new(ctx: &Context) -> Self {
        App {
            states: vec![
                Box::new(MainMenu::new(&ctx)),
            ],
        }
    }
}

impl event::EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        canvas.finish(ctx)?;
        Ok(())
    }
}