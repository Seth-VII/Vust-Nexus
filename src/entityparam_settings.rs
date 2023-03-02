use super::*;

pub struct SavedEntitySettings{
    entity_settings: EntityParams,
}
impl SavedEntitySettings
{
    pub fn new() -> Self 
    {
        Self { entity_settings: EntityParams::default() }
    }
    pub fn save(&mut self ,params: EntityParams)
    {
        self.entity_settings = params;
    }
    pub fn get_settings(&self) -> EntityParams
    {
        self.entity_settings
    }
}

pub struct EntitySettings{}
impl EntitySettings{
    pub fn player_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 100.0;
        params.armor = 4.0;
        params.speed = 350.0;
        params.damage = 4.0;
        params.firerate = 60.0;
        params.firespeed = 700.0;
        return params;
    }

    // ---------------------------------
    // Level Tiles
    pub fn spawner_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0 + (1.0 * world.difficulty_level as f32);
        params.armor = 5.0;
        return params;
    }
    pub fn trap_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0 + (1.0 * world.difficulty_level as f32);
        params.armor = 3.5 + (0.5 * world.difficulty_level as f32);
        params.damage = 2.5 + (1.35 * world.difficulty_level as f32);
        return params;
    }
    pub fn destructible_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0 + (1.0 * world.difficulty_level as f32);
        params.armor = 3.0 + (0.5 * world.difficulty_level as f32);
        return params;
    }
    pub fn turret_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        params.damage = 3.0;
        params.firerate = 10.0;
        params.firespeed = 300.0;
        return params;
    }

    // ---------------------------------
    // Enemies

    pub fn enemy_default_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0 + (1.5 * world.difficulty_level as f32);
        params.speed = 150.0 + (5.25 * world.difficulty_level as f32);
        params.armor = 2.0 + (0.3 * world.difficulty_level as f32);
        params.damage = 2.0 + (0.5 * world.difficulty_level as f32);
        return params;
    }
    pub fn enemy_tank_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 20.0 + (3.0 * world.difficulty_level as f32);
        params.speed = 120.0 + (4.0 * world.difficulty_level as f32);
        params.armor = 5.0 + (0.5 * world.difficulty_level as f32);
        params.damage = 3.0 + (0.3 * world.difficulty_level as f32);
        params.firerate = 0.7 + (0.5 * world.difficulty_level as f32);
        params.firespeed = 300.0 + (3.0 * world.difficulty_level as f32);
        return params;
    }
    pub fn enemy_gunner_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 3.0 + (1.0 * world.difficulty_level as f32);
        params.speed = 150.0 + (5.25 * world.difficulty_level as f32);
        params.armor = 2.0 + (0.3 * world.difficulty_level as f32);
        params.damage = 1.0 + (0.5 * world.difficulty_level as f32);
        params.firerate = 1.3 + (0.6 * world.difficulty_level as f32);
        params.firespeed = 300.0 + (3.0 * world.difficulty_level as f32);
        return params;
    }
    pub fn enemy_heavygunner_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;
        return params;
    }
    pub fn enemy_exploder_settings( world: &mut World) -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;
        return params;
    }
    pub fn enemy_boss_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;
        return params;
    }
}