use ggez::{ Context, mint::Point2, glam::Vec2, graphics::{self, Mesh, Rect, Color, Canvas, DrawMode } };

use crate::game::constant::*;

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
    pub fn init(ctx: &Context) -> Board {
        let mut rect = vec![Rect::default(); GRID_SIZE.0 * GRID_SIZE.1];
        for i in 0..GRID_SIZE.0 * GRID_SIZE.1 {
            rect[i] = Rect {
                x: 240. + (i % GRID_SIZE.0) as f32 * GRID_DIMENSION.0,
                y: 135. + (i / GRID_SIZE.0) as f32 * GRID_DIMENSION.1,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            };
        }
        let grid = graphics::Mesh::new_rectangle(
            ctx, DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(4.)
                .with_line_cap(graphics::LineCap::Square)
                .with_line_join(graphics::LineJoin::Bevel)
            ),
            Rect {
                x: 0.,
                y: 0.,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
            }, Color::WHITE).unwrap();
        
        let sign = vec![Sign::None; GRID_SIZE.0 * GRID_SIZE.1];
        let mut sign_x = Vec::new();
        for j in 0..4 {
            let points = [
                Point2 { x: 0., y: 0. },
                Point2 {
                    x: 0. + (45. + 90. * j as f32).to_radians().cos() * ((GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 / 4.),
                    y: 0. + (45. + 90. * j as f32).to_radians().sin() * ((GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 / 4.),
                }
            ];
            sign_x.push(graphics::Mesh::new_line(ctx, &points, 4., Color::WHITE).unwrap());
        }
        let sign_x: [graphics::Mesh; 4] = [
            sign_x[0].clone(),
            sign_x[1].clone(),
            sign_x[2].clone(),
            sign_x[3].clone(),
        ];
        let sign_o = [
            graphics::Mesh::new_circle(
                ctx, 
                DrawMode::Stroke(
                    graphics::StrokeOptions::default()
                    .with_line_width(4.)
                ), 
                Point2 { x: 0., y: 0. }, 
                (GRID_DIMENSION.0 + GRID_DIMENSION.1) as f32 * 3. / 16., 
                0.1, 
                Color::WHITE,
            ).unwrap()
        ];

        Board {
            rect,
            sign,
            grid,
            sign_x,
            sign_o,
        }
    }

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