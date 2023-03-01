use super::*;

pub struct EntitySettings{}
impl EntitySettings{

    pub fn player_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 1000000.0;
        params.armor = 3.0;
        params.speed = 250.0;
        params.damage = 3.0;
        params.firerate = 60.0;
        params.firespeed = 1000.0;
        return params;
    }

    // ---------------------------------
    // Level Tiles
    pub fn spawner_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        return params;
    }
    pub fn trap_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 50.0;
        params.armor = 3.0;
        params.damage = 100000.0;
        return params;
    }
    pub fn destructible_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        return params;
    }
    pub fn turret_settings() -> EntityParams
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

    pub fn enemy_default_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;
        return params;
    }
    pub fn enemy_tank_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.speed = 170.0;
        params.armor = 5.0;
        params.damage = 3.0;
        return params;
    }
    pub fn enemy_gunner_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 3.0;
        params.speed = 200.0;
        params.armor = 2.0;
        params.damage = 1.0;
        params.firerate = 1.0;
        params.firespeed = 450.0;
        return params;
    }
    pub fn enemy_heavygunner_settings() -> EntityParams
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 250.0;
        params.armor = 2.0;
        params.damage = 2.0;
        return params;
    }
    pub fn enemy_exploder_settings() -> EntityParams
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