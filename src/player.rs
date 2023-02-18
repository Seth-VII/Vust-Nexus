use macroquad::audio::{play_sound, PlaySoundParams};

use super::*;

pub struct Player
{
    pub entity: Entity,
    sprite: TextureAsset,
    weapon: Weapon,

    reached_end: bool,

    sfx_move: SoundData,
    sfx_shoot: SoundData,
    sfx_on_hit: SoundData,
}
impl Player
{
    pub fn new(world: &mut World) -> Self
    {
        let mut player_weapon = Weapon::new("Player Weapn", "Player Weapon", world);
        player_weapon.init(world);
        
        let mut entity = Entity::new("Player", "Player", world);

        let mut params = EntityParams::default();
        params.health = 1000000.0;
        params.armor = 3.0;
        params.speed = 250.0;
        params.damage = 2.0;
        params.firerate = 50.0;
        params.firespeed = 400.0;
        entity.entity_params = params;

        let sprite = world.assets.get_asset_by_id(4).get_texture_asset();

        Self { 
            entity: entity, 
            sprite: sprite, 
            weapon: player_weapon,

            reached_end: false,

            sfx_move:  world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_shoot: world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_on_hit: world.assets.get_asset_by_name("hurt_sound_1".to_string()).unwrap().get_sound_data(),

        }
    }  
    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        if is_key_down(KeyCode::Space)
        {
            self.weapon.set_stats( self.entity.entity_params.damage, self.entity.entity_params.firerate, self.entity.entity_params.firespeed);
            if self.weapon.shoot( misslepool, world)
            {
                let mut params = PlaySoundParams::default();
                params.volume = 0.15;
                play_sound( self.sfx_shoot.sound.unwrap(), params)
            }
        }
    }
}
impl GameObject for Player
{
    fn init(&mut self, world: &mut World) {
        self.sprite.setup_sheet(5, 3);
        self.sprite.animation_controller.apply_state_setup( StateMachineSetup::player_setup() );
        self.sprite.animation_controller.play_anim_once();

        if self.sprite.texture_data == Texture2D::empty()
        {
            self.entity.transform.set_size(vec2(60.0,60.0));
        }else 
        {
            self.entity.transform.set_size(self.sprite.get_sheet_tile_size());
            //self.entity.transform.set_size(vec2( self.sprite.width(), self.sprite.height()));
            self.entity.transform.set_scale( 1.0);
        }
        self.entity.transform.set_position( vec2( self.entity.transform.position.x + GAME_SIZE_X as f32 * 0.5,self.entity.transform.position.y + GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.set_rect_color(WHITE);
    }
    fn update(&mut self, world: &mut World) {
        

        self.entity.hit_cooldown();
        if self.reached_end {
            world.level_completed = true;
            return;
        }
        
        // MOVEMENT
        if is_key_down(KeyCode::W)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position - (vec2(0.0, self.entity.entity_params.speed) * get_frame_time());
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect, world.level_offset) && !resolve_levelwalls(updated_transform.rect, world.get_active_level().get_visible_walls(world.level_offset))
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }
        if is_key_down(KeyCode::S)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position + (vec2(0.0, self.entity.entity_params.speed) * get_frame_time());
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect, world.level_offset) && !resolve_levelwalls(updated_transform.rect, world.get_active_level().get_visible_walls(world.level_offset))
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }

        if is_key_down(KeyCode::A)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position - vec2(self.entity.entity_params.speed, 0.0) * get_frame_time();
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect, world.level_offset) && !resolve_levelwalls(updated_transform.rect, world.get_active_level().get_visible_walls(world.level_offset))
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }
        if is_key_down(KeyCode::D)
        {
            let mut updated_transform = self.entity.transform;
            let new_position = self.entity.transform.position + vec2(self.entity.entity_params.speed, 0.0) * get_frame_time();
            updated_transform.set_position(new_position);

            if !resolve_windowborder(updated_transform.rect, world.level_offset) && !resolve_levelwalls(updated_transform.rect, world.get_active_level().get_visible_walls(world.level_offset))
            {
                self.entity.transform.set_position(updated_transform.position);
            }
        }

        let mut updated_transform = self.entity.transform;
        let new_position = self.entity.transform.position + (vec2(self.entity.entity_params.speed * 3.0, 0.0) * get_frame_time());
        updated_transform.set_position(new_position);
        if !resolve_levelwalls(updated_transform.rect, world.get_active_level().get_visible_walls(world.level_offset))
        {
            self.entity.transform.set_position( vec2(self.entity.transform.position.x + LEVEL_SPEED * get_frame_time(), self.entity.transform.position.y));
        }

        if resolve_deathzone(updated_transform.rect, world.level_offset)
        {
            self.entity.entity_params.health = 0.0;
        }

        

        // WEAPON
        self.weapon.set_parent(Some(self.entity.clone()));
        self.weapon.update(world);
        // Update World
        world.set_entity(&mut self.entity);
    }
    fn late_update(&mut self, world: &mut World) {
        for entity in world.entities.iter_mut()
        {
            self.on_collision( entity);
        }
        self.weapon.late_update(world);

        if is_key_down(KeyCode::W) {
            self.sprite.animation_controller.get_statemachine_mut().SetState(2);
        } else if is_key_down(KeyCode::S) {
            self.sprite.animation_controller.get_statemachine_mut().SetState(1);
        } else {
            self.sprite.animation_controller.get_statemachine_mut().SetState(0);
        }
        
        self.sprite.animation_controller.update();

    }
    fn draw(&mut self) {
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, 2.0,COLLISION_COLOR);
        }
        if self.sprite.texture_data == Texture2D::empty()
        {
            draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.entity.get_rect_color());
        }else
        {
            
            let frame = self.sprite.get_current_anim_controller_frame(); 

            let params = DrawTextureParams { 
                dest_size: Some(self.entity.transform.get_fullsize()), 
                rotation: self.entity.transform.rotation,
                source: frame,
                ..Default::default() 
            };

            draw_texture_ex(self.sprite.texture_data, self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.get_rect_color(), params);
        }
        self.weapon.draw();
    }
}
impl Collision for Player
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            //self.hit_wall = false;
            return;
        }
        let mut params = PlaySoundParams::default();
        params.volume = 0.15;

        match entity.tag.as_str()
        {
            "Enemy" => {
                self.entity.hit(&entity.entity_params);
                play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            "Enemy Weapon Missle" => {
                self.entity.hit(&entity.entity_params);
                play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            "End" => {
                self.reached_end = true;
            }
            _ => {}
        }
    }
} 