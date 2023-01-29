pub use super::*;

pub trait GameObject
{
    fn init(&mut self);
    fn update(&mut self, world: &mut World);
    fn late_update(&mut self, world: &mut World);
    fn draw(&mut self);
}