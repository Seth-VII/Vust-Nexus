
use macroquad::audio::{play_sound, PlaySoundParams};
use macroquad::rand::gen_range;

use super::*;

#[derive(Clone)]
pub struct Level
{
    leveldata: LevelData,
}
impl Level
{
    pub fn new(world: &mut World, loaded: LoadedLevelData) -> Self
    {
        //println!("loaded data: {:?}", loaded.level_end);
        let mut leveldata = LevelData::new(45.0);
        leveldata.load_wall_fillings(&loaded, world);
        leveldata.load_level_end(loaded.level_end, world);
        leveldata.load_blocking_walls(loaded.blockingWalls, world);
        leveldata.load_trap_walls(loaded.trapWalls, world);
        leveldata.load_destructibles(loaded.destructibles, world);
        leveldata.load_enemyspawner(loaded.enemy_spawner, world);
        leveldata.load_turrets(loaded.turrets, world);
        println!("Level data state: Success!");
        Self { leveldata: leveldata }
    }
    pub fn has_reached_level_end(&self, progress: f32) -> bool
    {
        self.leveldata.end_of_level.as_ref().unwrap().reached_end(progress)
    }
    pub fn get_blocking_walls(&self, level_offset: f32) -> Vec<Rect>
    {
        let mut visibles = Vec::new();

        // Add Blocking Walls 
        let mut elements = self.leveldata.blockingwalls.clone();
        elements.retain(|e| e.entity.in_view == true);
        for element in elements.iter()
        {
            if inside_windowview(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }
        // Add Trap Walls 
        /*
        let elements = self.leveldata.trapwalls.clone();
        for element in elements.iter()
        {
            if inside_windowborder(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x, element.entity.transform.get_fullsize().y)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }
        */
        // Add Destructibles
        let mut elements = self.leveldata.destructibles.clone();
        elements.retain(|e| e.entity.in_view == true);
        for element in elements.iter()
        {
            if inside_windowborder(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x, element.entity.transform.get_fullsize().y)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }
        // Add Turrets 
        let mut elements = self.leveldata.turrets.clone();
        elements.retain(|e| e.entity.in_view == true);
        for element in elements.iter()
        {
            if inside_windowborder(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x, element.entity.transform.get_fullsize().y)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }

        visibles
    }

    pub fn init(&mut self, world: &mut World)
    {
        for trap_wall_element in self.leveldata.trapwalls.iter_mut()
        {
            trap_wall_element.init(world);
        }
        for enemyspawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            enemyspawner_element.init(world);
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.init(world);
        }
    }
    pub fn spawer_update(&mut self, enemypool: &mut EnemyPool, world: &mut World)
    {
        for enemyspawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            enemyspawner_element.update(enemypool,world);
        }

    }
    pub fn update(&mut self, world: &mut World)
    {
        for destructible_element in self.leveldata.destructibles.iter_mut()
        {
            destructible_element.update(world);
        }
    }

    pub fn late_update(&mut self, world: &mut World, misslepool: &mut MisslePool)
    {
        for wall_element in self.leveldata.walls.iter_mut()
        {
            wall_element.late_update(world);
        }
        for blocking_wall_element in self.leveldata.blockingwalls.iter_mut()
        {
            blocking_wall_element.late_update(world);
        }
        for trap_wall_element in self.leveldata.trapwalls.iter_mut()
        {
            trap_wall_element.late_update(world);
        }
        for destructible_element in self.leveldata.destructibles.iter_mut()
        {
            destructible_element.late_update(world);
        }
        for enemyspawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            enemyspawner_element.late_update(world);
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.shoot(misslepool, world);
            turret_element.late_update(world);
        }
    }
    pub fn draw(& mut self)
    {
        for trap_wall_element in self.leveldata.trapwalls.iter()
        {
            trap_wall_element.draw();
        }
        for wall_element in self.leveldata.walls.iter()
        {
            wall_element.draw();
        }
        for blocking_wall_element in self.leveldata.blockingwalls.iter()
        {
            blocking_wall_element.draw();
        }
       
        for destructible_element in self.leveldata.destructibles.iter()
        {
            destructible_element.draw();
        }
        for enemyspawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            enemyspawner_element.draw();
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.draw();
        }

        match &self.leveldata.end_of_level {
            Some (end_of_level)=> { end_of_level.draw();}
            None => {}
        }
    }
}

#[derive(Clone)]
pub struct LevelData
{
    pub level_scale: f32,
    pub walls: Vec<WallElement>,
    pub blockingwalls: Vec<BlockingWallElement>,
    pub trapwalls: Vec<TrapWallElement>,
    pub enemy_spawner: Vec<EnemySpawnerElement>,
    pub destructibles: Vec<DestructibleElement>,
    pub turrets: Vec<TurretElement>,

    pub end_of_level: Option<LevelEndElement>,
}
impl LevelData
{
    pub fn new(scale: f32) -> Self { 
        Self {
            level_scale: scale, 
            walls: Vec::new(), 
            blockingwalls: Vec::new(), 
            trapwalls: Vec::new(), 
            enemy_spawner: Vec::new(), 
            destructibles: Vec::new(), 
            turrets: Vec::new(), 
            end_of_level: None 
        }
    }
    pub fn load_level_end(&mut self, level_end: Vec<Vec2>, world: &mut World)
    {
        println!("Load End... ");
        let mut end_element = LevelEndElement::new(world);
        let mut y_size = 0.0;
        let mut pos_y = 0.0; 
        let mut pos_x = 0.0;
        for i in 0..level_end.len()
        {
            if level_end[i].y > y_size {
                pos_x = level_end[i].x;
                y_size = (level_end.last().unwrap().y - level_end[0].y) + 1.0; 
                pos_y = level_end[0].y;
            } 
        }
        end_element.entity.transform.set_size( vec2(1.0, y_size));
        end_element.entity.transform.set_scale( self.level_scale );
        end_element.entity.transform.set_position_not_centered(vec2( pos_x , pos_y) * self.level_scale);
        world.set_entity(&mut end_element.entity);
        self.end_of_level = Some(end_element);
    }
    pub fn load_wall_fillings(&mut self, loader_data: &LoadedLevelData, world: &mut World)
    {
        println!("Load Walls... ");
        //println!("Load Infected {}", (loader_data.walls.len()  + loader_data.infected_wall_filling.len()));
        for i in 0..(loader_data.walls.len()  + loader_data.infected_wall_filling.len())
        {
            //println!("LevelData: {}", walls[i]);
            let mut wall = WallElement::new(world);
            wall.transform.set_size( vec2(1.0, 1.0));
            wall.transform.set_scale( self.level_scale );
            
            if i < loader_data.walls.len()
            {
                wall.transform.set_position_not_centered(loader_data.walls[i] * self.level_scale);
                wall.set_sprite( vec2(0.0, 0.0));
            } else 
            {
                wall.transform.set_position_not_centered(loader_data.infected_wall_filling[i - loader_data.walls.len()] * self.level_scale);
                wall.set_sprite( vec2(32.0, 0.0));
            }

            wall.transform.rotation = self.rotate_tile();
            
            self.walls.push(wall);
            //println!("Wall: {} / {}", i, walls.len());
        }
    }

    pub fn load_blocking_walls(&mut self, blockingwalls: Vec<Vec2>, world: &mut World)
    {
        println!("Load Collider... ");
        for i in 0..blockingwalls.len()
        {
            //println!("LevelData: {}", walls[i]);
            let mut wall = BlockingWallElement::new(world);
            wall.entity.transform.set_size( vec2(1.0, 1.0));
            wall.entity.transform.set_scale( self.level_scale );
            wall.entity.transform.set_position_not_centered(blockingwalls[i] * self.level_scale);

            wall.entity.transform.rotation = self.rotate_tile();

            world.set_entity(&mut wall.entity);
            self.blockingwalls.push(wall);
            //println!("Collider: {} / {}", i, blockingwalls.len());
        }
    }

    pub fn load_trap_walls(&mut self, trapwalls: Vec<Vec2>, world: &mut World)
    {
        println!("Load Traps... ");
        for i in 0..trapwalls.len()
        {
            //println!("LevelData: {}", walls[i]);
            let mut trap = TrapWallElement::new(world);
            trap.entity.transform.set_size( vec2(1.0, 1.0));
            trap.entity.transform.set_scale( self.level_scale );
            trap.entity.transform.set_position_not_centered(trapwalls[i] * self.level_scale);

            trap.entity.transform.rotation = self.rotate_tile();

            world.set_entity(&mut trap.entity);
            self.trapwalls.push(trap);
            //println!("trap: {} / {}", i, trapwalls.len());
        }
    }

    pub fn load_destructibles(&mut self, destructibles: Vec<Vec2>, world: &mut World)
    {
        println!("Load Destructibles... ");
        for i in 0..destructibles.len()
        {
            let mut destructible = DestructibleElement::new(world);
            destructible.entity.transform.set_size( vec2(1.0, 1.0));
            destructible.entity.transform.set_scale( self.level_scale );
            destructible.entity.transform.set_position_not_centered(destructibles[i] * self.level_scale);

            destructible.entity.transform.rotation = self.rotate_tile();

            world.set_entity(&mut destructible.entity);
            self.destructibles.push(destructible);
            //println!("destructible: {} / {}", i, destructibles.len());
        }
    }
    pub fn load_enemyspawner(&mut self, e_spawner: Vec<(Vec2, usize, usize)>, world: &mut World)
    {
        println!("Load Spawners... ");
        for i in 0..e_spawner.len()
        {
            //println!("Spawner   {} / {}  ||| Count: {} | type: {}", i, e_spawner.len(), e_spawner[i].1, e_spawner[i].2);
            let mut spawner_element = EnemySpawnerElement::new(e_spawner[i].1, e_spawner[i].2 ,world);

            // Apply Spawner Transform
            spawner_element.entity.transform.set_size( vec2(1.0, 1.0));
            spawner_element.entity.transform.set_scale( self.level_scale );
            spawner_element.entity.transform.set_position_not_centered(e_spawner[i].0 * self.level_scale);

            spawner_element.spawner.set_transform(&spawner_element.entity.transform);
            spawner_element.entity.transform.rotation = self.rotate_tile();
             // Apply Spawner To World
            world.set_entity(&mut spawner_element.entity);
            self.enemy_spawner.push(spawner_element);
        }
    }
    pub fn load_turrets(&mut self, turrets: Vec<Vec2>, world: &mut World)
    {
        println!("Load turrets... ");
        for i in 0..turrets.len()
        {
            let mut turret = TurretElement::new(world);
            turret.entity.transform.set_size( vec2(1.0, 1.0));
            turret.entity.transform.set_scale( self.level_scale );
            turret.entity.transform.set_position_not_centered(turrets[i] * self.level_scale);
            world.set_entity(&mut turret.entity);
            self.turrets.push(turret);
            //println!("turrets: {} / {}", i, turrets.len());
        }
    }

    pub fn rotate_tile(&mut self) -> f32
    {
        let random_rotation = gen_range(0, 12); 
        let mut rotation = 0;
        match random_rotation
        {
            0 => { rotation = 0; }
            1 => {  rotation =  90; }
            2 => {  rotation =  180; }
            3 => {  rotation =  270; }

            4 => {  rotation =  0; }
            5 => {  rotation =  90; }
            6 => {  rotation =  180; }
            7 => {  rotation =  270; }

            8 => {  rotation =  0; }
            9 => {  rotation =  90; }
            10 => {  rotation =  180; }
            11 => {  rotation =  270; }

            _=> {  rotation =  0;}
        }

        return  f32::to_radians(rotation as f32);
    }
}

#[derive(Clone)]
pub struct LevelEndElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
}
impl LevelEndElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("End", "End", world);
        entity.set_rect_color(WHITE);
        entity.in_view = true;
        Self { entity: entity, sprite: TextureAsset::new() } 
    }
    pub fn place_endline(&mut self, position: Vec2)
    {
        self.entity.transform.set_position(position);
    }
    pub fn reached_end(&self, progress: f32) -> bool { inside_windowborder(self.entity.transform.rect, progress, 0.0)}
    pub fn draw(&self)
    {
        if !self.entity.is_active {}else
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
        }else {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
        }
    }
}

#[derive(Clone)]
pub struct WallElement
{
    pub transform: Transform,
    pub sprite: TextureAsset,
    tilesheet_offset: Vec2,
    in_view: bool,
}
impl WallElement
{
    pub fn new(world: &mut World) -> Self { 
        //let mut entity = Entity::new("Wall", "Wall", world);
        //entity.set_rect_color(WHITE);
        Self { 
            //entity: entity,
            transform: Transform::default(), 
            sprite: world.assets.get_asset_by_name("tile_texture_atlas".to_string()).as_mut().unwrap().get_texture_asset(),
            tilesheet_offset: vec2(0.0, 0.0),
            in_view: false,
        } 
    }

    pub fn late_update(&mut self, world: &mut World)
    {
        self.in_view = inside_windowview(self.transform.rect, world.level_offset);
    }

    pub fn set_sprite(&mut self, tilesheet_offset: Vec2)
    {
        self.tilesheet_offset = tilesheet_offset;
    }

    pub fn draw(&self)
    {
        
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.transform.rect.x, 
                self.transform.rect.y, 
                self.transform.rect.w, 
                self.transform.rect.h, 
                2.0,
                WHITE
            );
        }else if self.in_view{

            let tile_rect = Rect::new(self.tilesheet_offset.x, self.tilesheet_offset.y, 16.0, 16.0);
            let mut params = DrawTextureParams::default();
            params.source = Some(tile_rect);
            params.dest_size = Some(vec2( self.transform.rect.w + 20.0, self.transform.rect.h + 20.0));
            params.rotation = self.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, 
                self.transform.rect.x - 10.0, 
                self.transform.rect.y- 10.0, 
                WHITE, params);
        }
    }
}

#[derive(Clone)]
pub struct BlockingWallElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
}
impl BlockingWallElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("BlockingWall", "BlockingWall", world);
        entity.set_rect_color(GRAY);

        Self { 
            entity: entity, 
            sprite: world.assets.get_asset_by_name("tile_texture_atlas".to_string()).as_mut().unwrap().get_texture_asset(),
        } 
    }

    pub fn late_update(&mut self, world: &mut World)
    {
        //self.entity.hit_cooldown();
        self.entity.in_view = inside_windowview(self.entity.transform.rect, world.level_offset);
    }
    pub fn draw(&self)
    {
        if !self.entity.is_active {}else
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
        }else if self.entity.in_view{

            let tile_rect = Rect::new(17.0, 0.0,16.0, 16.0);
            let mut params = DrawTextureParams::default();
            params.source = Some(tile_rect);
            params.dest_size = Some(vec2( self.entity.transform.rect.w + 40.0, self.entity.transform.rect.h  + 40.0));
            params.rotation = self.entity.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, 
                self.entity.transform.rect.x - 20.0, 
                self.entity.transform.rect.y - 20.0, 
                WHITE, params);
        }
    }
}

#[derive(Clone)]
pub struct TrapWallElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
}
impl TrapWallElement
{
    pub fn new(world: &mut World) -> Self { 
        let entity = Entity::new("Trap", "TrapWall", world);

        Self { 
            entity: entity, 
            sprite: world.assets.get_asset_by_name("tile_texture_atlas".to_string()).as_mut().unwrap().get_texture_asset(), 
        } 
    }
    pub fn init(&mut self, world: &mut World)
    {
        self.entity.entity_params = EntitySettings::trap_settings(world);
        self.entity.set_rect_color(MAGENTA);
        world.set_entity(&mut self.entity);
    }
    pub fn late_update(&mut self, world: &mut World)
    {
        //self.entity.hit_cooldown();
        self.entity.in_view = inside_windowview(self.entity.transform.rect, world.level_offset);
    }
    pub fn draw(&self)
    {
        if !self.entity.is_active {}else
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
        }else if self.entity.in_view{

            let tile_rect = Rect::new(34.0, 0.0, 16.0, 16.0);
            let mut params = DrawTextureParams::default();
            params.source = Some(tile_rect);
            params.dest_size = Some(vec2( self.entity.transform.rect.w + 20.0, self.entity.transform.rect.h + 20.0));
            params.rotation = self.entity.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, 
                self.entity.transform.rect.x - 10.0, 
                self.entity.transform.rect.y- 10.0, 
                WHITE, params);
        }
    }
}

#[derive(Clone)]
pub struct DestructibleElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
}
impl DestructibleElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("Destructible Wall", "Destructible", world);
        entity.entity_params = EntitySettings::destructible_settings(world);

        entity.set_rect_color(WHITE);
        entity.hit_feedback_timer = 0.001;

        Self { 
            entity: entity, 
            sprite: world.assets.get_asset_by_name("tile_texture_atlas".to_string()).as_mut().unwrap().get_texture_asset(),
        } 
    }
    pub fn update(&mut self, world: &mut World)
    {
        if !self.entity.is_active {return;}
        self.entity.hit_cooldown();
        for entity in world.get_actives().iter_mut()
        {
            self.on_collision( entity);
        }
    }
    pub fn late_update(&mut self, world: &mut World) {
        
        if inside_windowview(self.entity.transform.rect, world.level_offset){
            self.entity.is_active = true;
            self.entity.in_view = true;
        }else {
            
        }

        if !self.entity.is_active {return;}
        if self.entity.entity_params.health <= 0.0
        {
            self.entity.is_active = false;
            self.entity.entity_params.health = 1.0;
            world.particlesystem_pool.spawn_system_at_position( 
                self.entity.transform.position, 
                64, 
                destruction_settings(LIGHTGRAY, WHITE, DARKGRAY));
            self.entity.transform = Transform::zero();
            world.set_entity(&mut self.entity);
            return;
        }

        self.entity.in_view = inside_windowview(self.entity.transform.rect, world.level_offset);
       
        
    }
    pub fn draw(&self)
    {
        if !self.entity.is_active {
            return;
        }else
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
        }else if self.entity.in_view{
            let tile_rect = Rect::new(51.0, 0.0, 16.0, 16.0);
            let mut params = DrawTextureParams::default();
            params.source = Some(tile_rect);
            params.dest_size = Some(vec2( self.entity.transform.rect.w + 40.0, self.entity.transform.rect.h + 40.0));
            params.rotation = self.entity.transform.rotation;
            draw_texture_ex(self.sprite.texture_data, 
                self.entity.transform.rect.x - 20.0, 
                self.entity.transform.rect.y- 20.0, 
                self.entity.get_rect_color(), params);
        }
    }
}
impl Collision for DestructibleElement
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            return;
        }
        //let mut params = PlaySoundParams::default();
        //params.volume = 0.15;
        match entity.tag.as_str()
        {
            "Player Weapon Missle" => {
                self.entity.hit(&entity.entity_params);
                //play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            _ => {}
        }
    }
} 

#[derive(Clone)]
pub struct EnemySpawnerElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
    color: Color,
    spawner: EnemySpawner,
}
impl EnemySpawnerElement
{
    pub fn new(count: usize, spawner_type: usize,world: &mut World) -> Self { 
        let mut entity = Entity::new("EnemySpawner", "EnemySpawner", world);
        entity.entity_params = EntitySettings::spawner_settings(world);

        entity.hit_feedback_timer = 0.001;
        let spawner = EnemySpawner::create_spawner(count, spawner_type, world);

        //println!("Spawner Element");
        Self { 
            entity: entity, 
            sprite: world.assets.get_asset_by_name("spawner_sheet".to_string()).as_mut().unwrap().get_texture_asset(),
            color: RED,
            spawner: spawner,
        } 
    }
    pub fn init(&mut self, world: &mut World)
    {
        self.sprite.setup_sheet(4, 2);
        self.sprite.animation_controller.apply_state_setup( StateMachineSetup::spawner_setup() );
        self.sprite.animation_controller.get_statemachine_mut().SetState(0);
        self.sprite.animation_controller.play_anim_once();
        if self.sprite.texture_data == Texture2D::empty()
        {
            self.entity.transform.set_size(vec2(60.0,60.0));
        }else 
        {
            self.entity.transform.set_size(self.sprite.get_sheet_tile_size());
            self.entity.transform.set_scale( 4.0);
        }
    }
    pub fn update(&mut self, enemypool: &mut EnemyPool,world: &mut World)
    {
        if inside_windowborder_extended_sides(self.entity.transform.rect, world.level_offset, 200.0, vec2(200.0, 100.0)) 
        {
            self.entity.in_view = true;
            self.entity.is_active = true;
        }else {
            self.entity.in_view = false;
            self.entity.is_active = false;
        }
        if !self.entity.is_active {return;}
        if inside_windowborder_extended_sides(self.entity.transform.rect, world.level_offset, 200.0, vec2(200.0, 600.0)) {
            if self.entity.entity_params.health > 0.0
            {
                self.spawner.update(enemypool, world);
            }
        }
    }
    pub fn late_update(&mut self, world: &mut World)
    {
        if !self.entity.is_active {return;}
        if self.entity.entity_params.health <= 0.0
        {
            self.entity.is_active = false;
            self.entity.entity_params.health = 1.0;

            let mut params = PlaySoundParams::default();
            params.volume = 0.5;
            play_sound(world.assets.get_asset_by_name("explosion_3".to_string()).unwrap().get_sound_data().sound.unwrap(), params );

            world.particlesystem_pool.spawn_system_at_position( self.entity.transform.position, 64, explosion_settings(MAGENTA, RED, color_u8!(255,0,255,0)));
            self.entity.transform = Transform::zero();
            world.set_entity(&mut self.entity);
            return;
        }
        self.entity.hit_cooldown();

        if inside_windowborder_extended_sides(self.entity.transform.rect, world.level_offset, 200.0, vec2(200.0, -100.0)) 
        {
            if !self.sprite.animation_controller.get_statemachine_mut().animation_states[0].is_playing() &&
            !self.sprite.animation_controller.get_statemachine_mut().animation_states[1].is_playing()
            {
                self.sprite.animation_controller.get_statemachine_mut().SetState(1);
                self.sprite.animation_controller.play_anim_once();
            }
        }
        if inside_windowborder_extended_sides(self.entity.transform.rect, world.level_offset, 200.0, vec2(200.0, -50.0)) 
        {
            self.sprite.animation_controller.update();
        }
        for entity in world.get_actives().iter_mut()
        {
            self.on_collision( entity);
        }
    }
    pub fn draw(&mut self)
    {
        if !self.entity.is_active {return;}
        if SHOW_COLLISION  
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.color
            );
        }else if self.entity.in_view{
            let frame = self.sprite.get_current_anim_controller_frame(); 
            let mut params = DrawTextureParams::default();
            params.dest_size = Some(self.entity.transform.get_fullsize());
            params.rotation = self.entity.transform.rotation;
            params.source = frame;


            draw_texture_ex(self.sprite.texture_data, 
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.get_rect_color(), params);
        }
    }
}

impl Collision for EnemySpawnerElement
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            return;
        }
        //let mut params = PlaySoundParams::default();
        //params.volume = 0.15;
        match entity.tag.as_str()
        {
            "Player Weapon Missle" => {
                self.entity.hit(&entity.entity_params);
                //play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            _ => {}
        }
    }
} 

#[derive(Clone)]
pub struct TurretElement
{
    pub entity: Entity,
    pub sprite: TextureAsset,
    pub weapon: Weapon,
}
impl TurretElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("Turret", "Turret", world);
        entity.entity_params = EntitySettings::turret_settings(world);

        entity.set_rect_color(BLUE);
        entity.hit_feedback_timer = 0.001;

        let mut weapon = Weapon::new("Turret", "Enemy Weapon", world);
        weapon.entity.entity_params = entity.entity_params;

        Self { entity: entity, sprite: TextureAsset::new(), weapon: weapon} 
    }

    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        //println!("WF Speed: {}, name: {}", weapon.entity.entity_params.firespeed, weapon.entity.name);
        if self.weapon.shoot( misslepool, world)
        {
            //let mut params = PlaySoundParams::default();
            //params.volume = 0.15;
            //play_sound( self.variant.sfx_shoot.sound.unwrap(), params)
        }
    }

    pub fn init(&mut self, world: &mut World)
    {
        self.weapon.init(world);
    }
    pub fn late_update(&mut self, world: &mut World) {
        
        if !self.entity.is_active {return;}
        if self.entity.entity_params.health <= 0.0
        {
            self.entity.is_active = false;
            self.entity.entity_params.health = 1.0;
            world.particlesystem_pool.spawn_system_at_position( self.entity.transform.position, 64, explosion_settings(YELLOW, RED, color_u8!(255,255,0,0)));
            self.entity.transform = Transform::zero();
            world.set_entity(&mut self.entity);
            return;
        }
        self.entity.hit_cooldown();

        
        self.entity.in_view = inside_windowview(self.entity.transform.rect, world.level_offset);

        for entity in world.entities.iter_mut()
        {
            self.on_collision( entity);
        }
        
        
        self.weapon.set_parent(Some(self.entity.clone()));
        self.weapon.update(world);
        // Update World
        world.set_entity(&mut self.entity);
    }
    pub fn draw(&mut self)
    {
        if !self.entity.is_active {}else
        if SHOW_COLLISION 
        {
            draw_rectangle_lines(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                2.0,
                self.entity.get_rect_color()
            );
            self.weapon.draw();
        }else if self.entity.in_view {
            draw_rectangle(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                self.entity.get_rect_color()
            );
            self.weapon.draw();
        }
    }
}

impl Collision for TurretElement
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            return;
        }
        //let mut params = PlaySoundParams::default();
        //params.volume = 0.15;
        match entity.tag.as_str()
        {
            "Player Weapon Missle" => {
                self.entity.hit(&entity.entity_params);
                //play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            "Enemy Weapon Missle" => {
                //self.entity.hit(&entity.entity_params);
                //play_sound(self.sfx_on_hit.sound.unwrap(), params);
            }
            _ => {}
        }
    }
} 