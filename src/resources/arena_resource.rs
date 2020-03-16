#[derive(Copy, Clone, Default)]
pub struct Arena {
    pub height: f32,
    pub width: f32,
}

impl Arena {
    pub fn new() -> Self {
        Self { height: 100.0, width: 100.0 }
    }
}