#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
use game::core::{ MainState, SCREEN_SIZE };

use ggez::{ event, GameResult };

fn main() -> GameResult {
    let (ctx, events_loop) = 
        ggez::ContextBuilder::new(
            "Tic tac toe", 
            "alimulap")
        .window_setup(
            ggez::conf::WindowSetup::default()
            .title("Tic tac toe"))
        .window_mode(
            ggez::conf::WindowMode::default()
            .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = MainState::new(&ctx);
    event::run(ctx, events_loop, state)
}