use super::*;

#[derive(Clone)]
pub struct Weapon
{
    pub entity: Entity,
    parent: Option<Entity>,
    sprite: TextureAsset,
    direction: Vec2,

    pub missle_spawn_offset: Vec2,
    params: DrawTextureParams,

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
            missle_spawn_offset: vec2(0.0, 0.0),
            params: DrawTextureParams::default(),
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
            misslepool.fire_missle( self.entity.clone(), self.direction, self.missle_spawn_offset, world);
            self.cooldown_t = 2.0;
            self.sprite.animation.set_animation_speed( f32::clamp(self.entity.entity_params.firerate * 0.25, 1.0, 1000.0));
            self.sprite.animation.play_anim_once();
            //println!("Firerate: {}",self.entity.entity_params.firerate );

            
            world.particlesystem_pool.spawn_system_at_position(self.entity.transform.position + self.missle_spawn_offset, 16, fire_settings(self.direction));
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
        

        match self.entity.tag.as_str()
        {
            "Player Weapon" => {
                if self.sprite.texture_data == Texture2D::empty()
                {
                    self.entity.transform.set_size(vec2(20.0,20.0));
                }else
                {
                    self.entity.transform.set_size(self.sprite.get_sheet_tile_size());
                    self.entity.transform.set_scale(0.2);
                }
                self.entity.transform.set_position( vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5 ));
                self.params.flip_x = false;
                self.entity.entity_params.firespeed = 100.0;
            }
            "Enemy Weapon" => {
                if self.sprite.texture_data == Texture2D::empty()
                {
                    self.entity.transform.set_size(vec2(20.0,20.0));
                }else
                {
                    self.entity.transform.set_size(self.sprite.get_sheet_tile_size());
                    self.entity.transform.set_scale(0.5);
                }
                self.entity.transform.set_position( vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5 ));
                self.params.flip_x = false;
                self.entity.entity_params.firespeed = 100.0;
            }
            _ => {}
        }
    }
    fn update(&mut self, world: &mut World) {
        
        match self.entity.tag.as_str()
        {
            "Player Weapon" => {
                let relative_mouseposition = vec2(mouse_position().0, mouse_position().1);
                let mouseposition_worldoffset = vec2( relative_mouseposition.x + world.level_offset, relative_mouseposition.y);
                //println!("{}", mouseposition_worldoffset);

                let dir_unnormalized = vec2(self.entity.transform.position.x - mouseposition_worldoffset.x, self.entity.transform.position.y - mouseposition_worldoffset.y);
                self.direction = dir_unnormalized.normalize() * -1.0;
                //self.direction = vec2(1.0,0.0) ;

                self.missle_spawn_offset = vec2(-15.0,0.0) + 50.0 * self.direction;
                //self.missle_spawn_offset = vec2(25.0,0.0) ;

                let rotation = f32::atan2(dir_unnormalized.x, dir_unnormalized.y) *-1.0;
                self.entity.transform.rotation = f32::to_radians(rotation.to_degrees() - 90.0); 
                   
                self.params.pivot = Some( vec2( self.entity.transform.position.x - (self.entity.transform.get_fullsize().x * 0.5) + 15.0, self.entity.transform.position.y));
                
                // Update from parent
                match &mut self.parent
                {
                    Some(parent) => {
                        self.entity.transform.set_position(parent.transform.position + vec2(parent.transform.get_fullsize().x * 0.5, 0.0)  - vec2( 15.0, 2.0));
                    }
                    None => {}
                }
            }
            "Enemy Weapon" => {
                let player_option =  world.get_entity_by_tag("Player");
                match player_option
                {
                    Some(player) => {
                        self.direction = (player.transform.position - self.entity.transform.position).normalize();
                        self.missle_spawn_offset = 65.0 * self.direction;

                        let rotation = f32::atan2(self.direction.x, self.direction.y) *-1.0;
                        self.entity.transform.rotation = f32::to_radians(rotation.to_degrees() - 90.0);     
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
            let frame = self.sprite.get_current_animation_frame(); 
            self.params.source = frame;
            self.params.dest_size = Some(self.entity.transform.get_fullsize());
            
            self.params.rotation = self.entity.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.get_rect_color(), self.params.clone());
        }
    }
}