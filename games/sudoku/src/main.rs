#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
use game::core as Game;

fn main() {
    Game::run();
}
