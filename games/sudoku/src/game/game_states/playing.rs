use ggez::{
    Context, GameResult,
    graphics,
};

use crate::game::{
    entity::*,
    game_states::*,
};

pub struct Playing {
    game_board: GameBoard,
    number_board: NumberBoard,
}

impl Playing {
    pub fn new(ctx: &Context) -> Self {
        Playing {
            game_board: GameBoard::init(&ctx),
            number_board: NumberBoard::init(&ctx),
        }
    }
}

impl StateTrait for Playing {
    fn update(&mut self, _ctx: &Context) -> GameResult<Option<GameState>> {
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        self.game_board.draw(canvas)?;
        self.number_board.draw(canvas)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: &MouseButton, _point: &Point2<f32>) -> GameResult {
        Ok(())
    }
}