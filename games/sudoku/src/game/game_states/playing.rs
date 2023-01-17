use ggez::{
    Context, GameResult,
    graphics,
};

use crate::game::{
    entity::Board,
    game_states::*,
};

pub struct Playing {
    board: Board,
}

impl Playing {
    pub fn new(ctx: &Context) -> Self {
        Playing {
            board: Board::init(&ctx),
        }
    }
}

impl StateTrait for Playing {
    fn update(&mut self, _ctx: &Context) -> GameResult<Option<GameState>> {
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        self.board.draw(canvas)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: &MouseButton, _point: &Point2<f32>) -> GameResult {
        Ok(())
    }
}