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
    pub numbers: [[u8; 9]; 9],
    pub number_state: [[Condition; 9]; 9],
    pub number_selected: u8,
    grid_mesh: Mesh,
    grid_mesh_selection: Mesh,
    region_mesh: Mesh,
    number_draw: [Text; 10],
}

impl GameBoard {
    pub fn init(ctx: &Context, x: f32, y: f32, difficulty: &Difficulty) -> GameBoard {
        let mut grid_rect = [[Rect::default(); 9]; 9];
        for i in 0..9 {
            for j in 0..9 {
                grid_rect[i][j] = Rect::new(
                    x + (j as f32 * GRID_DIMENSION.0),
                    y + (i as f32 * GRID_DIMENSION.1),
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
        let grid_mesh_selection = Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::Stroke( 
                graphics::StrokeOptions::default()
                .with_line_width(2.)
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
                .with_line_width(2.)
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

        let (numbers, number_state) = GameBoard::generate_sudoku(difficulty);

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
            grid_mesh_selection,
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
                    Condition::PreDetermined => (2, graphics::Color::new(0.8,0.8,0.3,1.0)),
                    Condition::Neutral => (1, graphics::Color::WHITE),
                    Condition::Wrong => (3, graphics::Color::RED),
                };

                if self.numbers[i][j] == self.number_selected
                    && self.numbers[i][j] != 0 {
                    canvas.draw(
                        &self.grid_mesh_selection,
                        graphics::DrawParam::default()
                        .dest(Vec2::new(self.grid_rect[i][j].x, self.grid_rect[i][j].y))
                        .z(5).color(graphics::Color::CYAN)
                    );
                } else {  
                    canvas.draw(
                        &self.grid_mesh,
                        graphics::DrawParam::default()
                        .dest(Vec2::new(self.grid_rect[i][j].x, self.grid_rect[i][j].y))
                        .z(index).color(color)
                    );
                }

                canvas.draw(
                    &self.number_draw[self.numbers[i][j] as usize],
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

    fn generate_sudoku(difficulty: &Difficulty) -> ([[u8; 9]; 9],[[Condition; 9]; 9]) {
        let mut numbers = [[0u8; 9]; 9];
        GameBoard::solve_sudoku(&mut numbers);
        let mut conditions = [[Condition::PreDetermined; 9]; 9];
        let number_remove: usize = match difficulty {
            Difficulty::None => 81,
            Difficulty::Easy => 45,
            Difficulty::Intermediate => 54,
            Difficulty::Hard => 63,
        };
        let mut number_removed = Vec::new();

        while number_removed.len() < number_remove {
            let (i, j): (usize, usize) = {
                let rand = rand::random::<usize>() % 81;
                (rand/9,rand%9)
            };
            if numbers[i][j] == 0 {
                continue;
            }
            let backup = numbers[i][j];
            numbers[i][j] = 0;
            conditions[i][j] = Condition::Neutral;
            number_removed.push((i, j, backup));
            if GameBoard::has_unique_solution(&numbers) {
                let (i, j, backup) = number_removed.pop().unwrap();
                numbers[i][j] = backup;
                conditions[i][j] = Condition::PreDetermined;
            }
        }
        (numbers, conditions)
    }

    fn has_unique_solution(numbers: &[[u8; 9]; 9]) -> bool {
        let mut numbers_temp = numbers.clone();
        return GameBoard::solve_sudoku(&mut numbers_temp) && numbers_temp == *numbers;
    }

    fn solve_sudoku(numbers: &mut [[u8; 9]; 9]) -> bool {
        let (i, j) = if let Some(empty_cell) = GameBoard::find_empty(&numbers) {
            (empty_cell as usize / 9, empty_cell as usize % 9)
        } else {
            return true;
        };
        print!("a");

        for number in 1..=9 {
            if GameBoard::check_valid(number, i, j, &numbers) {
                numbers[i][j] = number;
                if GameBoard::solve_sudoku(numbers) {
                    return true;
                }
                numbers[i][j] = 0;
            }
        }
        false
    }

    fn find_empty(numbers: &[[u8; 9]; 9]) -> Option<u8> {
        for i in 0..81 {
            if numbers[i/9][i%9] == 0 {
                return Some(i as u8);
            }
        }
        None
    } 

    /// true means fine, false means there is same number(s) horizontally, vertically, or in the same region
    pub fn check_valid(number: u8, i: usize, j: usize, numbers: &[[u8; 9]; 9]) -> bool {
        let start_i = i - i % 3;
        let start_j = j - j % 3;
        for k in 0..9 {
            if numbers[start_i + k / 3][start_j + k % 3] == number && !(start_i + k / 3 == i && start_j + k % 3 == j) {
                return false;
            }

            if numbers[i][k] == number && j != k {
                return false;
            }

            if numbers[k][j] == number && i != k {
                return false;
            }
        }
        true
    }
}

pub struct NumberBoard {
    pub rect: [Rect; 10],
    mesh: Mesh,
    mesh_selection: Mesh,
    numbers: [Text; 10],
    pub number_selection: u8,
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
            if i == self.number_selection as usize {
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
