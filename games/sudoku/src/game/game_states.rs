use ggez::{GameResult, Context, graphics::Canvas};

pub mod main_menu;

pub enum GameState {
    MainMenu,
}

pub trait StateTrait {
    fn update(&mut self, ctx: &Context) -> GameResult;
    fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult;
}