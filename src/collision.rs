use super::*;

pub trait Collision
{
    fn on_collision(&mut self, entity: &mut Entity);
}

pub fn resolve_windowborder(rect: Rect) -> bool
{
    rect.x < 0.0 || rect.x + rect.w > GAME_SIZE_X as f32 || rect.y < 0.0 || rect.y + rect.h > GAME_SIZE_Y as f32
}
pub fn resolve_intersection(rect_1: Rect, rect_2: Rect) -> bool
{
    match rect_1.intersect(rect_2)
    {
        Some(_intersection) => {return true;}
        None => {return false;}
    }
}