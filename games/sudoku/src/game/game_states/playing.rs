use ggez::{
    Context, GameResult,
    graphics,
};

use crate::game::{
    constants::*,
    entity::*,
    game_states::*,
};

pub struct Playing {
    game_board: GameBoard,
    number_board: NumberBoard,
    background: graphics::Mesh,
}

impl Playing {
    pub fn new(ctx: &Context) -> Self {
        let vertices = [
            graphics::Vertex { position: [0., 0.], uv: [0., 0.], color: [0.001, 0., 0.001, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0, 0.], uv: [SCREEN_SIZE.0, 0.], color: [0., 0., 0.01, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0/2., SCREEN_SIZE.1/2.], uv: [SCREEN_SIZE.0/2., SCREEN_SIZE.1/2.], color: [0.015, 0., 0.02, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0, SCREEN_SIZE.1], uv: [SCREEN_SIZE.0, SCREEN_SIZE.1], color: [0.001, 0., 0.001, 1.] },
            graphics::Vertex { position: [0., SCREEN_SIZE.1], uv: [0., SCREEN_SIZE.1], color: [0., 0., 0.01, 1.] },
        ];
        let indices = [0, 1, 2, 2, 1, 3, 3, 2, 4, 4, 2, 0];
        let background = graphics::Mesh::from_data(ctx, graphics::MeshData {
            vertices: &vertices,
            indices: &indices,
        });
        Playing {
            game_board: GameBoard::init(&ctx),
            number_board: NumberBoard::init(&ctx),
            background,
        }
    }
}

impl StateTrait for Playing {
    fn update(&mut self, _ctx: &Context) -> GameResult<Option<GameState>> {
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());
        self.game_board.draw(canvas)?;
        self.number_board.draw(canvas)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: &MouseButton, _point: &Point2<f32>) -> GameResult {
        Ok(())
    }
}