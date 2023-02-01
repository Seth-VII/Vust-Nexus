use macroquad::prelude::*;

pub const GAME_SIZE_X: i32 = 1280;
pub const GAME_SIZE_Y: i32 = 1280;
pub const LATE_UPDATE_TICK: f32 = 0.05;
pub const SHOW_COLLISION: bool = false;
pub const COLLISION_COLOR: Color = WHITE;
// Engine
//------------------
mod world;
pub use world::*;

mod assetloader;
pub use assetloader::*;

mod gameobject;
pub use gameobject::*;

mod collision;
pub use collision::*;

mod renderer;
pub use renderer::*;

mod entity;
pub use entity::*;

// Game
//------------------
mod game;
pub use game::*;

mod gamestate;
pub use gamestate::*;

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
    let mut game = Game::init().await;

    loop {
        clear_background(BLACK);
        game.Run();
        
        //println!("Active Entities: {}", world.get_actives().len());
        draw_text(format!("FPS: {}", get_fps()).as_str(), 30.0, 60.0, 30.0, WHITE);
        draw_rectangle_lines(0.0, 0.0, GAME_SIZE_X as f32, GAME_SIZE_Y as f32, 2.0, WHITE);
        //let world_entity = world.get_entity_by_tag("Player").unwrap();
        //println!("World Entity {}", world_entity.tranform.position);
        //println!("World {}", world.entities.len());
        next_frame().await;
    }
}
