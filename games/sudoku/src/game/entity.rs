use ggez::{
    glam::Vec2,
    Context, GameResult, 
    graphics::{ self, Mesh, Rect, Text }
};

const GRID_DIMENSION: (f32, f32) = (30., 30.);

pub struct Board {
    grid_rect: [[Rect; 9]; 9],
    grid_mesh: Mesh,
    region_mesh: Mesh,
    numbers: [[usize; 9]; 9],
    number_draw: [Text; 10],
}

impl Board {
    pub fn init(ctx: &Context) -> Board {
        let mut grid_rect = [[Rect::default(); 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                grid_rect[i][j] = Rect::new(
                    225. + (i%3) as f32 * (GRID_DIMENSION.0 * 3.) + (j%3) as f32 * GRID_DIMENSION.0,
                    105. + (i/3) as f32 * (GRID_DIMENSION.1 * 3.) + (j/3) as f32 * GRID_DIMENSION.1,
                    GRID_DIMENSION.0,
                    GRID_DIMENSION.1
                );
            }
        }
        let grid_mesh = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(2.)
                .with_line_join(graphics::LineJoin::Bevel)
             ), 
            Rect {
                x: 0.,
                y: 0.,
                w: 30.,
                h: 30.,
            }, 
            graphics::Color::WHITE,
        ).unwrap();
        let region_mesh = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(3.)
                .with_line_join(graphics::LineJoin::Bevel)
             ), 
            Rect {
                x: 0.,
                y: 0.,
                w: 90.,
                h: 90.,
            }, 
            graphics::Color::WHITE,
        ).unwrap();

        let numbers = [[0usize; 9]; 9];

        let mut number_draw: [Text; 10] = Default::default();

        number_draw[0] = Text::new(
            graphics::TextFragment::new("")
            .color(graphics::Color::WHITE)
            .scale(0.)
        )
        .set_layout(graphics::TextLayout::center()).to_owned();

        for i in 0..10 {
            let number = i.to_string();
            number_draw[i] = Text::new(
                graphics::TextFragment::new(number)
                .color(graphics::Color::WHITE)
                .scale(15.)
            )
            .set_layout(graphics::TextLayout::center()).to_owned();
        }

        Board {
            grid_rect,
            grid_mesh,
            region_mesh,
            numbers,
            number_draw
        }
    }

    pub fn draw (&mut self, canvas: &mut graphics::Canvas) -> GameResult {
        for i in 0..9 {
            canvas.draw(
                &self.region_mesh, 
                Vec2::new(
                    (i%3) as f32 * (GRID_DIMENSION.0 * 3.),
                    (i/3) as f32 * (GRID_DIMENSION.1 * 3.),
                )
            );
            for j in 0..9 {
                canvas.draw(
                    &self.grid_mesh,
                    Vec2::new(self.grid_rect[i][j].x, self.grid_rect[i][j].y),
                );

                canvas.draw(
                    &self.number_draw[self.numbers[i][j]],
                    Vec2::new(
                        (i%3) as f32 * (GRID_DIMENSION.0 * 3.) + ((j%3) as f32 + 0.5) * GRID_DIMENSION.0,
                        (i/3) as f32 * (GRID_DIMENSION.1 * 3.) + ((j/3) as f32 + 0.5) * GRID_DIMENSION.1,
                    )
                );
            }
        }
        Ok(())
    }
}
