use ggez::{
    Context, GameResult,
    graphics,
};

use crate::game::{
    constants::*,
    entity::*,
    game_states::*,
    ui::*,
    context,
};

use std::{fs, io::Write};
use ron::{ser, de};

pub struct Playing {
    game_board: GameBoard,
    number_board: NumberBoard,
    back_button: Button,
    background: graphics::Mesh,
    number_selection: u8,
    difficulty: Difficulty,
    time: TimeUI,
    scores: Vec<Score>,
    gameover: bool,
    change_state: Option<GameState>,
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

        let back_button = Button::new(
            &ctx,
            graphics::Rect::new(60., 390., 80., 30.),
            graphics::Text::new(
                graphics::TextFragment::new("Back")
                .color(graphics::Color::WHITE)
                .scale(20.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );

        let serialized = fs::read_to_string("./games/sudoku/saves/scores.ron").unwrap();
        let scores: Vec<Score> = de::from_str(&serialized).unwrap();
        
        Playing {
            game_board: GameBoard::init(&ctx, 180., 60., &addon_ctx.difficulty.unwrap()),
            number_board: NumberBoard::init(&ctx, 60., 60.),
            back_button,
            background,
            number_selection: 0,
            difficulty: addon_ctx.difficulty.unwrap(),
            time: TimeUI::new(),
            scores,
            gameover: false,
            change_state: None,
        }
    }

    fn update_state(&mut self) {
        let mut gameover = true;
        for i in 0..9 {
            for j in 0..9 {
                if !GameBoard::check_valid(self.game_board.numbers[i][j], i, j, &self.game_board.numbers)
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

                if gameover {
                    if self.game_board.number_state[i][j] == Condition::Wrong || self.game_board.numbers[i][j] == 0 {
                        gameover = false;
                    }
                }
            }
        }
        if gameover { self.gameover(); }
    }

    fn gameover(&mut self) {
        self.gameover = true;
        self.scores.push(Score::new("Something", self.difficulty, self.time.time.time_since_start()));
        self.scores.sort_by_key(|score| score.time.as_millis());

        let serialized = ser::to_string_pretty(
            &self.scores, 
            ser::PrettyConfig::default().struct_names(true)
        ).unwrap();
        fs::File::create("./games/sudoku/saves/scores.ron").unwrap().write_all(serialized.as_bytes()).unwrap();

        self.change_state = Some(GameState::LeaderBoard);
    }
}

impl StateTrait for Playing {
    fn update(&mut self, _ctx: &Context, _addon_ctx: &mut AddOnContext) -> GameResult<Option<GameState>> {
        if let Some(new_state) = self.change_state {
            return Ok(Some(new_state));
        }

        if self.gameover { return Ok(None); }

        self.update_state();
        self.time.update();
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());
        self.game_board.draw(canvas)?;
        self.number_board.draw(canvas)?;
        self.back_button.draw(canvas);
        self.time.draw(canvas);

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: &MouseButton, point: &Point2<f32>) -> GameResult {
        if *button == MouseButton::Left {
            if self.back_button.rect.contains(*point) {
                self.change_state = Some(GameState::SelectDifficulty);
            }

            if self.gameover { return Ok(()); }

            for i in 0..10 {
                if self.number_board.rect[i].contains(*point) {
                    self.number_selection = i as u8;
                    self.number_board.number_selection = self.number_selection;
                    self.game_board.number_selected = self.number_selection;
                }
            }

            for i in 0..9 {
                for j in 0..9 {
                    if self.game_board.grid_rect[i][j].contains(*point) 
                        && self.game_board.numbers[i][j] == 0
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