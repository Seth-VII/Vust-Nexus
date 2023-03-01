use super::*;

pub trait Collision
{
    fn on_collision(&mut self, entity: &mut Entity);
}

pub fn resolve_windowborder(rect: Rect, level_progress: f32) -> bool
{
    rect.x < 0.0 + level_progress || rect.x + rect.w > GAME_SIZE_X + level_progress|| rect.y < 0.0 || rect.y + rect.h > GAME_SIZE_Y
}
pub fn resolve_windowborder_with_offset(rect: Rect, level_progress: f32, x_offset: Vec2, y_offset: Vec2) -> bool
{
    rect.x < x_offset.x + level_progress || rect.x + rect.w > x_offset.y + GAME_SIZE_X + level_progress|| rect.y < y_offset.x || rect.y + rect.h > y_offset.y + GAME_SIZE_Y
}
pub fn resolve_deathzone(rect: Rect, level_progress: f32) -> bool
{
    rect.x < -100.0 + level_progress || rect.x + rect.w > 100.0 + GAME_SIZE_X + level_progress|| rect.y < 0.0 || rect.y + rect.h > GAME_SIZE_Y
}
pub fn resolve_extended_deathzone(rect: Rect, level_progress: f32) -> bool
{
    rect.x < -100.0 + level_progress || rect.x + rect.w > 100.0 + GAME_SIZE_X + level_progress|| rect.y < -50.0 || rect.y + rect.h > GAME_SIZE_Y + 50.0
}
pub fn inside_windowborder(rect: Rect, level_progress: f32, y_offset: f32) -> bool
{
    rect.x > 0.0 - level_progress && rect.x + rect.w < GAME_SIZE_X + level_progress && 
    rect.y > 0.0 - y_offset && rect.y + rect.h < GAME_SIZE_Y + y_offset
}
pub fn inside_windowview(rect: Rect, level_progress: f32) -> bool
{
    rect.x > 0.0 - level_progress - 150.0 && rect.x + rect.w < GAME_SIZE_X + level_progress + 150.0 && 
    rect.y > 0.0 - 150.0 && rect.y + rect.h < GAME_SIZE_Y + 150.0
}
pub fn inside_windowborder_extended_sides(rect: Rect, level_progress: f32, y_offset: f32, x_extrension_range: Vec2) -> bool
{
    rect.x > level_progress - x_extrension_range.x && rect.x + rect.w < (GAME_SIZE_X + level_progress) + x_extrension_range.y && 
    rect.y > 0.0 - y_offset && rect.y + rect.h < GAME_SIZE_Y + y_offset
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