use super::*;

pub struct Player
{
    pub entity: Entity,
    sprite: Texture2D,
}
impl Player
{
    pub fn new(world: &mut World) -> Self
    {
        Self { 
            entity: Entity::new("Player", "Player", world), 
            sprite: Texture2D::empty(), 
        }
    }
    
}
impl GameObject for Player
{
    fn init(&mut self) {
        self.entity.transform.set_position( vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.transform.set_size(vec2(60.0,60.0));
        self.entity.entity_params.speed = 200.0;
    }
    fn update(&mut self, world: &mut World) {
        
        // MOVEMENT
        if is_key_down(KeyCode::W)
        {
            let position = self.entity.transform.position - (vec2(0.0, self.entity.entity_params.speed) * get_frame_time()); 
            let updated_rect = Rect::new(
                position.x, 
                position.y, 
                self.entity.transform.rect.w,
                self.entity.transform.rect.h
            );
            if !resolve_windowborder(updated_rect)
            {
                self.entity.transform.set_position(position);
            }
        }
        if is_key_down(KeyCode::S)
        {
            let position = self.entity.transform.position + (vec2(0.0, self.entity.entity_params.speed) * get_frame_time());
            let updated_rect = Rect::new(
                position.x, 
                position.y + self.entity.transform.rect.h, 
                self.entity.transform.rect.w,
                self.entity.transform.rect.h
            );
            if !resolve_windowborder(updated_rect)
            {
                self.entity.transform.set_position(position);
            }
        }

        if is_key_down(KeyCode::A)
        {
            let position = self.entity.transform.position - (vec2(self.entity.entity_params.speed, 0.0) * get_frame_time()); 
            let updated_rect = Rect::new(
                position.x, 
                position.y, 
                self.entity.transform.rect.w,
                self.entity.transform.rect.h
            );
            if !resolve_windowborder(updated_rect)
            {
                self.entity.transform.set_position(position);
            }
        }
        if is_key_down(KeyCode::D)
        {
            let position = self.entity.transform.position + (vec2(self.entity.entity_params.speed, 0.0) * get_frame_time()); 
            let updated_rect = Rect::new(
                position.x + self.entity.transform.rect.w, 
                position.y, 
                self.entity.transform.rect.w,
                self.entity.transform.rect.h
            );
            if !resolve_windowborder(updated_rect)
            {
                self.entity.transform.set_position(position);
            }
        }


        world.set_entity(&mut self.entity);
    }
    fn late_update(&mut self, world: &mut World) {
        
    }
    fn draw(&mut self) {
        

        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, BLACK);
    }
}
impl Collision for Player
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            return;
        }
        match entity.tag.as_str()
        {
            "Enemy" => {
                entity.transform = Transform::zero();
            }
            _ => {}
        }
    }
} 