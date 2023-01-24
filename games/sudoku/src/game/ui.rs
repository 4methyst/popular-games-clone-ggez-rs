use ggez::{
    glam::Vec2, Context,
    graphics::{ self, Mesh, Rect, Canvas, DrawParam, Text, TextFragment },
    timer::TimeContext,
};

pub struct TimeUI {
    pub time: TimeContext,
    mesh: Text,
}

impl TimeUI {
    pub fn new() -> Self {
        let time = TimeContext::new();
        let mesh = Text::new("Time: ")
            .add(time.time_since_start().as_secs().to_string())
            .add(".")
            .add(time.time_since_start().subsec_millis().to_string())
            .set_scale(20.).to_owned();
        TimeUI {
            time,
            mesh
        }
    }

    pub fn update(&mut self) {
        self.mesh.fragments_mut()[1] = TextFragment::new(self.time.time_since_start().as_secs().to_string());
        self.mesh.fragments_mut()[3] = TextFragment::new(self.time.time_since_start().subsec_millis().to_string());
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
                .with_line_join(graphics::LineJoin::Bevel)
            ),  
            rect, 
            graphics::Color::WHITE
        ).unwrap();

        Button {
            rect,
            mesh,
            text,
        }
    }
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default());
        canvas.draw(&self.text, Vec2::new(self.rect.x + self.rect.w/2., self.rect.y + self.rect.h/2.));
    }
}