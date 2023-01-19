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
    number_selection: usize,
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
            game_board: GameBoard::init(&ctx, &Difficulty::Easy),
            number_board: NumberBoard::init(&ctx),
            background,
            number_selection: 0,
        }
    }

    fn update_state(&mut self) {
        for i in 0..9 {
            for j in 0..9 {
                if !GameBoard::check(self.game_board.numbers[i][j], i, j, &self.game_board.numbers).expect("Error") 
                    && self.game_board.number_state[i][j] != Condition::PreDetermined
                    && self.game_board.numbers[i][j] != 0
                {
                    self.game_board.number_state[i][j] = Condition::Wrong;
                } 
                else if self.game_board.number_state[i][j] == Condition::PreDetermined {
                    self.game_board.number_state[i][j] = Condition::PreDetermined;
                }
                else {
                    self.game_board.number_state[i][j] = Condition::Neutral;
                }
            }
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

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: &MouseButton, point: &Point2<f32>) -> GameResult {
        if *button == MouseButton::Left {
            for i in 0..10 {
                if self.number_board.rect[i].contains(*point) {
                    self.number_selection = i;
                }
            }

            for i in 0..9 {
                for j in 0..9 {
                    if self.game_board.grid_rect[i][j].contains(*point) 
                        && self.game_board.number_state[i][j] != Condition::PreDetermined
                    {
                        // if !GameBoard::check(self.number_selection, i, j, &self.game_board.numbers).expect("Error") {
                        //     self.game_board.numbers[i][j] = self.number_selection;
                        // }
                        self.game_board.numbers[i][j] = self.number_selection;
                        self.update_state();
                    }
                }
            }
        }
        Ok(())
    }
}