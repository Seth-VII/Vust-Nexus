use super::*;

pub struct Viewspace
{
    position: Vec2,
    radius: f32,
}
impl Viewspace
{
    pub fn new() -> Self { Self { position: vec2(0.0,0.0), radius: 500.0 }}
    pub fn set_position(&mut self, position: Vec2) {self.position = position;}
    pub fn set_radius(&mut self, radius: f32) {self.radius = radius;}

    pub fn get_position(&self) -> Vec2{self.position}
    pub fn get_radius(&self) -> f32 {self.radius}

    pub fn draw(&self) 
    {
        draw_circle(self.position.x, self.position.y, self.radius, color_u8!(50,50,50,50));
    }
}


pub fn inside_screen(rect: Rect) -> bool
{
    rect.x < 0.0 || rect.x + rect.w > GAME_SIZE_X as f32 || rect.y < 0.0 || rect.y + rect.h > GAME_SIZE_Y as f32
}
// Circle like Area
pub fn inside_visible_area(rect: Rect, viewspace_position: Vec2, radius: f32) -> bool
{
    let viewspace = radius;
    let mut test_x = viewspace_position.x;
    let mut test_y = viewspace_position.y;

    if viewspace_position.x < rect.x
    {
        test_x = rect.x
    }else
    if viewspace_position.x > rect.x + rect.w
    {
        test_x = rect.x + rect.w
    }

    if viewspace_position.y < rect.y
    {
        test_y = rect.y
    }else
    if viewspace_position.y > rect.y + rect.h
    {
        test_y = rect.y + rect.h
    }

    let dist_x = viewspace_position.x - test_x;
    let dist_y = viewspace_position.y - test_y;

    let distance = ((dist_x*dist_x) + (dist_y*dist_y)).sqrt();

    if distance <= viewspace
    {
        return true;
    }
    return false;
}