use macroquad::prelude::*;

pub const GAME_SIZE_X: f32 = 1920.0;
pub const GAME_SIZE_Y: f32 = 1080.0;
pub const LATE_UPDATE_TICK: f32 = 0.065;
pub const FIXED_UPDATE_TICK: f32 = 0.01;
pub const SHOW_COLLISION: bool = false;
pub const COLLISION_COLOR: Color = WHITE;


pub const SELECTED_LEVEL: usize = 0;

pub const LEVEL_SPEED: f32 = 300.0;

// Engine
//------------------
mod world;
pub use world::*;

mod assetloader;
pub use assetloader::*;

mod audio;
pub use audio::*;

mod renderer;
pub use renderer::*;

mod gameobject;
pub use gameobject::*;

mod collision;
pub use collision::*;

mod entity;
pub use entity::*;

mod animationcontroller;
pub use animationcontroller::*;

mod particle_sys;
pub use particle_sys::*;
mod particle_settings;
pub use particle_settings::*;

mod level;
pub use level::*;
mod levelloader;
pub use levelloader::*;
mod level_blending;
pub use level_blending::*;
// Game
//------------------
mod game;
pub use game::*;

mod gamestate;
pub use gamestate::*;

mod entityparam_settings;
pub use entityparam_settings::*;

mod player;
pub use player::*;

mod enemy;
pub use enemy::*;

mod enemytypes;
pub use enemytypes::*;

mod enemy_spawner;
pub use enemy_spawner::*;

mod enemypool;
pub use enemypool::*;

mod weapon;
pub use weapon::*;

mod missle;
pub use missle::*;

fn window_conf() -> Conf
{

    let app_conf : Conf = Conf 
    {
        window_title        : "VustNexus".to_string(),
        window_width        : GAME_SIZE_X as i32,
        window_height       : GAME_SIZE_Y as i32,
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

    let mut game = Game::init().await;
    
    loop {
        show_mouse(false);
        simulate_mouse_with_touch(true);

        game.Run();
        
       
        draw_rectangle_lines(0.0, 0.0, screen_width(), screen_height(), 2.0, WHITE);
        // Cursor
        draw_circle_lines(mouse_position().0, mouse_position().1, 7.0, 3.5, color_u8!(0,64,128,255));
        draw_circle_lines(mouse_position().0, mouse_position().1, 7.0, 1.3, WHITE);

        next_frame().await;
    }
}
