use super::*;

pub struct Enemy
{
    pub entity: Entity,
    sprite: Texture2D,

    rect_color: Color,
    hit_feedback_duration: f32,
    hit_feedback_timer: f32,
}
impl Enemy
{
    pub fn new(world: &mut World) -> Self
    {
        Self { 
            entity: Entity::new("Enemy", "Enemy", world), 
            sprite: Texture2D::empty(), 
            rect_color: GREEN,
            hit_feedback_duration: 0.2,
            hit_feedback_timer: 0.0,
        }
    }
    
    fn hit(&mut self, entity_params: &EntityParams)
    {
        self.hit_feedback_timer = self.hit_feedback_duration;
        self.entity.entity_params.health -= 1.0;
    }
    fn reset(&mut self)
    {
        self.entity.transform = Transform::zero();
        self.entity.SetActive(false);
    }
}

impl GameObject for Enemy
{
    fn init(&mut self) {
        self.entity.transform.set_size(vec2(60.0,60.0));
        self.entity.entity_params.speed = 200.0;
        self.entity.entity_params.health = 10.0;
    }
    fn update(&mut self, world: &mut World) {
        if !self.entity.is_active
        {
            return;
        }
        // MOVEMENT
        
        if self.entity.entity_params.health <= 0.0
        {
            self.reset();
        }

        if self.hit_feedback_timer > 0.0
        {
            self.hit_feedback_timer -= get_frame_time();
            self.rect_color = RED;
        }else
        {
            self.rect_color = GREEN;
        }

        world.set_entity(&mut self.entity);
    }
    fn late_update(&mut self, world: &mut World) {
        
    }
    fn draw(&mut self) {
        if !self.entity.is_active || !self.entity.sprite_is_active
        {
            return;
        }
        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.rect_color);
        draw_text(
            format!("HP : {}", self.entity.entity_params.health).as_str(), 
            self.entity.transform.position.x, 
            self.entity.transform.position.y, 
            16.0, 
            BLACK);
    }
}
impl Collision for Enemy
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            //self.rect_color = GREEN;
            return;
        }
        match entity.tag.as_str()
        {
            "Player" => {

            }
            "Missle" => {
                //println!("HIT!!!");
                if self.hit_feedback_timer <= 0.0
                {
                    self.hit(&entity.entity_params);
                }

            }
            _ => {}
        }
    }
} 