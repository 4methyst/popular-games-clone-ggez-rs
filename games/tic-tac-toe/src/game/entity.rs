use ggez::{ glam::Vec2, graphics::{Mesh, Rect, Canvas } };

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    None,
    P1,
    P2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Sign {
    None,
    X,
    O,
}

pub struct Board {
    pub sign: Vec<Sign>,
    pub grid: Mesh,
    pub rect: Vec<Rect>,
    pub sign_x: [Mesh; 4],
    pub sign_o: [Mesh; 1],
}

impl Board {
    pub fn draw(&mut self, canvas: &mut Canvas) {
        for i in 0..self.rect.len() {
            canvas.draw(&self.grid, Vec2::new(self.rect[i].x, self.rect[i].y));
            match self.sign[i] {
                Sign::X => {
                    for j in 0..4 {
                        canvas.draw(
                            &self.sign_x[j], 
                            Vec2::new(
                                self.rect[i].w/2. + self.rect[i].x,
                                self.rect[i].h/2. + self.rect[i].y
                            )
                        );
                    }
                },
                Sign::O => {
                    canvas.draw(
                        &self.sign_o[0], 
                        Vec2::new(
                            self.rect[i].w/2. + self.rect[i].x,
                            self.rect[i].h/2. + self.rect[i].y
                        )
                    );
                },
                Sign::None => {},
            }
        }
    }
}