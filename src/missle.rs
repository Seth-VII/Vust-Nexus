use super::*;
use macroquad::audio::{play_sound, PlaySoundParams};
pub struct MisslePool
{
    pool: Vec<Missle>,
    active_pool: Vec<Missle>
}
impl MisslePool
{
    pub fn new() -> Self
    {
        Self { pool: Vec::new(), active_pool: Vec::new() }
    }
    pub fn create_pool(&mut self, count: usize, world: &mut World)
    {
        self.pool.clear();
        self.active_pool.clear();
        for i in 0..count
        {
            let mut missle = Missle::new(i, world);
            missle.reset_missle();
            world.set_entity(&mut missle.entity);
            self.pool.push(missle);
        }
    }
    pub fn fire_missle(&mut self, from_weapon: Entity, dir: Vec2, missle_spawn_offset: Vec2, world: &mut World)
    {
        let free_slot = self.get_free_slot();
        match free_slot
        {
            Some(slot) => {
                self.pool[slot].setup_missle(from_weapon, dir, missle_spawn_offset);
                self.pool[slot].fire();
                self.active_pool.push(self.pool[slot].clone());
                world.set_entity(&mut self.pool[slot].entity);
                //println!("Missle tag: {}",  self.pool[slot].entity.tag );
            }
            None => {
                println!("No Free Missle Slot Found!");
            }
        }
    }
    fn get_free_slot(&self) -> Option<usize>
    {
        for i in 0..self.pool.len()
        {
            if !self.pool[i].entity.is_active
            {
                return Some(i);
            }
        }
        return None
    }
   
   
}
impl GameObject for MisslePool
{
    fn init(&mut self, world: &mut World) {
    }
    fn update(&mut self, world: &mut World) {
        self.active_pool = self.pool.clone();
        self.active_pool.retain(|m| m.entity.is_active == true); 
        for missle in self.active_pool.iter_mut()
        {
            //let pool_missle = &mut self.pool[self.active_pool[i].misslepool_id];
            //pool_missle.update(world);

            //self.active_pool[i] = pool_missle.clone();
            //world.set_entity(&mut pool_missle.entity);
            missle.update(world);
            self.pool[missle.misslepool_id] = missle.clone();
            world.set_entity(&mut missle.entity);
        }
        //println!("count : {}", self.active_pool.len());
    }
    fn late_update(&mut self, world: &mut World) {
        for missle in self.active_pool.iter_mut()
        {
            //let pool_missle = &mut self.pool[self.active_pool[i].misslepool_id];
            //pool_missle.late_update(world);

            //self.active_pool[i] = pool_missle.clone();
            //world.set_entity(&mut pool_missle.entity);
            missle.late_update(world);
            self.pool[missle.misslepool_id] = missle.clone();
            world.set_entity(&mut missle.entity);
        }
    }
    fn draw(&mut self) {
        for missle in self.pool.iter_mut()
        {
            missle.draw();
        }
    }
}

#[derive(Clone)]
pub struct Missle
{
    misslepool_id: usize, 
    entity: Entity,
    weapon: Option<Entity>,
    dir: Vec2,
    sprite: Texture2D,
    color: Color,

    sfx_hit: SoundData,

}
impl Missle
{
    pub fn new(pool_id: usize,world: &mut World) -> Self
    {
        let fx_params = ParticleParams::new();
        let vfx = ParticleSystem::new(32, fx_params);

        Self {
            misslepool_id: pool_id,
            entity: Entity::new("Missle","Missle", world),
            weapon: None,
            dir: vec2(0.0, 0.0),
            sprite: world.assets.get_asset_by_name("player_missle_1".to_string()).unwrap().get_texture_data(),
            color: WHITE,
            sfx_hit: world.assets.get_asset_by_name("hit_1".to_string()).unwrap().get_sound_data(),
        }
    }

    pub fn setup_missle(&mut self,from_weapon: Entity, dir: Vec2, missle_spawn_offset: Vec2)
    {
        if self.sprite == Texture2D::empty()
        {
            self.entity.transform.set_size( vec2(30.0,30.0));
            self.entity.transform.set_scale( 1.0 );
        }else 
        {
            // Missle Texture -> Size Squared for balanced shooting in a rotation
            self.entity.transform.set_size(vec2( self.sprite.height(), self.sprite.height()));
            self.entity.transform.set_scale( 1.5 );
        }

        self.entity.transform.set_position(from_weapon.transform.position + missle_spawn_offset);
        self.entity.transform.rotation = from_weapon.transform.rotation;
        self.entity.entity_params = from_weapon.entity_params.clone();
        self.entity.tag = format!("{} Missle", from_weapon.tag);

        if from_weapon.tag.as_str().contains("Player")
        {
            self.color = WHITE;
        }else if from_weapon.tag.as_str().contains("Enemy")
        {
            self.color = RED;
        }

        self.weapon = Some(from_weapon);
        self.dir = dir;

        
    }
    pub fn fire(&mut self)
    {
        self.entity.SetActive(true);
        
    }
    pub fn reset_missle(&mut self)
    {
        self.weapon = None;
        self.dir = vec2(0.0, 0.0);

        self.entity.transform = Transform::zero();
        self.entity.SetActive(false);
    }

}
impl GameObject for Missle
{
    fn init(&mut self, world: &mut World) {
        
    }
    fn update(&mut self, world: &mut World) {
        if !self.entity.is_active
        {
            return;
        }
        //println!("Speed: {}", self.entity.entity_params.firespeed);
        let position = self.entity.transform.position + (self.dir * self.entity.entity_params.firespeed * get_frame_time());
        self.entity.transform.set_position(position);
        if resolve_windowborder(self.entity.transform.rect, world.level_offset)
        {
            self.reset_missle();
        }

        
    }

    fn late_update(&mut self, world: &mut World) {
        //let mut filtered_world = world.entities.clone();
        //filtered_world.retain(|e| e.tag != "Missle");
        for entity in world.get_actives().iter_mut()
        {
            if !entity.tag.contains("Missle")
            {
                self.on_collision(entity);
            }
        }
    }

    fn draw(&mut self) {
        if !self.entity.is_active
        {
            return;
        }
        if self.sprite == Texture2D::empty()
        {
            draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.color);
        }else
        {

            let params = DrawTextureParams { dest_size: Some(self.entity.transform.get_fullsize()), rotation: self.entity.transform.rotation,..Default::default() };
            draw_texture_ex(self.sprite, self.entity.transform.rect.x, self.entity.transform.rect.y, self.color, params);
        }
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, 2.0,COLLISION_COLOR);
        }
    }
}
impl Collision for Missle
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            //self.rect_color = GREEN;
            return;
        }

        let mut params = PlaySoundParams::default();
        params.volume = 0.01;

        match entity.tag.as_str()
        {
            "Enemy" => {
                if self.entity.tag.contains("Player")
                {
                    self.reset_missle();
                    play_sound( self.sfx_hit.sound.unwrap(), params);
                }
            },
            "Player" => {
                if self.entity.tag.contains("Enemy")
                {
                    self.reset_missle();
                    play_sound( self.sfx_hit.sound.unwrap(), params);
                }
            },
            "Wall" => {
                if self.entity.tag.contains("Player") || self.entity.tag.contains("Enemy")
                {
                    self.reset_missle();
                    play_sound( self.sfx_hit.sound.unwrap(), params);
                }
            },
            "Destructible" => {
                if self.entity.tag.contains("Player")
                {
                    self.reset_missle();
                    play_sound( self.sfx_hit.sound.unwrap(), params);
                }
            },
            "Turret" => {
                if self.entity.tag.contains("Player")
                {
                    self.reset_missle();
                    play_sound( self.sfx_hit.sound.unwrap(), params);
                }
            },
            _ => {}
        }
    }
}