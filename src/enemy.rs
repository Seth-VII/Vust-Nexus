use super::*;
use macroquad::audio::{play_sound, PlaySoundParams};

#[derive(Clone)]
pub enum EnemyType {
    Default, Heavy, Gunner,
}

#[derive(Clone)]
pub struct EnemyVariant 
{
    params: EntityParams,
    sprite: Texture2D,
    size: Vec2,
    color: Color,
    weapon: Option<Weapon>,
    points: i32,

    sfx_move: SoundData,
    sfx_shoot: SoundData,
    sfx_on_hit: SoundData,
    sfx_explosion: SoundData,
}
impl EnemyVariant 
{
    pub fn has_weapon (&self) -> bool { self.weapon.is_some()}
    pub fn get_variant (e_type: EnemyType, world: &mut World) -> Self 
    {
        match e_type
        {
            EnemyType::Default =>   {return EnemyVariant::default_variant(world);}
            EnemyType::Heavy =>     {return EnemyVariant::heavy_variant(world);}
            EnemyType::Gunner =>    {return EnemyVariant::gunner_variant(world);}
        }
    }
    fn default_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(1).get_texture_data();
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;

        let mut size = vec2(80.0, 80.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 2.0;
            
        }

        let color = WHITE;

        let mut weapon = None;

        let points = 20;

        Self { 
            params: params, 
            sprite: sprite, 
            size: size, 
            color: color, 
            weapon: weapon, 
            points: points,

            sfx_move:       world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_shoot:      world.assets.get_asset_by_name("laserShoot_1".to_string()).unwrap().get_sound_data(),
            sfx_on_hit:     world.assets.get_asset_by_name("hurt_sound_1".to_string()).unwrap().get_sound_data(),
            sfx_explosion:  world.assets.get_asset_by_name("explosion_1".to_string()).unwrap().get_sound_data(),
        }
    }
    fn heavy_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(0).get_texture_data();
        let mut params = EntityParams::default();
        params.health = 20.0;
        params.speed = 170.0;
        params.armor = 5.0;
        params.damage = 3.0;

        let mut size = vec2(120.0, 120.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 3.0;
        }

        let color = WHITE;

        let mut weapon = None;

        let points = 50;

        Self { 
            params: params, 
            sprite: sprite, 
            size: size, 
            color: color, 
            weapon: weapon, 
            points: points,

            sfx_move:       world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_shoot:      world.assets.get_asset_by_name("laserShoot_1".to_string()).unwrap().get_sound_data(),
            sfx_on_hit:     world.assets.get_asset_by_name("hurt_sound_1".to_string()).unwrap().get_sound_data(),
            sfx_explosion:  world.assets.get_asset_by_name("explosion_1".to_string()).unwrap().get_sound_data(),
        }
    }
    fn gunner_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(2).get_texture_data();
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 200.0;
        params.armor = 1.5;
        params.damage = 1.0;
        params.firerate = 1.0;
        params.firespeed = 250.0;

        let mut size = vec2(80.0, 80.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 1.5;
        }

        let color = WHITE;

        let mut weapon = Weapon::new("Gunner Weapon", "Enemy Weapon", world);
        weapon.entity.entity_params = params;
        weapon.set_stats(params.damage, params.firerate, params.firespeed);
        world.set_entity(&mut weapon.entity);

        let points = 10;

        Self { 
            params: params, 
            sprite: sprite, 
            size: size, 
            color: color, 
            weapon: Some(weapon), 
            points: points,

            sfx_move:       world.assets.get_asset_by_name("fire_1".to_string()).unwrap().get_sound_data(),
            sfx_shoot:      world.assets.get_asset_by_name("laserShoot_2".to_string()).unwrap().get_sound_data(),
            sfx_on_hit:     world.assets.get_asset_by_name("hurt_sound_1".to_string()).unwrap().get_sound_data(),
            sfx_explosion:  world.assets.get_asset_by_name("explosion_1".to_string()).unwrap().get_sound_data(),
        }
    }
}


#[derive(Clone)]
pub struct Enemy
{
    pub enemy_id: usize,
    pub entity: Entity,

    pub variant: EnemyVariant,
    pub in_viewspace: bool,
}
impl Enemy
{
    pub fn new(index: usize,world: &mut World) -> Self
    {
        Self { 
            enemy_id: index,
            entity: Entity::new("Enemy", "Enemy", world), 
            variant: EnemyVariant::get_variant(EnemyType::Default, world),
            in_viewspace: false,
        }
    }
    

    pub fn reset(&mut self)
    {
        self.entity.entity_params = EntityParams::default();
        self.entity.transform.set_position(vec2(0.0,0.0));
        self.entity.is_active = false;
    }
    pub fn set_enemytype(&mut self,e_type: &EnemyType, world: &mut World)
    {
        self.variant = EnemyVariant::get_variant(e_type.clone(), world);
        self.entity.entity_params = self.variant.params;
        self.entity.transform.set_size(self.variant.size);
        self.entity.set_rect_color(self.variant.color);
    }
    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        let variant = &mut self.variant;
        let mut weapon = variant.weapon.as_mut().unwrap();
        weapon.entity.entity_params = self.entity.entity_params;
        //println!("WF Speed: {}, name: {}", weapon.entity.entity_params.firespeed, weapon.entity.name);
        if weapon.shoot( misslepool, world)
        {
            let mut params = PlaySoundParams::default();
            params.volume = 0.15;
            play_sound( self.variant.sfx_shoot.sound.unwrap(), params)
        }
    }
}

impl GameObject for Enemy
{
    fn init(&mut self, world: &mut World) {
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.init(world);
            }
            None => {}
        }
    }
    fn update(&mut self, world: &mut World) {

        if self.entity.entity_params.health <= 0.0
        {
            let mut params = PlaySoundParams::default();
            params.volume = 0.5;
            play_sound(world.assets.get_asset_by_name("explosion_2".to_string()).unwrap().get_sound_data().sound.unwrap(), params );

            world.particlesystem_pool.spawn_system_at_position(self.entity.transform.position, 128, Explosion_settings( ORANGE, WHITE, DARKPURPLE));

            self.reset();
            world.add_scorepoints( self.variant.points);
            world.set_entity(&mut self.entity);
            return;
        }

        self.entity.hit_cooldown();

        // MOVEMENT
        //println!("active {}", self.entity.is_active);
        let dir = (self.entity.transform.position - world.get_entity_by_tag("Player").as_ref().unwrap().transform.position).normalize();
        let position = self.entity.transform.position - (dir * self.entity.entity_params.speed * get_frame_time());
        self.entity.transform.set_position(position);
        
        let rotation = f32::atan2(dir.x, dir.y) * -1.0;
        self.entity.transform.rotation = f32::to_radians(rotation.to_degrees() - 270.0);

        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.set_parent(Some(self.entity.clone()));
                weapon.update(world);
            }
            None => {}
        }

       

    }
    fn late_update(&mut self, world: &mut World) {
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.late_update(world);
            }
            None => {}
        }
    }
    fn draw(&mut self) {
        if !self.entity.is_active || !self.entity.sprite_is_active
        {
            return;
        }


        if self.variant.sprite == Texture2D::empty()
        {
            draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.entity.get_rect_color());
        }else
        {

            let params = DrawTextureParams { dest_size: Some(self.entity.transform.get_fullsize()), rotation: self.entity.transform.rotation,..Default::default() };
            draw_texture_ex(self.variant.sprite, self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.get_rect_color(), params);
        }
        
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, 2.0,COLLISION_COLOR);
        }

        draw_text(
            format!("E: {} | HP : {}",self.enemy_id , self.entity.entity_params.health).as_str(), 
            self.entity.transform.position.x, 
            self.entity.transform.position.y, 
            16.0, 
            BLACK);
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.draw();
            }
            None => {}
        }
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
                self.entity.entity_params.health = 0.0;
            }
            "Player Weapon Missle" => {
                //println!("HIT!!!");
                if self.entity.hit_feedback_timer <= 0.0
                {
                    self.entity.hit(&entity.entity_params);
                }

            }
            _ => {}
        }
    }
} 