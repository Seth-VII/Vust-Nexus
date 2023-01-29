use macroquad::rand::gen_range;

use super::*;

pub struct EnemySpawner
{
    pub pool: Vec<Enemy>,

    spawn_time: f32,
    timer: f32,
}
impl EnemySpawner
{
    pub fn new() -> Self
    {
        Self {
            pool: Vec::new(),
            spawn_time: 0.0,
            timer: 0.0,
        }
    }

    pub fn create_pool(&mut self, count: usize, world: &mut World)
    {
        for _i in 0..count
        {
            let mut enemy = Enemy::new(world);
            enemy.entity.SetActive(false);
            world.set_entity(&mut enemy.entity);
            self.pool.push(enemy);
        }
    }
    pub fn spawn_enemy(&mut self)
    {
        if self.timer <= 0.0
        {
            let free_slot = self.get_free_slot();
            match free_slot
            {
                Some(slot) => { self.activate_enemy(slot); }
                None => {}
            }
            self.timer = self.spawn_time;
        }else {
            self.timer -= get_frame_time();
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
   
    fn activate_enemy(&mut self, slot: usize)
    {
        let enemy = &mut self.pool[slot]; 
        enemy.entity.SetActive(true);
        
        let random_position = vec2(gen_range(0.0, GAME_SIZE_X as f32), gen_range(0.0, GAME_SIZE_Y as f32));
        enemy.entity.transform.set_position(random_position);
    }
}

impl GameObject for EnemySpawner
{
    fn init(&mut self) {
        for enemy in self.pool.iter_mut()
        {
            enemy.init();
        }
    }
    fn update(&mut self, world: &mut World) {
        self.spawn_enemy();
        for enemy in self.pool.iter_mut()
        {
            enemy.update(world);
        }
    }
    fn late_update(&mut self, world: &mut World) {
        for enemy in self.pool.iter_mut()
        {
            for entity in world.get_actives().iter_mut()
            {
                enemy.on_collision(entity);
                //world.set_entity(&mut entity.clone());
            }
        }
    }
    fn draw(&mut self) {
        draw_text(format!("Next Enemy: {}", self.timer).as_str(), 30.0, 30.0, 16.0, BLACK);
        for enemy in self.pool.iter_mut()
        {
            enemy.draw();
        }
    }
}