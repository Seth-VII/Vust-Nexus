use macroquad::rand::{gen_range, RandomRange};
use super::*;

pub struct EnemySpawnerPool
{
    pool: Vec<EnemySpawner>
}
impl EnemySpawnerPool
{
    pub fn new () -> Self { 

        Self { 
            pool: Vec::new(),
        } 
    }
    pub fn apply_spawnerpool( &mut self, pool: &Vec<EnemySpawner>)
    {
        self.pool = pool.clone();
    }

    pub fn update(&mut self, enemypool: &mut EnemyPool, world: &mut World)
    {
        for spawner in self.pool.iter_mut()
        {
            if inside_windowborder(spawner.entity.transform.rect, world.level_offset, 200.0) 
            {

                if spawner.spawned < spawner.spawn_count
                { 
                    if spawner.time > 0.0
                    {
                        spawner.time -= 1.0 * get_frame_time();
                    }else {
                        spawner.time = spawner.spawn_duration;
                        // Spawn
                        enemypool.spawn_enemy( spawner.entity.transform.get_centered_position(), &spawner.spawner_type, world);
                        spawner.spawned += 1;
                    }
                }
                println!("{}", spawner.spawned );
                world.set_entity(&mut spawner.entity);
            }  
        }
    }
    pub fn late_update(&mut self, world: &mut World) {
        // Only activate inside Windowborder
        for spawner in self.pool.iter_mut()
        {
            if inside_windowborder(spawner.entity.transform.rect, world.level_offset, 200.0) 
            {
                spawner.entity.is_active = true; 
                world.set_entity(&mut spawner.entity);
            }
        }
    }
}

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
            1 => { EnemyType::Heavy},
            2 => { EnemyType::Gunner},
            _ => { EnemyType::Default},
        };

        Self {
            entity: entity,
            spawner_type: enemy_type,
            spawn_count: count,
            spawned: 0,
            spawn_duration: 3.0,
            time: 3.0,
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
        enemypool.spawn_enemy( self.entity.transform.get_centered_position(), &self.spawner_type, world);
    }
}