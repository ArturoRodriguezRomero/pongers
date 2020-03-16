use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};
use amethyst::core::math::base::Vector2;

#[derive(Copy, Clone)]
pub struct Ball {
    pub velocity: Vector2<f32>,
    pub radius: f32,
}

impl Ball {
    pub fn new() -> Self {
        Self { velocity: Vector2::new(75.0, 50.0), radius: 2.0 }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}