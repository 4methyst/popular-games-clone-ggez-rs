use ggez::{
    glam::Vec2,
    graphics::{self, Canvas, DrawParam, Mesh, Rect, Text, TextFragment},
    timer::TimeContext,
    Context,
};

use std::time::Duration;

pub struct TimeUI {
    pub time: TimeContext,
    mesh: Text,
}

impl TimeUI {
    pub fn new() -> Self {
        let time = TimeContext::new();
        let mesh = Text::new("Time: ")
            .add(TimeUI::format_common(&time.time_since_start()))
            .set_scale(20.)
            .to_owned();
        TimeUI { time, mesh }
    }

    pub fn format_common(time: &Duration) -> String {
        let secs = time.as_secs();
        let milli = &time.subsec_millis().to_string()[0..1];

        (secs / 60).to_string() + ":" + &(secs % 60).to_string() + "." + milli
    }

    pub fn update(&mut self) {
        self.mesh.fragments_mut()[1] =
            TextFragment::new(TimeUI::format_common(&self.time.time_since_start()));
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, Vec2::new(20., 20.));
    }
}

pub struct Button {
    pub rect: Rect,
    pub mesh: Mesh,
    pub text: Text,
}

impl Button {
    pub fn new(ctx: &Context, rect: Rect, text: Text) -> Self {
        let mesh = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::Stroke(
                graphics::StrokeOptions::default()
                    .with_line_width(1.)
                    .with_line_join(graphics::LineJoin::Bevel),
            ),
            rect,
            graphics::Color::WHITE,
        )
        .unwrap();

        Button { rect, mesh, text }
    }
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default());
        canvas.draw(
            &self.text,
            Vec2::new(
                self.rect.x + self.rect.w / 2.,
                self.rect.y + self.rect.h / 2.,
            ),
        );
    }
}
