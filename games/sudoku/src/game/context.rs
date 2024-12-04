use super::entity::Difficulty;
pub struct AddOnContext {
    pub difficulty: Option<Difficulty>,
}

impl AddOnContext {
    pub fn new() -> Self {
        AddOnContext { difficulty: None }
    }

    pub fn new_forced() -> Self {
        AddOnContext {
            difficulty: Some(Difficulty::None),
        }
    }
}
