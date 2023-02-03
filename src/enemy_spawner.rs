use macroquad::rand::{gen_range, RandomRange};

use super::*;

pub struct EnemySpawner
{
    pub pool: Vec<Enemy>,
    active_pool: Vec<Enemy>,

    spawn_time: f32,
    timer: f32,
}
impl EnemySpawner
{
    pub fn new() -> Self
    {
        Self {
            pool: Vec::new(),
            active_pool: Vec::new(),
            spawn_time: 1.0,
            timer: 0.0,
        }
    }

    pub fn create_pool(&mut self, count: usize, world: &mut World)
    {
        self.pool.clear();
        self.active_pool.clear();
        for i in 0..count
        {
            let mut enemy = Enemy::new(i,world);
            enemy.entity.SetActive(false);
            world.set_entity(&mut enemy.entity);
            self.pool.push(enemy);
        }
    }
    pub fn spawn_enemy(&mut self, world: &mut World)
    {
        if self.timer <= 0.0
        {
            let free_slot = self.get_free_slot();
            match free_slot
            {
                Some(slot) => { self.activate_enemy(slot, world); }
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
   
    fn activate_enemy(&mut self, slot: usize, world: &mut World)
    {
        let enemy = &mut self.pool[slot]; 
        enemy.entity.SetActive(true);
        EnemySpawner::set_enemytype(enemy, world);
        
        let random_position = gen_range(0, 4);
        let mut outer_position = vec2( 0.0 , 0.0);

        // Select Border
        match random_position
        {
            //LEFT
            0 => { outer_position = vec2( 0.0 - enemy.entity.transform.get_fullsize().x, gen_range( 0.0 , GAME_SIZE_Y as f32)); }
            //TOP
            1 => { outer_position = vec2( gen_range( 0.0 , GAME_SIZE_X as f32), 0.0 - enemy.entity.transform.get_fullsize().y ); }
            //RIGHT
            2 => { outer_position = vec2( GAME_SIZE_X as f32 + enemy.entity.transform.get_fullsize().x, gen_range( 0.0 , GAME_SIZE_Y as f32)); }
            //BOTTOM
            3 => { outer_position = vec2( gen_range( 0.0 , GAME_SIZE_X as f32), GAME_SIZE_Y as f32 + enemy.entity.transform.get_fullsize().y ); }
            _ => { outer_position = vec2( 0.0 - enemy.entity.transform.get_fullsize().x, gen_range( 0.0 , GAME_SIZE_Y as f32));}
        }


        enemy.entity.transform.set_position(outer_position);
        self.active_pool.push(enemy.clone());
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
    fn set_enemytype(enemy: &mut Enemy,world: &mut World)
    {
        let t32 = RandomRange::gen_range(0, 3);
            let mut tt = EnemyType::Default;
            match t32
            {
                0 => {tt = EnemyType::Default;}
                1 => {tt = EnemyType::Heavy;}
                2 => {tt = EnemyType::Gunner;}
                _ => {tt = EnemyType::Default;}
            }
            enemy.set_enemytype(tt, world);
            enemy.init(world);
    }
}

impl GameObject for EnemySpawner
{
    fn init(&mut self,world: &mut World) {
        for enemy in self.pool.iter_mut()
        {
            EnemySpawner::set_enemytype(enemy,world);
        }
    }
    fn update(&mut self, world: &mut World) {
        self.spawn_enemy(world);
        for enemy in self.pool.iter_mut()
        {
            if enemy.entity.is_active
            {
                enemy.update(world);
                world.set_entity(&mut enemy.entity);
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
        draw_text(format!("Next Enemy: {}", self.timer).as_str(), 30.0, 30.0, 16.0, BLACK);
        for enemy in self.active_pool.iter_mut()
        {
            self.pool[enemy.enemy_id].draw();
        }
    }
}