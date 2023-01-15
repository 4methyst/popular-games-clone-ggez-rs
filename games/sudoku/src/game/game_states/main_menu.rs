use std::collections::BTreeMap;

use ggez::{
    GameResult, Context,
    graphics::{ self, Text },
};

use crate::game::ui::*;
use super::StateTrait;

pub struct MainMenu {
    texts: BTreeMap<&'static str, Text>,
    buttons: BTreeMap<&'static str, Button>,
}

impl MainMenu {
    pub fn new(ctx: &Context) -> Self {
        MainMenu {
            texts: BTreeMap::new(),
            buttons: BTreeMap::new(),
        }
    }
}

impl StateTrait for MainMenu {
    fn update(&mut self, _ctx: &Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context, _canvas: &mut graphics::Canvas) -> GameResult {
        Ok(())
    }
}