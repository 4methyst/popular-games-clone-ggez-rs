use ggez::{
    event, Context, GameResult,
    graphics::{ self }
};

// use crate::game::entity::*;
use crate::game::{
        game_states::{
        GameState, StateTrait, main_menu::MainMenu
    },
    constants::*,
};

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

    let state = App::new(&ctx, GameState::MainMenu);
    event::run(ctx, events_loop, state);
}

struct App {
    states: Vec<Box<dyn StateTrait>>,
    current_state: GameState,
}

impl App {
    fn new(ctx: &Context, initial_state: GameState) -> Self {
        App {
            states: vec![
                Box::new(MainMenu::new(&ctx)),
            ],
            current_state: initial_state,
        }
    }

    fn get_current_state(&mut self) -> &mut Box<dyn StateTrait> {
        match self.current_state {
            GameState::MainMenu => &mut self.states[GameState::MainMenu as usize],
        }
    }
}

impl event::EventHandler for App {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.get_current_state().update(&ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        self.get_current_state().draw(ctx, &mut canvas)?;
        canvas.finish(ctx)?;
        Ok(())
    }
}