use super::context::AddOnContext;
use ggez::{event::MouseButton, graphics::Canvas, mint::Point2, Context, GameResult};

pub mod leader_board;
pub mod main_menu;
pub mod playing;
pub mod select_difficulty;

#[derive(Clone, Copy)]
pub enum GameState {
    MainMenu,
    SelectDifficulty,
    Playing,
    LeaderBoard,
}

pub trait StateTrait {
    fn update(
        &mut self,
        _ctx: &Context,
        _addon_ctx: &mut AddOnContext,
    ) -> GameResult<Option<GameState>>;
    fn draw(&mut self, _ctx: &mut Context, _canvas: &mut Canvas) -> GameResult;
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: &MouseButton,
        _point: &Point2<f32>,
    ) -> GameResult;
}
