use ggez::{
    glam::Vec2, Context,
    graphics::{ self, Mesh, Rect, Canvas, DrawParam, Text }
};

pub struct Button {
    pub rect: Rect,
    pub mesh: Mesh,
    pub text: Text,
}

impl Button {
    fn new(ctx: &Context, rect: Rect, text: Text) -> Self {
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