use macroquad::audio::{play_sound, PlaySoundParams};

use super::*;
pub struct Game
{
    late_tick : f32,
    fixed_tick: f32,

    local_score: i32,
    last_score: i32,
    high_score: i32,

    gamestate: GameState,
    viewspace: Viewspace,
    pub world: World,

    misslepool: MisslePool,
    enemy_spawner: EnemySpawner,
    player: Player,
}
impl Game {
    pub fn Run(&mut self)
    {
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
                    self.world.particlesystem_pool.update();
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
        }
    }
    pub fn restart(&mut self)
    {
        self.world.reload();

        self.misslepool = MisslePool::new();
        self.misslepool.create_pool(512, &mut self.world);
        
        self.enemy_spawner = EnemySpawner::new();
        self.enemy_spawner.create_pool(32, &mut self.world);
        self.enemy_spawner.init(&mut self.world);

        self.player = Player::new(&mut self.world);
        self.player.init(&mut self.world);

        //self.world.reset_scorepoints();
        self.local_score = 0;
        self.gamestate = GameState::GameRunning;
    }
    pub async fn init() -> Self
    {
        let mut world = World::new().await;
        world.load_levels().await;

        let viewspace = Viewspace::new();

        let mut misslepool = MisslePool::new();
        let mut enemy_spawner = EnemySpawner::new();
        let mut player = Player::new(&mut world);
        
        
        misslepool.create_pool(512, &mut world);
        enemy_spawner.create_pool(32, &mut world);
        enemy_spawner.init(&mut world);
        player.init(&mut world);

        Self {
            late_tick: 0.0,
            fixed_tick: 0.0,

            local_score: 0,  
            last_score: 0,
            high_score: 0,

            gamestate: GameState::MainMenu,
            viewspace: viewspace,
            world: world,

            misslepool: misslepool,
            enemy_spawner: enemy_spawner,
            player: player,
        }

    }
    pub fn update(&mut self)
    {
        self.world.update_actives();
        self.viewspace.set_position(self.player.entity.transform.position);

        self.misslepool.update(&mut self.world);
        self.enemy_spawner.update(&mut self.world);
        self.enemy_spawner.enemy_shoot(&mut self.misslepool, &mut self.world);
        self.player.update(&mut self.world);
        self.player.shoot(&mut self.misslepool, &mut self.world);

    }
    pub fn late_update(&mut self)
    {
        self.world.update_level(&mut self.misslepool);
        
        self.enemy_spawner.late_update(&mut self.world);
        self.player.late_update(&mut self.world);
        self.misslepool.late_update(&mut self.world);
    }
    pub fn draw(&mut self)
    {
        self.world.levels[0].draw();

        
        self.viewspace.draw();
        self.enemy_spawner.draw();
        
        self.player.draw();
        self.misslepool.draw();



        let text = format!("Local Score: {}", self.local_score);
        let text_size =  50.0;
        let text_width = text.chars().count() as f32 * text_size;
        let centered_position = ( GAME_SIZE_X as f32 * 0.5) - ( text_width * 0.2);
        draw_text(text.as_str(),centered_position, 60.0, text_size, WHITE);
    }

    pub fn update_score(&mut self)
    {
        self.local_score = self.world.get_collected_scorepoints();
    }
}