use macroquad::rand::{gen_range, RandomRange};
use super::*;


#[derive(Clone)]
pub struct EnemySpawner
{
    pub entity: Entity,
    spawner_type: EnemyType,
    spawn_count: usize,
    spawned: usize,
    spawn_duration: f32,
    time: f32,
}
impl EnemySpawner
{
    pub fn create_spawner( count: usize, spawner_type: usize, world: &mut World) -> Self
    {
        let mut entity = Entity::new("EnemySpawner", "Spawner", world);

        let enemy_type = match spawner_type {
            0 => { EnemyType::Default},
            1 => { EnemyType::Tank},
            2 => { EnemyType::Gunner},
            3 => { EnemyType::HeavyGunner},
            4 => { EnemyType::Exploder},
            5 => { EnemyType::Boss},
            _ => { EnemyType::Default},
        };

        Self {
            entity: entity,
            spawner_type: enemy_type,
            spawn_count: count,
            spawned: 0,
            spawn_duration: 2.0,
            time: 0.2,
        }
    }
    pub fn set_transform(&mut self, transform: &Transform)
    {
        self.entity.transform = transform.clone();
    }
    pub fn update(&mut self, enemypool: &mut EnemyPool, world: &mut World) {

        if self.spawned < self.spawn_count
        { 
            if self.time > 0.0
            {
                self.time -= 1.0 * get_frame_time();
            }else {
                self.time = self.spawn_duration;
                // Spawn
                self.spawn(enemypool, world);
                self.spawned += 1;
            }
            //println!("{}", self.time );
        }
        world.set_entity(&mut self.entity);
        //println!("{} / {}", self.spawned , self.spawn_count);
    }
    pub fn spawn(&mut self ,enemypool: &mut EnemyPool, world: &mut World)
    {
        //println!("spawn");
        enemypool.spawn_enemy( self.entity.transform.position + self.entity.transform.get_halfsize(), &self.spawner_type, world);
    }
}