use std::collections::BTreeMap;

use ggez::{
    GameResult, Context,
    graphics::{ self, Text, Rect, Color },
};

use crate::game::{
    ui::*,
    constants::*,
};
use super::StateTrait;

pub struct MainMenu {
    texts: BTreeMap<&'static str, Text>,
    buttons: BTreeMap<&'static str, Button>,
    background: graphics::Mesh,
}

impl MainMenu {
    pub fn new(ctx: &Context) -> Self {
        let mut texts = BTreeMap::new();
        texts.insert(
            "0_Title", 
            Text::new(
                graphics::TextFragment::new("SUDOKU")
                .color(Color::WHITE)
                .scale(50.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        texts.insert(
            "1_Author", 
            Text::new(
                graphics::TextFragment::new("Made by alimulap")
                .color(Color::WHITE)
                .scale(20.)
            )
            .set_layout(graphics::TextLayout::center())
            .to_owned()
        );
        let mut buttons = BTreeMap::new();
        buttons.insert(
            "0_Play",
            Button::new(
                &ctx,
                Rect::new(320., 200., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("PLAY")
                    .color(Color::WHITE)
                    .scale(20.)
                )
                .set_layout(graphics::TextLayout::center())
                .to_owned()
            )
        );
        buttons.insert(
            "1_Exit",
            Button::new(
                &ctx,
                Rect::new(320., 250., 80., 30.),
                Text::new(
                    graphics::TextFragment::new("EXIT")
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
        MainMenu {
            texts,
            buttons,
            background,
        }
    }
}

impl StateTrait for MainMenu {
    fn update(&mut self, _ctx: &Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        // self.buttons.get_mut("0_Play").unwrap().draw(canvas);
        // self.buttons.get_mut("1_Exit").unwrap().draw(canvas);

        canvas.draw(&self.background, graphics::DrawParam::default());

        for (_key, button) in self.buttons.iter_mut() {
            button.draw(canvas);
        }

        for (_key, text) in self.texts.iter() {
            canvas.draw(text, graphics::DrawParam::default());
        }

        Ok(())
    }
}