use ggez::{ glam::Vec2, graphics::{ Mesh, Rect, Text as ggText, Canvas, DrawParam }};

pub struct Text {
    pub text: ggText,
    pub pos: Vec2
}

#[derive(Clone)]
pub struct Button {
    pub rect: Rect,
    pub mesh: Mesh,
    pub text: ggText,
}

impl Button {
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default());
        canvas.draw(&self.text, Vec2::new(self.rect.x + self.rect.w/2., self.rect.y + self.rect.h/2.));
    }
}