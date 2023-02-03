use super::*;

#[derive(Clone, Copy,PartialEq)]
pub struct Transform
{
    pub position: Vec2,
    pub rotation: f32,
    pub size: Vec2,

    pub scale: f32,
    pub rect: Rect,
}
impl Transform
{
    pub fn zero() -> Self
    {
        Self {
            position: vec2(0.0,0.0),
            rotation: 0.0,
            size: vec2(0.0,0.0),
            scale: 0.0,
            rect: Rect::new(0.0,0.0,0.0,0.0),
        }
    }
    pub fn default() -> Self
    {
        Self {
            position: vec2(0.0,0.0),
            rotation: 0.0,
            size: vec2(10.0,10.0),
            scale: 1.0,
            rect: Rect::new(0.0,0.0,10.0,10.0),
        }
    }
    pub fn set_position(&mut self, new_position: Vec2)
    {
        self.position = new_position;
        let centered = self.get_centered_position();
        self.rect.x = centered.x;
        self.rect.y = centered.y;
    }
    pub fn set_position_not_centered(&mut self, new_position: Vec2)
    {
        self.position = new_position;
        self.rect.x = new_position.x;
        self.rect.y = new_position.y;
    }
    pub fn set_size(&mut self, new_size: Vec2)
    {
        self.size = new_size;
        self.rect.w = new_size.x * self.scale;
        self.rect.h = new_size.y * self.scale;
    }
    pub fn set_scale(&mut self, new_scale: f32)
    {
        self.scale = new_scale;
        self.rect.w = self.size.x * new_scale;
        self.rect.h = self.size.y * new_scale;
    }
    pub fn get_fullsize(&self) -> Vec2 {self.size * self.scale}
    pub fn get_centered_position(&self) -> Vec2
    {
        vec2( self.position.x - ((self.size.x * self.scale) * 0.5), self.position.y - ((self.size.y * self.scale) * 0.5) )
    }
}

#[derive(Clone,PartialEq)]
pub struct Entity
{
    pub id: usize,
    pub name: String,
    pub tag: String,
    
    pub transform: Transform,
    pub entity_params: EntityParams,

    pub is_active: bool,
    pub collision_is_enabled: bool,
    pub sprite_is_active: bool,


    rect_color: Color,
    hit_color: Color,
    hit_feedback_duration: f32,
    pub hit_feedback_timer: f32,
}
impl Entity
{
    pub fn new(name: &str, tag: &str, world: &mut World) -> Self
    {
        let mut instance = Self {
            id: world.entities.len(),
            name: name.to_string(),
            tag: tag.to_string(),
            transform: Transform::default(),
            is_active: true,
            entity_params: EntityParams::default(),
            collision_is_enabled: true,
            sprite_is_active: true,

            rect_color: WHITE,
            hit_color: WHITE,
            hit_feedback_duration: 0.05,
            hit_feedback_timer: 0.0,
        };
        world.add_entity(&mut instance);
        instance
    }

    pub fn SetActive(&mut self, state: bool)
    {
        self.is_active = state;
    }

    pub fn get_rect_color(&self) -> Color {self.hit_color}
    pub fn set_rect_color(&mut self, color: Color){self.rect_color = color;}
    pub fn hit(&mut self, entity_params: &EntityParams)
    {
        self.hit_feedback_timer = self.hit_feedback_duration;
        self.entity_params.health -= 1.0;
    }
    pub fn hit_cooldown(&mut self)
    {
        if self.hit_feedback_timer > 0.0
        {
            self.hit_feedback_timer -= get_frame_time();
            self.hit_color = RED;
        }else
        {
            self.hit_color = self.rect_color;
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct EntityParams
{
    pub health: f32,
    pub speed: f32,
    
    pub damage: f32,
    pub armor: f32,
    pub firespeed: f32,
    pub firerate: f32,
}
impl EntityParams
{
    pub fn default() -> Self
    {
        Self {
            health: 1.0,
            speed: 0.0,
            damage: 0.0,
            armor: 0.0,
            firespeed: 0.0,
            firerate: 0.0,
        }
    }
}