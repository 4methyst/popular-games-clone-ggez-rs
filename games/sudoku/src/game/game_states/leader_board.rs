use ggez::{
    Context,
    glam::Vec2,
    graphics::{self, Text, Mesh},
};
use std::fs;
use ron::de;
use crate::game::{
    game_states::StateTrait,
    entity::Score,
    constants::SCREEN_SIZE,
};

pub struct LeaderBoard {
    scores: Vec<Score>,
    background: Mesh,
}

impl LeaderBoard {
    pub fn new(ctx: &Context) -> Self {
        let serialized = fs::read_to_string("./games/sudoku/saves/scores.ron").unwrap();
        let scores: Vec<Score> = de::from_str(&serialized).unwrap();
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
        LeaderBoard {
            scores,
            background,
        }
    }
}

impl StateTrait for LeaderBoard {
    fn update(&mut self, _ctx: &Context, _addon_ctx: &mut crate::game::context::AddOnContext) -> ggez::GameResult<Option<super::GameState>> {
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut ggez::graphics::Canvas) -> ggez::GameResult {
        canvas.draw(&self.background, graphics::DrawParam::default());
        for i in 0..self.scores.len() {
            canvas.draw(
                &Text::new(i.to_string()).add(" ")
                .add(self.scores[i].name.clone()).add(" ")
                .add(self.scores[i].difficulty).add(" ")
                .add(self.scores[i].time.as_secs().to_string()).add(".")
                .add(self.scores[i].time.subsec_millis().to_string()).to_owned(),
                Vec2::new(30., 10. * i as f32 + 30.)
            );
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: &ggez::event::MouseButton, _point: &ggez::mint::Point2<f32>) -> ggez::GameResult {
        Ok(())
    }
}