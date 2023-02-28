use macroquad::prelude::*;

pub const GAME_SIZE_X: i32 = 1600;
pub const GAME_SIZE_Y: i32 = 900;
pub const LATE_UPDATE_TICK: f32 = 0.05;
pub const FIXED_UPDATE_TICK: f32 = 0.01;
pub const SHOW_COLLISION: bool = false;
pub const COLLISION_COLOR: Color = WHITE;


pub const SELECTED_LEVEL: usize = 0;

pub const LEVEL_SPEED: f32 = 150.0;

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
    let mut game = Game::init().await;

    
    /*
    let mut sprite = game.world.assets.get_asset_by_name("weapon_sheet".to_string()).unwrap().get_texture_asset();
    sprite.setup_sheet(4, 4);
    sprite.animation.set_animation_duration(1.0);
    sprite.animation.set_animation_speed(1.5);
    */

    loop {
        clear_background(BLACK);
        
        game.Run();
        
        // Test Animation Sheet
        /*
        sprite.animation.play_anim_once();
        sprite.animation.update();
        let params = DrawTextureParams { dest_size: Some(sprite.get_sheet_tile_size() * 3.0), source: sprite.get_current_frame(), ..Default::default() };
        draw_texture_ex(sprite.texture_data, GAME_SIZE_X as f32 * 0.5 - 200.0, GAME_SIZE_Y as f32 * 0.5, WHITE, params);
        */
        
        

        //println!("Active Entities: {}", world.get_actives().len());
        draw_text(format!("FPS: {}", get_fps()).as_str(), 30.0 + game.world.level_offset, 60.0, 30.0, WHITE);
        draw_rectangle_lines(0.0, 0.0, GAME_SIZE_X as f32 + game.world.level_offset, GAME_SIZE_Y as f32, 2.0, WHITE);
        //let world_entity = world.get_entity_by_tag("Player").unwrap();
        //println!("World Entity {}", world_entity.tranform.position);
        //println!("World {}", game.world.entities.len());
        next_frame().await;
    }
}
