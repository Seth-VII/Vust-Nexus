use super::*;

pub trait Collision
{
    fn on_collision(&mut self, entity: &mut Entity);
}

pub fn resolve_windowborder(rect: Rect, level_progress: f32) -> bool
{
    rect.x < -100.0 + level_progress || rect.x + rect.w > GAME_SIZE_X as f32 + level_progress || rect.y < 0.0 || rect.y + rect.h > GAME_SIZE_Y as f32
}
pub fn inside_windowborder(rect: Rect, level_progress: f32, y_offset: f32) -> bool
{
    rect.x > 0.0 - level_progress && rect.x + rect.w < GAME_SIZE_X as f32 + level_progress && 
    rect.y > 0.0 - y_offset && rect.y + rect.h < GAME_SIZE_Y as f32 + y_offset
}
pub fn resolve_levelwalls(rect: Rect, walls: Vec<Rect>) -> bool
{
    for wall in walls.iter()
    {
        if rect.intersect(*wall).is_some() { return true;}
    }
    return false;
}
pub fn resolve_intersection(rect_1: Rect, rect_2: Rect) -> bool
{
    match rect_1.intersect(rect_2)
    {
        Some(_intersection) => {return true;}
        None => {return false;}
    }
}