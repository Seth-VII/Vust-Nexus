use macroquad::prelude::*;

pub const GAME_SIZE_X: i32 = 1280;
pub const GAME_SIZE_Y: i32 = 720;

// Engine
//------------------
mod world;
pub use world::*;

mod gameobject;
pub use gameobject::*;

mod collision;
pub use collision::*;

mod entity;
pub use entity::*;

// Game
//------------------
mod player;
pub use player::*;

mod enemy;
pub use enemy::*;

mod enemy_spawner;
pub use enemy_spawner::*;

mod weapon;
pub use weapon::*;

mod missle;
pub use missle::*;

fn window_conf() -> Conf
{

    let app_conf : Conf = Conf 
    {
        window_title        : "RustDefenders".to_string(),
        window_width        : GAME_SIZE_X,
        window_height       : GAME_SIZE_Y,
        high_dpi            : false,
        fullscreen          : false,
        sample_count        : 4,
        window_resizable    : true,
        ..Default::default()
    };
    app_conf
}

#[macroquad::main(window_conf)]
async fn main() {
    let late_update_tick = 0.05;
    let mut current_tick = 0.0;

    let mut game = Game::init();

    loop {
        clear_background(WHITE);
        
        game.update();
        
        if current_tick <= 0.0
        {
            game.late_update();
            current_tick = late_update_tick;
        }else 
        {
            current_tick -= get_frame_time();
        }

        game.draw();
        game.update_score();
        //println!("Active Entities: {}", world.get_actives().len());
        draw_text(format!("FPS: {}", get_fps()).as_str(), 30.0, 60.0, 30.0, BLACK);
        //let world_entity = world.get_entity_by_tag("Player").unwrap();
        //println!("World Entity {}", world_entity.tranform.position);
        //println!("World {}", world.entities.len());
        next_frame().await;
    }
}

pub struct Game
{
    local_score: i32,
    last_score: i32,

    world: World,
    misslepool: MisslePool,
    enemy_spawner: EnemySpawner,
    player: Player,
    player_weapon: Weapon,
}
impl Game {
    
    pub fn init() -> Self
    {
        let mut world = World::new();
        let mut misslepool = MisslePool::new();
        let mut enemy_spawner = EnemySpawner::new();
        let mut player = Player::new(&mut world);
        let mut player_weapon = Weapon::new("Player Weapn", "Player Weapon",&mut world);
        
        
        misslepool.create_pool(512, &mut world);
        enemy_spawner.create_pool(32, &mut world);
        enemy_spawner.init(&mut world);
        player.init(&mut world);
        player_weapon.init(&mut world);
        player_weapon.set_stats(2.0, 20.0, 300.0);

        Self {
            local_score: 0,  
            last_score: 0,

            world: world,
            misslepool: misslepool,
            enemy_spawner: enemy_spawner,
            player: player,
            player_weapon: player_weapon,
        }

    }
    pub fn update(&mut self)
    {
        self.world.update_actives();
        
        self.misslepool.update(&mut self.world);
        self.enemy_spawner.update(&mut self.world);
        self.enemy_spawner.enemy_shoot(&mut self.misslepool, &mut self.world);
        self.player.update(&mut self.world);
        

        if is_key_down(KeyCode::Space)
        {
            self.player_weapon.shoot(&mut self.misslepool,&mut self.world);
        }
        self.player_weapon.set_parent(Some(self.player.entity.clone()));
        self.player_weapon.update(&mut self.world);

    }
    pub fn late_update(&mut self)
    {
        for enemy in self.enemy_spawner.pool.iter_mut() {

            self.player.on_collision(&mut enemy.entity);
        }
        self.misslepool.late_update(&mut self.world);
        self.enemy_spawner.late_update(&mut self.world);
        self.player.late_update(&mut self.world);
        self.player_weapon.late_update(&mut self.world);
    }
    pub fn draw(&mut self)
    {
        self.misslepool.draw();
        self.enemy_spawner.draw();
        self.player.draw();
        self.player_weapon.draw();

        draw_text(format!("Local Score: {}", self.local_score).as_str(), GAME_SIZE_X as f32 * 0.5, 60.0, 60.0, BLACK);
    }

    pub fn update_score(&mut self)
    {
        self.local_score = self.world.get_collected_scorepoints();
    }
}