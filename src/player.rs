use macroquad::audio::{play_sound, PlaySoundParams};

use super::*;

pub struct Player
{
    pub entity: Entity,
    sprite: Texture2D,
    weapon: Weapon,

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
        player_weapon.set_stats(2.0, 20.0, 300.0);

        Self { 
            entity: Entity::new("Player", "Player", world), 
            sprite: Texture2D::empty(), 
            weapon: player_weapon,
            sfx_move:  world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_shoot: world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_on_hit: world.assets.get_asset_by_name("hurt_sound_1".to_string()).unwrap().get_sound_data(),

        }
    }  
    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        if is_key_down(KeyCode::Space)
        {
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
        self.sprite = world.assets.get_asset_by_id(3).get_texture_data();
        if self.sprite == Texture2D::empty()
        {
            self.entity.transform.set_size(vec2(60.0,60.0));
        }else 
        {
            self.entity.transform.set_size(vec2( self.sprite.width(), self.sprite.height()));
            self.entity.transform.set_scale( 1.5);
        }
        self.entity.transform.set_position( vec2( self.entity.transform.position.x + GAME_SIZE_X as f32 * 0.5,self.entity.transform.position.y + GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.entity_params.speed = 200.0;
        self.entity.set_rect_color(DARKBLUE);
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

        // WEAPON
        self.weapon.set_parent(Some(self.entity.clone()));
        self.weapon.update(world);

        self.entity.transform.rotation = self.weapon.entity.transform.rotation;
        // Update World
        world.set_entity(&mut self.entity);
    }
    fn late_update(&mut self, world: &mut World) {
        for entity in world.get_actives().iter_mut()
        {
            self.on_collision( entity);
        }
        self.weapon.late_update(world);
    }
    fn draw(&mut self, viewspace: &Viewspace) {
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, 2.0,COLLISION_COLOR);
        }
        if self.sprite == Texture2D::empty()
        {
            draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.entity.get_rect_color());
        }else
        {

            let params = DrawTextureParams { dest_size: Some(self.entity.transform.get_fullsize()), rotation: self.entity.transform.rotation,..Default::default() };
            draw_texture_ex(self.sprite, self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.get_rect_color(), params);
        }
        self.weapon.draw(viewspace);
    }
}
impl Collision for Player
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
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
            _ => {}
        }
    }
} 