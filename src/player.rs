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
    fn init(&mut self, world: &mut World) {
        self.entity.transform.set_size(vec2(60.0,60.0));
        self.entity.transform.set_position( vec2( self.entity.transform.position.x + GAME_SIZE_X as f32 * 0.5,self.entity.transform.position.y + GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.entity_params.speed = 200.0;
        self.entity.set_rect_color(BLACK);
    }
    fn update(&mut self, world: &mut World) {
        

        self.entity.hit_cooldown();

        // MOVEMENT
        if is_key_down(KeyCode::W)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position - (vec2(0.0, self.entity.entity_params.speed) * get_frame_time());
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect)
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }
        if is_key_down(KeyCode::S)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position + (vec2(0.0, self.entity.entity_params.speed) * get_frame_time());
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect)
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }

        if is_key_down(KeyCode::A)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position - vec2(self.entity.entity_params.speed, 0.0) * get_frame_time();
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect)
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }
        if is_key_down(KeyCode::D)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position + vec2(self.entity.entity_params.speed, 0.0) * get_frame_time();
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect)
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }


        world.set_entity(&mut self.entity);
    }
    fn late_update(&mut self, world: &mut World) {
        for entity in world.get_actives().iter_mut()
        {
            self.on_collision( entity);
        }
    }
    fn draw(&mut self) {
        

        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.entity.get_rect_color());
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
                self.entity.hit(&entity.entity_params);
            }
            "Enemy Weapon Missle" => {
                println!("HIT");
                self.entity.hit(&entity.entity_params);
            }
            _ => {}
        }
    }
} 