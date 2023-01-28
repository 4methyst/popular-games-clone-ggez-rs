use std::collections::BTreeMap;

use ggez::{
    event, glam::Vec2, mint::Point2,
    graphics::{self, Rect, Color, Text as ggText, TextFragment},
    input::keyboard::{KeyCode, KeyInput},
    input::mouse::MouseButton,
    Context, GameResult,
};

use crate::game::{entity::*, ui::*, constant::*};

pub struct MainState {
    board: Board,
    player: Player,
    winner: Player,
    text_map: BTreeMap<&'static str, Text>,
    gameover: bool,
    background: graphics::Mesh,
    buttons: BTreeMap<&'static str, Button>,
}

impl MainState { 
    pub fn new(ctx: &Context) -> Self {
        let mut buttons = BTreeMap::new();
        buttons.insert(
            "0_restart", 
            Button::new(
                &ctx, 
                Rect::new(240., 395., 80., 20.), 
                ggText::new(
                    TextFragment::new("(R) Restart")
                    .color(Color::WHITE)
                    .scale(12.)
                ).set_layout(graphics::TextLayout::center()).to_owned()
            )
        );
        buttons.insert(
            "1_quit", 
            Button::new(
                &ctx, 
                Rect::new(400., 395., 80., 20.), 
                ggText::new(
                    TextFragment::new("(Esc) Quit")
                    .color(Color::WHITE)
                    .scale(12.)
                ).set_layout(graphics::TextLayout::center()).to_owned()
            )
        );
        let vertices = [
            graphics::Vertex { position: [0., 0.], uv: [0., 0.], color: [0.001, 0., 0.001, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0, 0.], uv: [SCREEN_SIZE.0, 0.], color: [0., 0., 0.01, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0/2., SCREEN_SIZE.1/2.], uv: [SCREEN_SIZE.0/2., SCREEN_SIZE.1/2.], color: [0.015, 0., 0.02, 1.] },
            graphics::Vertex { position: [SCREEN_SIZE.0, SCREEN_SIZE.1], uv: [SCREEN_SIZE.0, SCREEN_SIZE.1], color: [0.001, 0., 0.001, 1.] },
            graphics::Vertex { position: [0., SCREEN_SIZE.1], uv: [0., SCREEN_SIZE.1], color: [0., 0., 0.01, 1.] },
        ];
        let indices = [0, 1, 2, 2, 1, 3, 3, 2, 4, 4, 2, 0];
        let background = graphics::Mesh::from_data(
            ctx, 
            graphics::MeshData { 
                vertices: &vertices, 
                indices: &indices 
            } 
        );

        MainState {
            board: Board::init(&ctx),
            player: Player::P1,
            winner: Player::None,
            text_map: Self::init_text(),
            gameover: false,
            background,
            buttons,
        }
    }

    fn init_text() -> BTreeMap<&'static str, Text> {
        let mut text_map = BTreeMap::new();
        let text = {
            Text {
                text: ggText::new(
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
            Text {
                text: ggText::new(
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
            Text {
                text: ggText::new(
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

        if !self.gameover && self.check_draw() {
            self.text_map.get_mut("2_Winner").unwrap().text.fragments_mut()[0].text
                    = String::from("DRAW");
            self.gameover();
        }
    }

    fn winner(&self) -> Player {
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

    fn check_draw(&self) -> bool {
        for i in self.board.sign.iter() {
            if *i == Sign::None {
                return false;
            }
        }
        return true;
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
        self.player = Player::P1;
        self.winner = Player::None;
        for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
            self.board.rect[i] = Rect {
                x: 240. + (i % GRID_SIZE.0) as f32 * GRID_DIMENSION.0,
                y: 135. + (i / GRID_SIZE.0) as f32 * GRID_DIMENSION.1,
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

        canvas.draw(&self.background, graphics::DrawParam::default());

        self.board.draw(&mut canvas);
        
        for (key, value) in self.text_map.iter() {
            if self.gameover && *key == "1_Turn" { continue; }
            canvas.draw(&value.text, value.pos);
        }

        self.buttons.iter_mut().for_each(|(_key, button)| button.draw(&mut canvas));

        for (_key, button) in self.buttons.iter_mut() {
            button.draw(&mut canvas);
        } 

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) -> GameResult {
        if !self.gameover {
            for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
                if self.board.rect[i].contains(Point2 { x, y } ) 
                    && button == MouseButton::Left 
                    && self.board.sign[i] == Sign::None
                {
                    self.board.sign[i] = match self.player {
                        Player::P1 => Sign::X,
                        Player::P2 => Sign::O,
                        Player::None => Sign::None,
                    };
                    self.state_update();
                    break;
                }
            }
        }

        for (key, button) in self.buttons.clone() {
            if button.rect.contains( Point2 { x, y } ) {
                match key {
                    "0_restart" => self.restart(),
                    "1_quit" => ctx.request_quit(),
                    _ => (),
                };
            };
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
