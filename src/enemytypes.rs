use super::*;



#[derive(Clone)]
pub enum EnemyType {
    Default, Gunner, Tank, HeavyGunner, Exploder, Boss
}

#[derive(Clone)]
pub struct EnemyVariant 
{
    pub params: EntityParams,
    pub sprite: Texture2D,
    pub size: Vec2,
    pub color: Color,
    pub weapon: Option<Weapon>,
    pub points: i32,

    pub sfx_move: SoundData,
    pub sfx_shoot: SoundData,
    pub sfx_on_hit: SoundData,
    pub sfx_explosion: SoundData,
}
impl EnemyVariant 
{
    pub fn has_weapon (&self) -> bool { self.weapon.is_some()}
    pub fn get_variant (e_type: EnemyType, world: &mut World) -> Self 
    {
        match e_type
        {
            EnemyType::Default =>   {return EnemyVariant::default_variant(world);}
            EnemyType::Gunner =>    {return EnemyVariant::gunner_variant(world);}
            EnemyType::Tank =>     {return EnemyVariant::tank_variant(world);}
            EnemyType::HeavyGunner =>     {return EnemyVariant::tank_variant(world);}
            EnemyType::Exploder =>    {return EnemyVariant::gunner_variant(world);}
            EnemyType::Boss =>    {return EnemyVariant::gunner_variant(world);}
        }
    }
    fn default_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(1).get_texture_data();
        let params = EntitySettings::enemy_default_settings();

        let mut size = vec2(80.0, 80.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 2.0;
            
        }

        let color = GREEN;

        let weapon = None;

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
    fn tank_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(0).get_texture_data();
        let params = EntitySettings::enemy_tank_settings();

        let mut size = vec2(120.0, 120.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 3.0;
        }

        let color = PURPLE;

        let mut weapon = Weapon::new("Tank Weapon", "Enemy Weapon", world);
        weapon.entity.entity_params = params;
        weapon.set_stats(params.damage, params.firerate, params.firespeed);
        world.set_entity(&mut weapon.entity);

        let points = 50;

        Self { 
            params: params, 
            sprite: sprite, 
            size: size, 
            color: color, 
            weapon: Some(weapon), 
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
        let params = EntitySettings::enemy_gunner_settings();

        let mut size = vec2(80.0, 80.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 1.5;
        }

        let color = RED;

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

