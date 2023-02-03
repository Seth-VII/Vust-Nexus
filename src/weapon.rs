use super::*;

#[derive(Clone)]
pub struct Weapon
{
    pub entity: Entity,
    parent: Option<Entity>,
    sprite: TextureAsset,
    direction: Vec2,

    cooldown_t: f32,
}
impl Weapon
{
    pub fn new(name: &str, tag: &str, world: &mut World) -> Self
    {
        Self {
            entity: Entity::new(name, tag, world),
            parent: None,
            sprite: world.assets.get_asset_by_name("weapon_sheet".to_string()).unwrap().get_texture_asset(),
            direction: vec2(0.0, 0.0),
            cooldown_t: 0.0,
        }
    }
    pub fn set_parent(&mut self, parent: Option<Entity>)
    {
        self.parent = parent;
    }

    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World) -> bool
    {
        //println!("FSpeed: {}", self.entity.entity_params.firespeed);
        if self.cooldown_t <= 0.0 && !self.sprite.animation.is_playing
        {
            misslepool.fire_missle( self.entity.clone(), self.direction, world);
            self.cooldown_t = 2.0;
            self.sprite.animation.play_anim_once();
            println!("Firerate: {}",self.entity.entity_params.firerate );
            self.sprite.animation.set_animation_speed(self.entity.entity_params.firerate * 0.25);
            return true;
        }else  {
            self.cooldown_t -= self.entity.entity_params.firerate * get_frame_time();
            return false;
        }
    }
    pub fn set_stats(&mut self, dmg: f32, firerate: f32, firespeed: f32)
    {
        self.entity.entity_params.damage = dmg;
        self.entity.entity_params.firerate = firerate;
        self.entity.entity_params.firespeed = firespeed;
    }
}
impl GameObject for Weapon
{
    fn init(&mut self, world: &mut World) {
        self.sprite.setup_sheet(4, 4);
        if self.sprite.texture_data == Texture2D::empty()
        {
            self.entity.transform.set_size(vec2(20.0,20.0));
        }else
        {
            self.entity.transform.set_size(self.sprite.get_sheet_tile_size());
            self.entity.transform.set_scale(0.75);
        }
        self.entity.transform.set_position( vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.entity_params.firespeed = 100.0;
    }
    fn update(&mut self, world: &mut World) {
        
        match self.entity.tag.as_str()
        {
            "Player Weapon" => {
                let dir = vec2(self.entity.transform.position.x - mouse_position().0, self.entity.transform.position.y - mouse_position().1);
                self.direction = dir.normalize() * -1.0;
                self.entity.transform.rotation = f32::to_radians(rotation.to_degrees() - 90.0);     

                // Update from parent
                match &mut self.parent
                {
                    Some(parent) => {
                        self.entity.transform.set_position(parent.transform.position);
                    }
                    None => {}
                }
            }
            "Enemy Weapon" => {
                let player_option =  world.get_entity_by_tag("Player");
                match player_option
                {
                    Some(player) => {
                        self.direction = vec2(-1.0, 0.0);
                        self.missle_spawn_offset = vec2(-55.0, 0.0);
                    }
                    None => {}
                }
                // Update from parent
                match &mut self.parent
                {
                    Some(parent) => {
                        self.entity.transform.set_position(parent.transform.position);
                    }
                    None => {}
                }
            }
            _ => {}
        }
    }
    fn late_update(&mut self, world: &mut World) {
        
    }
    fn draw(&mut self) {
        if self.sprite.texture_data == Texture2D::empty()
        {
            draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, DARKGRAY);
        }else
        {
            self.sprite.animation.update();
            let frame = self.sprite.get_current_frame(); 
            self.params.source = frame;
            self.params.dest_size = Some(self.entity.transform.get_fullsize());
            self.params.rotation = self.entity.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.get_rect_color(), self.params.clone());
        }
    }
}