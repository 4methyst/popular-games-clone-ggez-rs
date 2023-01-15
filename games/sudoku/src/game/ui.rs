use ggez::{
    glam::Vec2,
    graphics::{ Mesh, Rect, Canvas, DrawParam, Text }
};

pub struct Button {
    pub rect: Rect,
    pub mesh: Mesh,
    pub text: Text,
}

impl Button {
    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.mesh, DrawParam::default());
        canvas.draw(&self.text, Vec2::new(self.rect.x + self.rect.w/2., self.rect.y + self.rect.h/2.));
    }
}