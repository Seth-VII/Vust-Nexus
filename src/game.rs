use macroquad::audio::{play_sound, PlaySoundParams};

use super::*;
pub struct Game
{
    late_tick : f32,
    fixed_tick: f32,

    camera: Camera2D,

    local_score: i32,
    last_score: i32,
    high_score: i32,

    gamestate: GameState,
    viewspace: Viewspace,
    pub world: World,

    pub level_loader: LevelLoader,
    pub available_levels: usize,
    pub level: Option<Level>,
    selected_level: usize,

    misslepool: MisslePool,
    enemypool: EnemyPool,
    spawnerpool: EnemySpawnerPool,
    //enemy_spawner: EnemySpawner,
    player: Player,
}
impl Game {

    pub fn Run(&mut self)
    {

        if self.world.level_completed && self.gamestate != GameState::LevelCompleted {
            self.gamestate = GameState::LevelCompleted;
            self.world.level_offset = 0.0;
            println!("Level Completed!");
        }

        if self.level.is_none()
        {
            self.load_level();
        }

        draw_text(format!("HP: {}",self.player.entity.entity_params.health).as_str(), 30.0 + self.world.level_offset, 90.0, 30.0, WHITE);
        //println!("{:?}", self.gamestate);
        match self.gamestate
        {
            GameState::MainMenu => {
                if is_key_released(KeyCode::Space)
                {
                    self.gamestate = GameState::GameRunning;
                }
                let text = "Press Space to Start!";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position, GAME_SIZE_Y as f32 * 0.5, text_size, WHITE);


                

            }
            GameState::GamePaused => {
                if is_key_released(KeyCode::Tab)
                {
                    self.gamestate = GameState::GameRunning;
                }
                self.draw();

                let text = "Paused! Press TAB again!";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position, GAME_SIZE_Y as f32 * 0.5, text_size, WHITE);
            }
            GameState::GameRunning => {
                
                if is_key_released(KeyCode::Tab)
                {
                    self.gamestate = GameState::GamePaused;
                }

                self.update();
                
                // Fixed Update -> for heavy stuff but needs more updates
                if self.fixed_tick <= 0.0
                {
                    self.fixed_update();
                    self.fixed_tick = FIXED_UPDATE_TICK;
                }else 
                {
                    self.fixed_tick -= get_frame_time();
                }

                // Later Update -> for heavy stuff
                if self.late_tick <= 0.0
                {
                    self.late_update();
                    self.late_tick = LATE_UPDATE_TICK;
                }else 
                {
                    self.late_tick -= get_frame_time();
                }

                if self.player.entity.entity_params.health <= 0.0
                {
                    self.gamestate = GameState::GameOver;
                    self.world.level_offset = 0.0;

                    let mut params = PlaySoundParams::default();
                    params.volume = 0.5;
                    play_sound(self.world.assets.get_asset_by_name("explosion_1".to_string()).unwrap().get_sound_data().sound.unwrap(), params );

                }
                
                self.draw();
                self.world.particlesystem_pool.draw();
                self.update_score();
            }
            GameState::GameOver => {

                if self.local_score > self.high_score
                {
                    self.high_score = self.local_score;
                }
                self.world.level_completed = false;
                self.world.level_offset = 0.0;
                let level_position = vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5);
                self.camera.target = level_position;
                set_camera(&self.camera);

                let text = "GAME OVER";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position, GAME_SIZE_Y as f32 * 0.5 - 80.0, text_size, WHITE);

                // Local Score
                let text = format!("Score: {}", self.local_score);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 + 0.0, text_size, WHITE);

                // Last Score
                let text = format!("Last Score: {}", self.last_score);
                let text_size =  25.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 + 30.0, text_size, WHITE);
                
                // High Score
                let text = format!("High Score: {}", self.high_score);
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 + 100.0, text_size, WHITE);

                if is_key_pressed(KeyCode::Space)
                {
                    self.last_score = self.local_score;
                    self.restart();
                    return;
                }
            }
            GameState::LevelCompleted => {
                self.world.level_completed = false;
                self.world.level_offset = 0.0;
                let level_position = vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5);
                self.camera.target = level_position;
                set_camera(&self.camera);

                // Win Screen
                let text = format!("Level Completed");
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 - 100.0, text_size, WHITE);

                // Local Score
                let text = format!("Current Score: {}", self.local_score);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 + 30.0, text_size, WHITE);

                // Current Level
                let text = format!("Level {} / {}", self.selected_level, 10);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y as f32 * 0.5 + 0.0, text_size, WHITE);


                if is_key_pressed(KeyCode::Space)
                {
                    self.next_level();
                    return;
                }
            }
        }
    }

    pub fn restart(&mut self)
    {
        self.world.reload();
        self.load_level();

        self.misslepool = MisslePool::new();
        self.misslepool.create_pool(512, &mut self.world);
        
        self.enemypool = EnemyPool::new();
        self.enemypool.create_pool(128, &mut self.world);
        //self.enemy_spawner = EnemySpawner::new();
        //self.enemy_spawner.create_pool(32, &mut self.world);
        //self.enemy_spawner.init(&mut self.world);

        self.player = Player::new(&mut self.world);
        self.player.init(&mut self.world);

        //self.world.reset_scorepoints();
        self.local_score = 0;
        self.gamestate = GameState::GameRunning;
    }
    pub async fn init() -> Self
    {
        let mut world = World::new().await;
        //world.load_level();

        let mut loader = LevelLoader::new();
        loader.level_loader_init().await;

        println!("Count: {}", world.entities.len());
        let viewspace = Viewspace::new();

        let camera_rect = Rect::new(0.0,0.0, GAME_SIZE_X as f32 , GAME_SIZE_Y as f32 );
        let camera = Camera2D::from_display_rect(camera_rect);
        set_camera(&camera);


        let mut misslepool = MisslePool::new();
        misslepool.create_pool(512, &mut world);


        let mut enemypool = EnemyPool::new();
        enemypool.create_pool(128, &mut world);

        let mut spawnerpool = EnemySpawnerPool::new();
        //spawnerpool.apply_spawnerpool( &world.level.as_ref().unwrap().get_spawners());
        //let mut enemy_spawner = EnemySpawner::new();
        let mut player = Player::new(&mut world);
        
        //enemy_spawner.create_pool(32, &mut world);
        //enemy_spawner.init(&mut world);
        player.init(&mut world);



        Self {
            late_tick: 0.0,
            fixed_tick: 0.0,

            local_score: 0,  
            last_score: 0,
            high_score: 0,

            gamestate: GameState::MainMenu,
            viewspace: viewspace,
            camera: camera,
            world: world,

            available_levels: loader.levels.len(),
            level_loader: loader,
            level: None,
            selected_level: 0,

            misslepool: misslepool,
            enemypool: enemypool,
            spawnerpool: spawnerpool,
            //enemy_spawner: enemy_spawner,
            player: player,
        }

    }
    pub fn update(&mut self)
    {
        self.world.update_actives();
        self.viewspace.set_position(self.player.entity.transform.position);
        
        let level_position = vec2( GAME_SIZE_X as f32 * 0.5 + self.world.level_offset , GAME_SIZE_Y as f32 * 0.5);
        self.camera.target = level_position;

        set_camera(&self.camera);
        self.level_update();

        self.misslepool.update(&mut self.world);
        
        self.enemypool.update(&mut self.world);
        //self.enemy_spawner.update(&mut self.world);
        //self.enemy_spawner.enemy_shoot(&mut self.misslepool, &mut self.world);
        self.player.update(&mut self.world);
        self.player.shoot(&mut self.misslepool, &mut self.world);

    }
    pub fn fixed_update(&mut self)
    {
        self.level_fixed_update();
        self.world.fixed_update();
    }
    pub fn late_update(&mut self)
    {
        self.level_late_update();
        
        //self.enemy_spawner.late_update(&mut self.world);
        self.player.late_update(&mut self.world);
        self.misslepool.late_update(&mut self.world);
        self.enemypool.late_update(&mut self.world);
    }
    pub fn draw(&mut self)
    {
        self.world.level.as_mut().unwrap().draw();

        
        self.viewspace.draw();
        //self.enemy_spawner.draw();
        
        self.player.draw();
        self.misslepool.draw();
        self.enemypool.draw();
        
        let text = format!("Local Score: {}", self.local_score);
        let text_size =  50.0;
        let text_width = text.chars().count() as f32 * text_size;
        let centered_position_x = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2) + self.world.level_offset;
        draw_rectangle(0.0 + self.world.level_offset, 0.0, GAME_SIZE_X as f32 + self.world.level_offset, 80.0, BLACK);

        draw_text(text.as_str(),centered_position_x, 60.0, text_size, WHITE);
    }

    pub fn update_score(&mut self)
    {
        self.local_score = self.world.get_collected_scorepoints();
    }

    pub fn next_level(&mut self) {
        self.world.reload();
        println!("selected {} / available {}", self.selected_level, self.available_levels);
        if self.selected_level < self.available_levels -1
        {
            self.selected_level += 1;
            self.load_level();
        }

        println!("Count: {}", self.world.entities.len());
        self.misslepool = MisslePool::new();
        self.misslepool.create_pool(512, &mut self.world);

        self.enemypool = EnemyPool::new();
        self.enemypool.create_pool(128, &mut self.world);

        self.player = Player::new(&mut self.world);
        self.player.init(&mut self.world);
        self.gamestate = GameState::GameRunning;
    }

    
    pub fn load_level(&mut self)
    {
        println!("Loader Data: {}", self.level_loader.levels[SELECTED_LEVEL].enemy_spawner[0].1 );
        let mut level = Level::new(&mut self.world, self.level_loader.levels[SELECTED_LEVEL].clone());
        level.init(&mut self.world);
        self.level = Some(level.clone());
        self.world.level =  Some(level);
    }

    pub fn level_update(&mut self)
    {
        //self.level.as_mut().unwrap().late_update(self, misslepool);
        if self.level.is_some() {
            let mut lvl = self.level.as_mut().unwrap().clone();
            lvl.update(&mut self.world);
            lvl.spawer_update( &mut  self.enemypool, &mut  self.world);
            self.level = Some(lvl.clone());
            self.world.level =  Some(lvl);
        }
    }
    pub fn level_fixed_update(&mut self)
    {
        //self.level.as_mut().unwrap().late_update(self, misslepool);
        /*
        if self.level.is_some() {
            let mut lvl = self.level.as_mut().unwrap().clone();
            lvl.spawner_update(&mut self.enemypool, &mut self.world);
            self.level = Some(lvl.clone());
            self.world.level =  Some(lvl);
        }
        */
    }
    pub fn level_late_update(&mut self)
    {
        //self.level.as_mut().unwrap().late_update(self, misslepool);
        if self.level.is_some() {
            let mut lvl = self.level.as_mut().unwrap().clone();
            lvl.late_update(&mut self.world, &mut self.misslepool);
            self.level = Some(lvl.clone());
            self.world.level =  Some(lvl);
        }
    }
}