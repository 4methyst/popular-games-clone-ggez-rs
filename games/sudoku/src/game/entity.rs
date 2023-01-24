use ggez::{
    glam::Vec2,
    Context, GameResult, 
    graphics::{ self, Mesh, Rect, Text, TextFragment }
};
use std::time::Duration;
use serde::{Serialize, Deserialize};

const GRID_DIMENSION: (f32, f32) = (40., 40.);

#[derive(Clone, Copy, PartialEq)]
pub enum Condition {
    PreDetermined,
    Neutral,
    Wrong,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Difficulty {
    None,
    Easy,
    Intermediate,
    Hard,
}

impl Into<TextFragment> for Difficulty {
    fn into(self) -> TextFragment {
        match self {
            Difficulty::None => TextFragment::new("None"),
            Difficulty::Easy => TextFragment::new("Easy"),
            Difficulty::Intermediate => TextFragment::new("Intermediate"),
            Difficulty::Hard => TextFragment::new("Hard"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Score {
    pub name: String,
    pub difficulty: Difficulty,
    pub time: Duration,
}

impl Score {
    pub fn new(name: &str, difficulty: Difficulty, time: Duration) -> Self{
        Score {
            name: String::from(name), 
            difficulty, 
            time,
        }
    }
}

pub struct GameBoard {
    pub grid_rect: [[Rect; 9]; 9],
    pub numbers: [[usize; 9]; 9],
    pub number_state: [[Condition; 9]; 9],
    pub number_selected: u32,
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

        let (numbers, number_state) = GameBoard::define_numbers(difficulty);

        let mut number_draw: [Text; 10] = Default::default();

        number_draw[0] = Text::new(
            graphics::TextFragment::new("")
            .scale(0.)
        )
        .set_layout(graphics::TextLayout::center()).to_owned();

        for i in 1..10 {
            let number = i.to_string();
            number_draw[i] = Text::new(
                graphics::TextFragment::new(number)
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
            number_draw,
            number_selected: 0,
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

                if self.numbers[i][j] == self.number_selected as usize 
                    && self.numbers[i][j] != 0 {
                    canvas.draw(
                        &self.grid_mesh,
                        graphics::DrawParam::default()
                        .dest(Vec2::new(self.grid_rect[i][j].x, self.grid_rect[i][j].y))
                        .z(4).color(graphics::Color::CYAN)
                    );
                }

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
            if i >= 1000000 { break; }
            let i: usize = (i % (9 * 9)) as usize;
            if number_determined >= number_determine { break; }
            let rand: usize = rand::random();

            if chance.contains(&((rand % (9*9)) as i32)) 
                && GameBoard::check(1+rand%9, i/9, i%9, &numbers).unwrap() 
                && conditions[1/9][1%9] != Condition::PreDetermined {
                numbers[i/9][i%9] = 1+rand%9;
                conditions[i/9][i%9] = Condition::PreDetermined;
                number_determined += 1;
            }
        }

        (numbers, conditions)
    }

    /// true means fine, false means there is same number(s) horizontally, vertically, or in the same region
    pub fn check(number: usize, i: usize, j: usize, numbers: &[[usize; 9]; 9]) -> Result<bool, ()> {
        let pos_i_vertical: usize = match i {
            0 | 3 | 6 => 0,
            1 | 4 | 7 => 1,
            2 | 5 | 8 => 2,
            _ => 100,
        };
        let pos_i_horizontal: usize = match i {
            0..=2 => 0,
            3..=5 => 1,
            6..=8 => 2,
            _ => 100,
        };
        let pos_j_vertical: usize = match j {
            0 | 3 | 6 => 0,
            1 | 4 | 7 => 1,
            2 | 5 | 8 => 2,
            _ => 100,
        };
        let pos_j_horizontal: usize = match j {
            0..=2 => 0,
            3..=5 => 1,
            6..=8 => 2,
            _ => 100,
        };

        if pos_i_vertical == 100
            || pos_i_horizontal == 100
            || pos_j_vertical == 100
            || pos_j_horizontal == 100
        {
            return Err(());
        }

        for k in 0..9 {
            // check same region
            if number == numbers[i][k] && j != k {
                return Ok(false);
            }

            // check vertical
            if number == numbers[k / 3 * 3 + pos_i_vertical][k % 3 * 3 + pos_j_vertical] 
                && !(k / 3 * 3 + pos_i_vertical == i && k % 3 * 3 + pos_j_vertical == j)  
            {
                return Ok(false);
            } 

            // check horizontal
            if number == numbers[(pos_i_horizontal * 3) + (k / 3)][(pos_j_horizontal * 3) + (k % 3)] 
                && !((pos_i_horizontal * 3) + (k / 3) == i && (pos_j_horizontal * 3) + (k % 3) == j)
            {
                return Ok(false);
            }
        }
        return Ok(true);
    }
}

pub struct NumberBoard {
    pub rect: [Rect; 10],
    mesh: Mesh,
    mesh_selection: Mesh,
    numbers: [Text; 10],
    pub number_selection: usize,
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
        let mesh_selection = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(2.)
                .with_line_join(graphics::LineJoin::Bevel)
             ), 
            Rect::new(
                0.,
                0.,
                GRID_DIMENSION.0,
                GRID_DIMENSION.1,
            ), 
            graphics::Color::CYAN,
        ).unwrap();

        let mut numbers: [Text; 10] = Default::default();
        numbers[0] = Text::new(
            graphics::TextFragment::new("X")
            .scale(17.)
        )
        .set_layout(graphics::TextLayout::center()).to_owned();
        for i in 1..10 {
            let number = i.to_string();
            numbers[i] = Text::new(
                graphics::TextFragment::new(number)
                .scale(17.)
            )
            .set_layout(graphics::TextLayout::center()).to_owned();
        }

        NumberBoard { rect, mesh, mesh_selection, numbers, number_selection: 0 }
    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas) -> GameResult {
        for i in 0..self.rect.len() {
            if i == self.number_selection {
                canvas.draw(&self.mesh_selection, Vec2::new(self.rect[i].x, self.rect[i].y));    
                canvas.draw(
                    &self.numbers[i],
                    graphics::DrawParam::default()
                    .dest(Vec2::new(
                        self.rect[i].x + GRID_DIMENSION.0 / 2.,
                        self.rect[i].y + GRID_DIMENSION.1 / 2.,
                    ))
                    .color(graphics::Color::CYAN)
                    .z(2)
                ); 
            } else {
                canvas.draw(&self.mesh, Vec2::new(self.rect[i].x, self.rect[i].y));
                canvas.draw(
                    &self.numbers[i],
                    graphics::DrawParam::default()
                    .dest(Vec2::new(
                        self.rect[i].x + GRID_DIMENSION.0 / 2.,
                        self.rect[i].y + GRID_DIMENSION.1 / 2.,
                    ))
                    .color(graphics::Color::WHITE)
                    .z(1)
                ); 
            }
        }
        Ok(())
    }
}
