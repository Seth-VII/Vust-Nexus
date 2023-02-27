use super::*;



#[derive(Clone)]
pub enum EnemyType {
    Default, Tank, Gunner, HeavyGunner, Exploder, Boss
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
            EnemyType::Tank =>     {return EnemyVariant::heavy_variant(world);}
            EnemyType::Gunner =>    {return EnemyVariant::gunner_variant(world);}
            EnemyType::HeavyGunner =>     {return EnemyVariant::heavy_variant(world);}
            EnemyType::Exploder =>    {return EnemyVariant::gunner_variant(world);}
            EnemyType::Boss =>    {return EnemyVariant::gunner_variant(world);}
        }
    }
    fn default_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(1).get_texture_data();
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;

        let mut size = vec2(80.0, 80.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 2.0;
            
        }

        let color = WHITE;

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
    fn heavy_variant(world: &mut World) -> Self
    {
        let sprite = world.assets.get_asset_by_id(0).get_texture_data();
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.speed = 170.0;
        params.armor = 5.0;
        params.damage = 3.0;

        let mut size = vec2(120.0, 120.0);
        if sprite != Texture2D::empty()
        {
            size = vec2(sprite.width(), sprite.height()) * 3.0;
        }

        let color = WHITE;

        let weapon = None;

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
        params.health = 3.0;
        params.speed = 200.0;
        params.armor = 2.0;
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

