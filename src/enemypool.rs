use super::*;
use macroquad::rand::{gen_range, RandomRange};
pub struct EnemyPool
{
    pool: Vec<Enemy>,
    active_pool: Vec<Enemy>
}
impl EnemyPool
{
    pub fn new() -> Self
    {
        Self { 
            pool: Vec::new(), 
            active_pool: Vec::new() 
        }
    }
    pub fn create_pool(&mut self, count: usize, world: &mut World)
    {
        self.pool.clear();
        self.active_pool.clear();
        for i in 0..count
        {
            let mut enemy = Enemy::new(i, world);
            enemy.reset();
            world.set_entity(&mut enemy.entity);
            self.pool.push(enemy);
        }
    }
    pub fn spawn_enemy(&mut self, spawner_position: Vec2, spawner_type: &EnemyType, world: &mut World)
    {
        let free_slot = self.get_free_slot();
        match free_slot
        {
            Some(slot) => {

                self.pool[slot].set_enemytype(&spawner_type, world);
                self.pool[slot].init(world);
                self.pool[slot].entity.SetActive(true);
                self.pool[slot].entity.transform.set_position( spawner_position);

                self.active_pool.push(self.pool[slot].clone());
                world.set_entity(&mut self.pool[slot].entity);
            }
            None => {
                println!("No Free Enemy Slot Found!");
            }
        }
    }
    fn get_free_slot(&self) -> Option<usize>
    {
        for i in 0..self.pool.len()
        {
            if !self.pool[i].entity.is_active
            {
                return Some(i);
            }
        }
        return None
    }

    pub fn enemy_shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        for enemy in self.pool.iter_mut()
        {
            if enemy.entity.is_active  && enemy.variant.has_weapon()
            {
                enemy.shoot(misslepool,world);
            }
        }
    }
}
impl GameObject for EnemyPool
{
    fn init(&mut self,world: &mut World) {
    }
    fn update(&mut self, world: &mut World) {

        for enemy in self.pool.iter_mut()
        {
            if enemy.entity.is_active
            {
                if !inside_windowborder_extended_sides(enemy.entity.transform.rect, world.level_offset, 200.0, vec2(200.0, 600.0))
                {
                    enemy.reset();
                    enemy.entity.is_active = false;
                    world.set_entity(&mut enemy.entity);
                }else {
                    enemy.update(world);
                    world.set_entity(&mut enemy.entity);
                }
            }
        }
    }
    fn late_update(&mut self, world: &mut World) {
        for enemy in self.pool.iter_mut()
        {
            for entity in world.get_actives().iter_mut()
            {
                enemy.on_collision(entity);
            }
        }
    }
    fn draw(&mut self) {

        for enemy in self.active_pool.iter_mut()
        {
            self.pool[enemy.enemy_id].draw();
        }
    }
}