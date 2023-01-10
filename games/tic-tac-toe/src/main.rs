use std::collections::BTreeMap;

use ggez::{
    event, glam::Vec2, mint::Point2,
    graphics::{self, Rect, DrawMode, Color, Text as ggezText, TextFragment},
    input::keyboard::{KeyCode, KeyInput},
    input::mouse::MouseButton,
    Context, GameResult,
};

const DESIRED_FPS: u32 = 30;

const SCREEN_SIZE: (f32, f32) = (720., 480.);

const BOARD_DIMENSION: (f32, f32) = (240., 240.);

const GRID_SIZE: (usize, usize) = (3, 3);

const GRID_DIMENSION: (f32, f32) = (BOARD_DIMENSION.0 / GRID_SIZE.0 as f32, BOARD_DIMENSION.1 / GRID_SIZE.1 as f32);

mod amst {
    pub struct Text {
        pub text: ggez::graphics::Text,
        pub pos: ggez::glam::Vec2
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Sign {
    None,
    X,
    O,
}
struct Board {
    rect: Vec<Rect>,
    sign: Vec<Sign>,
    grid: graphics::Mesh,
    sign_x: [graphics::Mesh; 4],
    sign_o: [graphics::Mesh; 1]
}

impl Board {
    fn draw(&mut self, canvas: &mut graphics::Canvas) {
        for i in 0..self.rect.len() {
            canvas.draw(&self.grid, Vec2::new(self.rect[i].x, self.rect[i].y));
            match self.sign[i] {
                Sign::X => {
                    for j in 0..4 {
                        canvas.draw(
                            &self.sign_x[j], 
                            Vec2::new(
                                self.rect[i].w/2. + self.rect[i].x,
                                self.rect[i].h/2. + self.rect[i].y
                            )
                        );
                    }
                },
                Sign::O => {
                    canvas.draw(
                        &self.sign_o[0], 
                        Vec2::new(
                            self.rect[i].w/2. + self.rect[i].x,
                            self.rect[i].h/2. + self.rect[i].y
                        )
                    );
                },
                Sign::None => {},
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Player {
    None,
    P1,
    P2,
}

struct MainState {
    board: Board,
    player: Player,
    winner: Player,
    text_map: BTreeMap<&'static str, amst::Text>,
    gameover: bool,
}

impl MainState {
    fn new(ctx: &Context) -> Self {

        MainState {
            board: MainState::init_board(&ctx),
            player: Player::P1,
            winner: Player::None,
            text_map: MainState::init_text(),
            gameover: false,
        }
    }

    fn init_board(ctx: &Context) -> Board {
        
        let mut rect = vec![Rect::default(); GRID_SIZE.0 * GRID_SIZE.1];
        for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
            rect[i] = Rect {
                x: 240. + (i % GRID_SIZE.0) as f32 * GRID_DIMENSION.0,
                y: 135. + (i / GRID_SIZE.0) as f32 * GRID_DIMENSION.1,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            };
        }
        let grid = graphics::Mesh::new_rectangle(
            ctx, DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(6.)
                .with_line_cap(graphics::LineCap::Square)
                .with_line_join(graphics::LineJoin::Bevel)
            ),
            Rect {
                x: 0.,
                y: 0.,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            }, Color::WHITE).unwrap();
        
        let sign = vec![Sign::None; GRID_SIZE.0 * GRID_SIZE.1];
        let mut sign_x = Vec::new();
        for j in 0..4 {
            let points = [
                Point2 { x: 0., y: 0. },
                Point2 {
                    x: 0. + (45. + 90. * j as f32).to_radians().cos() * ((GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 / 4.),
                    y: 0. + (45. + 90. * j as f32).to_radians().sin() * ((GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 / 4.),
                }
            ];
            sign_x.push(graphics::Mesh::new_line(ctx, &points, 6., Color::WHITE).unwrap());
        }
        let sign_x: [graphics::Mesh; 4] = [
            sign_x[0].clone(),
            sign_x[1].clone(),
            sign_x[2].clone(),
            sign_x[3].clone(),
        ];
        let sign_o = [
            graphics::Mesh::new_circle(
                ctx, 
                DrawMode::Stroke(
                    graphics::StrokeOptions::default()
                    .with_line_width(6.)
                ), 
                Point2 { x: 0., y: 0. }, 
                (GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 * 3. / 16., 
                0.1, 
                Color::WHITE,
            ).unwrap()
        ];

        Board {
            rect,
            sign,
            grid,
            sign_x,
            sign_o,
        }
    }

    fn init_text() -> BTreeMap<&'static str, amst::Text> {
        

        let mut text_map = BTreeMap::new();
        let text = {
            amst::Text {
                text: ggezText::new(
                        TextFragment::new("Tic Tac Toe")
                        .color(Color::WHITE)
                        .scale(50.)
                    )
                    .set_layout(graphics::TextLayout::center())
                    .to_owned(),
                pos: Vec2 { x: 360., y: 55. }
            }
        };
        text_map.insert("0_Title", text);

        let text = {
            amst::Text {
                text: ggezText::new(
                        TextFragment::new("Turn: Player 1(X)")
                        .color(Color::WHITE)
                        .scale(15.)
                    )
                    .set_layout(graphics::TextLayout::center())
                    .to_owned(),
                pos: Vec2 { x: 360., y: 95. }
            }
        };
        text_map.insert("1_Turn", text);

        let text = {
            amst::Text {
                text: ggezText::new(
                        TextFragment::new("")
                        .color(Color::WHITE)
                        .scale(35.)
                    )
                    .set_layout(graphics::TextLayout::center())
                    .to_owned(),
                pos: Vec2 { x: 525., y: 260. }
            }
        };
        text_map.insert("2_Winner", text);
        text_map
    }

    fn state_update(&mut self) {
        self.player = 
        if self.player == Player::P1 { 
            self.text_map.get_mut("1_Turn").unwrap().text.fragments_mut()[0].text
                = String::from("Turn: Player 2(O)");
            Player::P2 
        } else { 
            self.text_map.get_mut("1_Turn").unwrap().text.fragments_mut()[0].text
                = String::from("Turn: Player 1(X)");
            Player::P1 
        };

        self.winner = self.winner();
        match self.winner {
            Player::None => {},
            Player::P1 => {
                self.text_map.get_mut("2_Winner").unwrap().text.fragments_mut()[0].text
                    = String::from("PLAYER 1\nWON");
                self.gameover();
            },
            Player::P2 => {
                self.text_map.get_mut("2_Winner").unwrap().text.fragments_mut()[0].text
                    = String::from("PLAYER 2\nWON");
                self.gameover();
            },
        }
    }

    fn winner(&mut self) -> Player {
        let mut winner = Player::None;
        let players = [Player::P1, Player::P2];
        let signs = [Sign::X, Sign::O];

        for i in 0..players.len() {
            //Check vertically
            for j in 0..(GRID_SIZE.0 * GRID_SIZE.1 - (GRID_SIZE.0 * 2)) {
                if self.board.sign[j] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 1 + j] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 2 + j] == signs[i]
                {
                    winner = players[i];
                }
            }
            //Check Horizontally
            for j in 0..GRID_SIZE.1 {
                for k in 0..(GRID_SIZE.0 - 2) {
                    if self.board.sign[(j * 3 + k)..(j * 3 + k + 3)] == [signs[i],signs[i],signs[i]] {
                        winner = players[i];
                    }
                }
            }
            //Check diagonally upper left
            for j in 0..(GRID_SIZE.0 * GRID_SIZE.1 - (GRID_SIZE.0 * 2)) {
                if (j%GRID_SIZE.0) + 2 >= GRID_SIZE.0 { continue; }
                if self.board.sign[j] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 1 + j + 1] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 2 + j + 2] == signs[i]
                {
                    winner = players[i];
                }
            }
            //Check diagonally upper right
            for j in 0..(GRID_SIZE.0 * GRID_SIZE.1 - (GRID_SIZE.0 * 2)) {
                if (j%GRID_SIZE.0) as i32 - 2 < 0 { continue; }
                if self.board.sign[j] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 1 + j - 1] == signs[i]
                    && self.board.sign[GRID_SIZE.0 * 2 + j - 2] == signs[i]
                {
                    winner = players[i];
                }
            }
        }
        winner
    }

    fn gameover(&mut self) {
        self.gameover = true;
        for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
            self.board.rect[i] = Rect {
                x: 120. + (i % GRID_SIZE.0) as f32 * GRID_DIMENSION.0,
                y: 135. + (i / GRID_SIZE.0) as f32 * GRID_DIMENSION.1,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            };
        }
    }

    fn restart(&mut self) {
        self.gameover = false;
        self.winner = Player::None;
        for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
            self.board.rect[i] = Rect {
                x: 240. + (i % GRID_SIZE.0) as f32 * GRID_DIMENSION.0,
                y: 140. + (i / GRID_SIZE.0) as f32 * GRID_DIMENSION.1,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            };
        }
        self.board.sign = vec![Sign::None; GRID_SIZE.0 * GRID_SIZE.1];
        self.text_map.get_mut("2_Winner").unwrap().text.fragments_mut()[0].text
            = String::from("");
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(DESIRED_FPS) {
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        self.board.draw(&mut canvas);
        
        for (key, value) in self.text_map.iter() {
            if self.gameover && *key == "1_Turn" { continue; }
            canvas.draw(&value.text, value.pos);
        }

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if !self.gameover {
            for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
                if self.board.rect[i].contains(Point2 { x, y } ) 
                    && ctx.mouse.button_just_pressed(button) 
                    && self.board.sign[i] == Sign::None
                {
                    self.board.sign[i] = match &self.player {
                        Player::P1 => Sign::X,
                        Player::P2 => Sign::O,
                        Player::None => Sign::None,
                    };
                    self.state_update();
                }
            }
        }
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeated: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Escape) => ctx.request_quit(),
            Some(KeyCode::R) => self.restart(),
            _ => {},
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = 
        ggez::ContextBuilder::new(
            "PlayGround", 
            "4methyst")
        .window_setup(
            ggez::conf::WindowSetup::default()
            .title("PlayGround"))
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = MainState::new(&ctx);
    event::run(ctx, events_loop, state)
}