use macroquad::audio::{play_sound, PlaySoundParams};

use super::*;
pub struct Game
{
    late_tick : f32,
    fixed_tick: f32,

    camera: Camera2D,
    render_target: RenderTarget,


    local_score: i32,
    last_score: i32,
    high_score: i32,

    gamestate: GameState,
    //viewspace: Viewspace,
    pub world: World,

    pub level_loader: LevelLoader,
    pub available_levels: usize,
    pub level: Option<Level>,
    selected_level: usize,
    level_transition: BlackBlend,

    misslepool: MisslePool,
    enemypool: EnemyPool,

    player: Player,
}
impl Game {

    pub fn Run(&mut self)
    {
        // Setup Camera
        self.init_camera();

        // Is Level Finished
        if self.world.level_completed {
            self.gamestate = GameState::Transition;
            self.level_transition.blend_in();
            self.level_transition.update_blend(self.world.level_offset);
            println!("Level Completed!");
        }

        // Safety Check for Level
        if self.level.is_none()
        {
            self.load_level();
        }

        // Draw Health Points
        draw_text(format!("HP: {}",self.player.entity.entity_params.health).as_str(), 30.0 , 90.0, 30.0, WHITE);

        match self.gamestate
        {
            GameState::Transition => {


                if self.level_transition.get_is_playing(){
                    // While In Blend
                    self.update();
                    self.late_update();
                    self.draw();
                    self.level_transition.update_blend(self.world.level_offset);
                }else{
                    // On Blend Finished
                    self.end_level();
                    self.next_level();
                    self.level_transition.set_start_blend(BlendingType::BlendOut, 1.0);
                }
            }
            GameState::MainMenu => {
                if is_key_released(KeyCode::Space) || is_mouse_button_released(MouseButton::Left)
                {
                    self.gamestate = GameState::GameRunning;
                    self.level_transition.set_start_blend(BlendingType::BlendOut, 1.0);
                }
                // UI
                let text = "Press [Space or Left Mousebutton] to Start!";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position, GAME_SIZE_Y * 0.5, text_size, WHITE);


                

            }
            GameState::GamePaused => {
                if is_key_released(KeyCode::Tab) 
                {
                    self.gamestate = GameState::GameRunning;
                }
                // Camera Position
                let level_position = vec2( GAME_SIZE_X * 0.5 + self.world.level_offset , GAME_SIZE_Y * 0.5);
                self.camera.target = level_position;
                set_camera(&self.camera);
                // Draw Game
                self.draw();

                // Pause UI
                draw_rectangle(self.world.level_offset, 0.0 , GAME_SIZE_X, GAME_SIZE_Y, color_u8!(0,0,0,220));
                let text = "Paused! Press TAB again!";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position + self.world.level_offset, GAME_SIZE_Y * 0.5, text_size, WHITE);
            }
            GameState::GameRunning => {
                // Pause Game
                if is_key_released(KeyCode::Tab)
                {
                    self.gamestate = GameState::GamePaused;
                }

                // Update Game -> Every possible Frame
                self.update();
                
                // Fixed Update -> Mostly used for the Particle Updates
                if self.fixed_tick <= 0.0
                {
                    self.fixed_update();
                    self.fixed_tick = FIXED_UPDATE_TICK;
                }else 
                {
                    self.fixed_tick -= get_frame_time();
                }

                // Later Update -> for heavy stuff (Collision)
                if self.late_tick <= 0.0
                {
                    self.late_update();
                    self.late_tick = LATE_UPDATE_TICK;
                }else 
                {
                    self.late_tick -= get_frame_time();
                }
                
                // Check for Players health
                if self.player.entity.entity_params.health <= 0.0
                {
                    self.gamestate = GameState::GameOver;
                    self.world.level_offset = 0.0;
                    
                    let mut params = PlaySoundParams::default();
                    params.volume = 0.5;
                    play_sound(self.world.assets.get_asset_by_name("explosion_1".to_string()).unwrap().get_sound_data().sound.unwrap(), params );
                    
                    
                }

                // Draw Game
                self.draw();

                // Update Scorepoints
                self.update_score();   
                
            }
            GameState::GameOver => {
                
                // Update Highscore
                if self.local_score > self.high_score
                {
                    self.high_score = self.local_score;
                }
                // Reset Level
                self.end_level();

                // ---------------GameOver UI
                // GameOver Text
                let text = "GAME OVER";
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text, centered_position, GAME_SIZE_Y * 0.5 - 80.0, text_size, WHITE);

                // Local Score
                let text = format!("Score: {}", self.local_score);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 + 0.0, text_size, WHITE);

                // Last Score
                let text = format!("Last Score: {}", self.last_score);
                let text_size =  25.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 + 30.0, text_size, WHITE);
                
                // High Score
                let text = format!("High Score: {}", self.high_score);
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 + 100.0, text_size, WHITE);

                // Restart Game
                if is_key_pressed(KeyCode::Space) || is_mouse_button_released(MouseButton::Left)
                {
                    self.last_score = self.local_score;
                    self.restart();
                    self.level_transition.set_start_blend(BlendingType::BlendOut, 1.0);
                    return;
                }
            }
            GameState::LevelCompleted => {
                self.end_level();

                // Win Screen
                let text = format!("Level Completed");
                let text_size =  60.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 - 100.0, text_size, WHITE);

                // Local Score
                let text = format!("Current Score: {}", self.local_score);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 + 30.0, text_size, WHITE);

                // Current Level
                let text = format!("Level {} / {}", self.selected_level, 10);
                let text_size =  40.0;
                let text_width = text.chars().count() as f32 * text_size;
                let centered_position = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2);
                draw_text(text.as_str(), centered_position, GAME_SIZE_Y * 0.5 + 0.0, text_size, WHITE);


                if is_key_pressed(KeyCode::Space)
                {
                    self.next_level();
                    return;
                }
            }
        }

        // Update Level Blending
        self.level_transition.update_blend(self.world.level_offset);

        // Update Camera Space  ->  Especially used for Resizing the Game Window
        self.draw_camera_to_screen();
    }
    pub fn end_level(&mut self)
    {
        self.world.level_completed = false;
        self.world.level_offset = 0.0;
        let level_position = vec2( GAME_SIZE_X * 0.5, GAME_SIZE_Y * 0.5);
        self.camera.target = level_position;
        set_camera(&self.camera);
    }

    pub fn init_camera(&mut self)
    {
        // Setup Camera
        let camera_rect = Rect::new(0.0,0.0, GAME_SIZE_X , GAME_SIZE_Y );
        let mut camera =Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(self.render_target);
        set_camera(&camera);

        //Draw & Clear Background
        clear_background(BLACK);
        draw_rectangle( 0.0, 0.0,  GAME_SIZE_X as f32 ,  GAME_SIZE_Y as f32 , BLACK);
    }

    pub fn draw_camera_to_screen(&mut self)
    {
        // Set Default Camera
        set_default_camera();
        // calculate game view size based on window size
        let game_diff_w = GAME_SIZE_X / GAME_SIZE_X as f32;
        let game_diff_h = GAME_SIZE_Y / GAME_SIZE_Y as f32;
        let aspect_diff = game_diff_w.min(game_diff_h);
        
        let scaled_game_size_w = screen_width() as f32 * aspect_diff;
        let scaled_game_size_h = screen_height() as f32 * aspect_diff;
        
        let width_padding = (screen_width() - scaled_game_size_w) * 0.5f32;
        let height_padding = (screen_height() - scaled_game_size_h) * 0.5f32;
        
        // Draw Game on Screen
        clear_background(BLACK);
        draw_texture_ex(
            self.render_target.texture,
            width_padding,
            height_padding,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(scaled_game_size_w, scaled_game_size_h)),
                flip_y: true,
                ..Default::default()
            },
        );

    }

    pub fn restart(&mut self)
    {
        // Reload everything
        self.world.reload();
        self.selected_level = 0;
        self.load_level();

        self.misslepool = MisslePool::new();
        self.misslepool.create_pool(512, &mut self.world);
        
        self.enemypool = EnemyPool::new();
        self.enemypool.create_pool(128, &mut self.world);

        self.player = Player::new(&mut self.world);
        self.player.init(&mut self.world);

        self.local_score = 0;
        self.gamestate = GameState::GameRunning;
    }
    pub async fn init() -> Self
    {
        // Create Render Target for Game
        let game_render_target = render_target(GAME_SIZE_X as u32, GAME_SIZE_Y as u32);
        game_render_target.texture.set_filter(FilterMode::Linear);
        request_new_screen_size(GAME_SIZE_X , GAME_SIZE_Y);
        next_frame().await;
        // Create & Set Camera
        let camera_rect = Rect::new(0.0,0.0, GAME_SIZE_X , GAME_SIZE_Y );
        let mut camera =Camera2D::from_display_rect(camera_rect);
        camera.render_target = Some(game_render_target);
        set_camera(&camera);


        // Create Game Systems
        let mut world = World::new().await;

        let mut loader = LevelLoader::new();
        loader.level_loader_init().await;


        let mut misslepool = MisslePool::new();
        misslepool.create_pool(512, &mut world);


        let mut enemypool = EnemyPool::new();
        enemypool.create_pool(128, &mut world);


        let mut player = Player::new(&mut world);
        player.init(&mut world);

        
        // Setup Game Data
        Self {
            late_tick: 0.0,
            fixed_tick: 0.0,

            local_score: 0,  
            last_score: 0,
            high_score: 0,

            gamestate: GameState::MainMenu,

            //viewspace: viewspace,
            camera: camera,
            render_target: game_render_target,

            world: world,

            available_levels: loader.levels.len(),
            level_loader: loader,
            level: None,
            selected_level:4,
            level_transition: BlackBlend::default(),

            misslepool: misslepool,
            enemypool: enemypool,

            player: player,
        }

    }
    pub fn update(&mut self)
    {
        // Update World Entites
        self.world.update_actives();
        
        // Update Camera
        let level_position = vec2( GAME_SIZE_X * 0.5 + self.world.level_offset , GAME_SIZE_Y * 0.5);
        self.camera.target = level_position;
        set_camera(&self.camera);
        
        // Update Missles
        self.misslepool.update(&mut self.world);
        
        // Update Enemies
        self.enemypool.update(&mut self.world);
        self.enemypool.enemy_shoot(&mut self.misslepool, &mut self.world);
        
        // Update Player
        self.player.update(&mut self.world);
        self.player.shoot(&mut self.misslepool, &mut self.world);
        self.level_update();

    }
    pub fn fixed_update(&mut self)
    {
        self.world.fixed_update();
    }
    pub fn late_update(&mut self)
    {
        // Update Level
        self.level_late_update();
        
        
        self.player.late_update(&mut self.world);
        self.misslepool.late_update(&mut self.world);
        self.enemypool.late_update(&mut self.world);
    }
    pub fn draw(&mut self)
    {
        // Draw Level
        self.world.level.as_mut().unwrap().draw();

        // Draw Particles
        self.world.particlesystem_pool.draw();

        // Draw Entities
        self.player.draw();
        self.misslepool.draw();
        self.enemypool.draw();
        
        // Draw Score UI
        let text = format!("Local Score: {}", self.local_score);
        let text_size =  50.0;
        let text_width = text.chars().count() as f32 * text_size;
        let centered_position_x = ( GAME_SIZE_X * 0.5) - ( text_width * 0.2) + self.world.level_offset;
        let x_padding = GAME_SIZE_X * 0.5 - 250.0; 
        draw_rectangle(x_padding + self.world.level_offset, 0.0, GAME_SIZE_X - (x_padding*2.0), 60.0, color_u8!(0,0,0,100));
        draw_text(text.as_str(),centered_position_x, 50.0, text_size, WHITE);

    }

    pub fn update_score(&mut self)
    {
        self.local_score = self.world.get_collected_scorepoints();
    }

    pub fn next_level(&mut self) {
        self.world.reload_for_next_level();
        if self.selected_level < self.available_levels -1
        {
            self.selected_level += 1;
        }else {
            self.selected_level = 0;
        }
        println!("selected {} / available {}", self.selected_level, self.available_levels);
        self.load_level();

        self.misslepool = MisslePool::new();
        self.misslepool.create_pool(512, &mut self.world);
        
        self.enemypool = EnemyPool::new();
        self.enemypool.create_pool(128, &mut self.world);
        
        self.player = Player::new(&mut self.world);
        self.player.init(&mut self.world);

        self.gamestate = GameState::GameRunning;
        println!("Count: {}", self.world.entities.len());
    }

    
    pub fn load_level(&mut self)
    {
        //println!("Loader Data: {}", self.level_loader.levels[self.selected_level].enemy_spawner.len() );
        let mut level = Level::new(&mut self.world, self.level_loader.levels[self.selected_level].clone());
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