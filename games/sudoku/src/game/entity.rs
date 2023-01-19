use ggez::{
    glam::Vec2,
    Context, GameResult, 
    graphics::{ self, Mesh, Rect, Text }
};

const GRID_DIMENSION: (f32, f32) = (40., 40.);

#[derive(Clone, Copy, PartialEq)]
pub enum Condition {
    PreDetermined,
    Neutral,
    Wrong,
}

pub enum Difficulty {
    None,
    Easy,
    Intermediate,
    Hard,
}

pub struct GameBoard {
    pub grid_rect: [[Rect; 9]; 9],
    pub numbers: [[usize; 9]; 9],
    pub number_state: [[Condition; 9]; 9],
    grid_mesh: Mesh,
    region_mesh: Mesh,
    number_draw: [Text; 10],
}

impl GameBoard {
    pub fn init(ctx: &Context, difficulty: &Difficulty) -> GameBoard {
        let mut grid_rect = [[Rect::default(); 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                grid_rect[i][j] = Rect::new(
                    180. + (i%3) as f32 * (GRID_DIMENSION.0 * 3.) + (j%3) as f32 * GRID_DIMENSION.0,
                    60. + (i/3) as f32 * (GRID_DIMENSION.1 * 3.) + (j/3) as f32 * GRID_DIMENSION.1,
                    GRID_DIMENSION.0,
                    GRID_DIMENSION.1
                );
            }
        }
        let grid_mesh = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(1.)
                .with_line_join(graphics::LineJoin::Bevel)
             ), 
            Rect {
                x: 0.,
                y: 0.,
                w: GRID_DIMENSION.0,
                h: GRID_DIMENSION.1,
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
                w: GRID_DIMENSION.0 * 3.,
                h: GRID_DIMENSION.1 * 3.,
            }, 
            graphics::Color::WHITE,
        ).unwrap();

        // let numbers = [[0usize; 9]; 9];
        // let number_state = [[Condition::Neutral; 9]; 9];

        let (numbers, number_state) = GameBoard::define_numbers(difficulty);

        let mut number_draw: [Text; 10] = Default::default();

        number_draw[0] = Text::new(
            graphics::TextFragment::new("")
            // .color(graphics::Color::WHITE)
            .scale(0.)
        )
        .set_layout(graphics::TextLayout::center()).to_owned();

        for i in 1..10 {
            let number = i.to_string();
            number_draw[i] = Text::new(
                graphics::TextFragment::new(number)
                // .color(graphics::Color::WHITE)
                .scale(17.)
            )
            .set_layout(graphics::TextLayout::center()).to_owned();
        }

        GameBoard {
            grid_rect,
            grid_mesh,
            region_mesh,
            numbers,
            number_state,
            number_draw
        }
    }

    pub fn draw (&mut self, canvas: &mut graphics::Canvas) -> GameResult {
        for i in 0..9 {
            canvas.draw(
                &self.region_mesh, 
                graphics::DrawParam::default()
                    .dest(Vec2::new(
                    180. + (i%3) as f32 * (GRID_DIMENSION.0 * 3.),
                    60. + (i/3) as f32 * (GRID_DIMENSION.1 * 3.),
                ))
                .z(4)
            );
            for j in 0..9 {
                let (index, color) = match self.number_state[i][j] {
                    Condition::PreDetermined => (2, graphics::Color::GREEN),
                    Condition::Neutral => (1, graphics::Color::WHITE),
                    Condition::Wrong => (3, graphics::Color::RED),
                };
                canvas.draw(
                    &self.grid_mesh,
                    graphics::DrawParam::default()
                    .dest(Vec2::new(self.grid_rect[i][j].x, self.grid_rect[i][j].y))
                    .z(index).color(color)
                );

                canvas.draw(
                    &self.number_draw[self.numbers[i][j]],
                    graphics::DrawParam::default()
                    .dest(Vec2::new(
                        self.grid_rect[i][j].x + GRID_DIMENSION.0 / 2.,
                        self.grid_rect[i][j].y + GRID_DIMENSION.1 / 2.,
                    ))
                    .color(color)
                );
            }
        }
        Ok(())
    }

    fn define_numbers(difficulty: &Difficulty) -> ([[usize; 9]; 9],[[Condition; 9]; 9]) {
        let mut numbers = [[0usize; 9]; 9];
        let mut conditions = [[Condition::Neutral; 9]; 9];
        let number_determine: i32 = match difficulty {
            Difficulty::None => 0,
            Difficulty::Easy => 18,
            Difficulty::Intermediate => 11,
            Difficulty::Hard => 7,
        };
        let chance = 0..number_determine;
        let mut number_determined = 0;

        for i in 0..std::u32::MAX {
            let i: usize = (i % (9 * 9)) as usize;
            if number_determined >= number_determine { break; }
            let rand: usize = rand::random();

            if chance.contains(&((rand % (9*9)) as i32)) {
                numbers[i/9][i%9] = 1+rand%9;
                conditions[i/9][i%9] = Condition::PreDetermined;
                number_determined += 1;
            }
        }

        (numbers, conditions)
    }
}

pub struct NumberBoard {
    pub rect: [Rect; 10],
    pub mesh: Mesh,
    pub numbers: [Text; 10],
}

impl NumberBoard {
    pub fn init(ctx: &Context) -> Self {
        let mut rect = [Rect::default(); 10];
        for i in 0..10 {
            rect[i] = Rect::new(
                60. + (i%2) as f32 * GRID_DIMENSION.0, 
                60. + (i/2) as f32 * GRID_DIMENSION.1, 
                GRID_DIMENSION.0, 
                GRID_DIMENSION.1,
            );
        }

        let mesh = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(1.)
                .with_line_join(graphics::LineJoin::Bevel)
             ), 
            Rect::new(
                0.,
                0.,
                GRID_DIMENSION.0,
                GRID_DIMENSION.1,
            ), 
            graphics::Color::WHITE,
        ).unwrap();

        let mut numbers: [Text; 10] = Default::default();
        numbers[0] = Text::new(
            graphics::TextFragment::new("X")
            .color(graphics::Color::WHITE)
            .scale(17.)
        )
        .set_layout(graphics::TextLayout::center()).to_owned();
        for i in 1..10 {
            let number = i.to_string();
            numbers[i] = Text::new(
                graphics::TextFragment::new(number)
                .color(graphics::Color::WHITE)
                .scale(17.)
            )
            .set_layout(graphics::TextLayout::center()).to_owned();
        }

        NumberBoard { rect, mesh, numbers }
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas) -> GameResult {
        for i in 0..self.rect.len() {
            canvas.draw(&self.mesh, Vec2::new(self.rect[i].x, self.rect[i].y));

            canvas.draw(
                &self.numbers[i],
                Vec2::new(
                    self.rect[i].x + GRID_DIMENSION.0 / 2.,
                    self.rect[i].y + GRID_DIMENSION.1 / 2.,
                ),
            ); 
        }
        Ok(())
    }
}
