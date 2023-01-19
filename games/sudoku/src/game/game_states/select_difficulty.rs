use std::collections::BTreeMap;

use ggez::{
    glam::Vec2,
    GameResult, Context,
    graphics::{ self, Text, Rect, Color },
};

use crate::game::{
    ui::*,
    constants::*,
    game_states::*,
    entity::Difficulty,
};

pub struct SelectDifficulty {
    texts: BTreeMap<&'static str, Text>,
    buttons: BTreeMap<&'static str, Button>,
    background: graphics::Mesh,
    change_state: Option<GameState>,
    selected_difficulty: Option<Difficulty>,
}

impl SelectDifficulty {
    pub fn new(ctx: &Context) -> Self {
        let mut texts = BTreeMap::new();
        texts.insert(
            "0_Title", 
            Text::new(
                graphics::TextFragment::new("Select Difficulty")
                .color(Color::WHITE)
                .scale(80.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        texts.insert(
            "1_Author", 
            Text::new(
                graphics::TextFragment::new("Made by alimulap")
                .color(Color::WHITE)
                .scale(15.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        let mut buttons = BTreeMap::new();
        buttons.insert(
            "0_None",
            Button::new(
                &ctx,
                Rect::new(320., 200., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("None")
                    .color(Color::WHITE)
                    .scale(20.)
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned()
            )
        );
        buttons.insert(
            "1_Easy",
            Button::new(
                &ctx,
                Rect::new(320., 240., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("Easy")
                    .color(Color::WHITE)
                    .scale(20.)
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned()
            )
        );
        buttons.insert(
            "2_Intermediate",
            Button::new(
                &ctx,
                Rect::new(320., 280., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("Intermediate")
                    .color(Color::WHITE)
                    .scale(20.)
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned()
            )
        );
        buttons.insert(
            "3_Hard",
            Button::new(
                &ctx,
                Rect::new(320., 320., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("Hard")
                    .color(Color::WHITE)
                    .scale(20.)
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned()
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
        let background = graphics::Mesh::from_data(ctx, graphics::MeshData {
            vertices: &vertices,
            indices: &indices,
        });
        SelectDifficulty {
            texts,
            buttons,
            background,
            change_state: None,
            selected_difficulty: None,
        }
    }
}

impl StateTrait for SelectDifficulty {
    fn update(&mut self, _ctx: &Context, addon_ctx: &mut AddOnContext) -> GameResult<Option<GameState>> {
        if let Some(new_state) = self.change_state.clone() {
            self.change_state = None;
            addon_ctx.difficulty = self.selected_difficulty;
            return Ok(Some(new_state));
        }
        Ok(None)
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {

        canvas.draw(&self.background, graphics::DrawParam::default());

        for (_key, button) in self.buttons.iter_mut() {
            button.draw(canvas);
        }

        for (key, text) in self.texts.iter() {
            match *key {
                "0_Title" => canvas.draw(text, Vec2::new(360., 100.)),
                "1_Author" => canvas.draw(text, Vec2::new(640., 450.)),
                _ => (),
            }
        }

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: &MouseButton, point: &Point2<f32>) -> GameResult {
        for (key, buttonui) in self.buttons.iter_mut() {
            if buttonui.rect.contains(*point) && *button == MouseButton::Left {
                match *key {
                    "0_None" => {
                        self.selected_difficulty = Some(Difficulty::None);
                        self.change_state = Some(GameState::Playing);
                    },
                    "1_Easy" => {
                        self.selected_difficulty = Some(Difficulty::Easy);
                        self.change_state = Some(GameState::Playing);
                    },
                    "2_Intermediate" => {
                        self.selected_difficulty = Some(Difficulty::Intermediate);
                        self.change_state = Some(GameState::Playing);
                    },
                    "3_Hard" => {
                        self.selected_difficulty = Some(Difficulty::Hard);
                        self.change_state = Some(GameState::Playing);
                    },
                    // "1_Exit" => ctx.request_quit(),
                    _ => (),
                }
            }
        }
        Ok(())
    }
}