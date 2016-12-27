pub mod sine;
pub mod square;
pub mod mult;
pub mod add;

pub use module::sine::Sine;
pub use module::square::Square;
pub use module::mult::Mult;
pub use module::add::Add;

pub trait Module
{
    fn generate(&mut self, step: f32) -> Option<f32>;
}

impl Module for f32
{
    fn generate(&mut self, _step: f32) -> Option<f32>
    {
        Some(*self)
    }
}
