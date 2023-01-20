// use std::time::Duration;

use ggez::{
    Context, GameResult,
    graphics,
    timer::TimeContext,
};

use crate::game::{
    constants::*,
    entity::*,
    game_states::*,
    context,
};

pub struct Playing {
    game_board: GameBoard,
    number_board: NumberBoard,
    background: graphics::Mesh,
    number_selection: usize,
    // time_start: Duration,
    time: TimeContext,
}

impl Playing {
    pub fn new(ctx: &Context, addon_ctx: &context::AddOnContext) -> Self {
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
        // let time = TimeContext::new();
        Playing {
            game_board: GameBoard::init(&ctx, &addon_ctx.difficulty.unwrap()),
            number_board: NumberBoard::init(&ctx),
            background,
            number_selection: 0,
            // time_start: Duration::new(0, 0),
            time: TimeContext::new(),
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
    fn update(&mut self, _ctx: &Context, _addon_ctx: &mut AddOnContext) -> GameResult<Option<GameState>> {
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());
        self.game_board.draw(canvas)?;
        self.number_board.draw(canvas)?;

        canvas.draw(&graphics::Text::new(
            self.time.time_since_start().as_secs().to_string())
                .set_scale(20.).to_owned(), 
            ggez::glam::vec2(10., 10.)
        );

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: &MouseButton, point: &Point2<f32>) -> GameResult {
        if *button == MouseButton::Left {
            for i in 0..10 {
                if self.number_board.rect[i].contains(*point) {
                    self.number_selection = i;
                    self.number_board.number_selection = self.number_selection;
                }
            }

            for i in 0..9 {
                for j in 0..9 {
                    if self.game_board.grid_rect[i][j].contains(*point) 
                        && self.game_board.number_state[i][j] != Condition::PreDetermined
                    {
                        self.game_board.numbers[i][j] = self.number_selection;
                        self.update_state();
                    }
                }
            }
        }
        if *button == MouseButton::Right {
            for i in 0..9 {
                for j in 0..9 {
                    if self.game_board.grid_rect[i][j].contains(*point) 
                        && self.game_board.number_state[i][j] != Condition::PreDetermined
                    {
                        self.game_board.numbers[i][j] = 0;
                        self.update_state();
                    }
                }
            }
        }
        Ok(())
    }
}