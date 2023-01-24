use ggez::{
    Context, GameResult,
    glam::Vec2,
    graphics::{self, Text, Mesh},
};
use std::{fs, collections::BTreeMap};
use ron::de;
use crate::game::{
    ui::Button,
    game_states::*,
    entity::Score,
    constants::SCREEN_SIZE,
};

pub struct LeaderBoard {
    scores: Vec<Score>,
    texts: BTreeMap<&'static str, Text>,
    back_button: Button,
    background: Mesh,
    change_state: Option<GameState>,
}

impl LeaderBoard {
    pub fn new(ctx: &Context) -> Self {
        let serialized = fs::read_to_string("./games/sudoku/saves/scores.ron").unwrap();
        let scores: Vec<Score> = de::from_str(&serialized).unwrap();
        let mut texts = BTreeMap::new();
        texts.insert(
            "0_Title", 
            Text::new(
                graphics::TextFragment::new("LEADERBOARD")
                .color(graphics::Color::WHITE)
                .scale(50.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        texts.insert(
            "1_Author", 
            Text::new(
                graphics::TextFragment::new("Made by alimulap")
                .color(graphics::Color::WHITE)
                .scale(15.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
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
            graphics::Rect::new(30., 420., 80., 30.),
            graphics::Text::new(
                graphics::TextFragment::new("Back")
                .color(graphics::Color::WHITE)
                .scale(20.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        LeaderBoard {
            scores,
            texts,
            back_button,
            background,
            change_state: None,
        }
    }
}

impl StateTrait for LeaderBoard {
    fn update(&mut self, _ctx: &Context, _addon_ctx: &mut crate::game::context::AddOnContext) -> GameResult<Option<GameState>> {
        if let Some(new_state) = self.change_state {
            return Ok(Some(new_state));
        }
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut ggez::graphics::Canvas) -> GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());
        for (key, text) in self.texts.iter() {
            match *key {
                "0_Title" => canvas.draw(text, Vec2::new(360., 50.)),
                "1_Author" => canvas.draw(text, Vec2::new(640., 450.)),
                _ => (),
            }
        }
        for i in 0..self.scores.len() {
            canvas.draw(
                &Text::new((i+1).to_string()).add(". ")
                .add(self.scores[i].name.clone()).add(" ")
                .add(self.scores[i].difficulty).add(" ")
                .add(self.scores[i].time.as_secs().to_string()).add(".")
                .add(self.scores[i].time.subsec_millis().to_string()).to_owned(),
                Vec2::new(30., 20. * i as f32 + 100.)
            );
        }
        self.back_button.draw(canvas);
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: &ggez::event::MouseButton, point: &ggez::mint::Point2<f32>) -> GameResult {
        if self.back_button.rect.contains(*point) {
            self.change_state = Some(GameState::MainMenu);
        }
        Ok(())
    }
}