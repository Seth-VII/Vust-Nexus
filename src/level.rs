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
        let mut leveldata = LevelData::new(50.0);
        leveldata.load_level_end(loaded.level_end, world);
        leveldata.load_walls(loaded.walls, world);
        leveldata.load_destructibles(loaded.destructibles, world);
        leveldata.load_enemyspawner(loaded.enemy_spawner, world);
        leveldata.load_turrets(loaded.turrets, world);
        Self { leveldata: leveldata }
    }
    pub fn has_reached_level_end(&self, progress: f32) -> bool
    {
        self.leveldata.end_of_level.as_ref().unwrap().reached_end(progress)
    }
    pub fn get_visible_walls(&self, level_offset: f32) -> Vec<Rect>
    {
        let mut visibles = Vec::new();

        // Add Default Walls 
        let elements = self.leveldata.walls.clone();
        for element in elements.iter()
        {
            if inside_windowborder(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x, element.entity.transform.get_fullsize().y)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }
        // Add Destructibles
        let elements = self.leveldata.destructibles.clone();
        for element in elements.iter()
        {
            if inside_windowborder(element.entity.transform.rect, level_offset + element.entity.transform.get_fullsize().x, element.entity.transform.get_fullsize().y)
            {
                visibles.push(element.entity.transform.rect.clone());
            }
        }
        // Add Turrets 
        let elements = self.leveldata.turrets.clone();
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
        for spawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            spawner_element.create_spawner(world);
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.init(world);
        }
    }
    pub fn update(&mut self, world: &mut World)
    {
      
        for spawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            spawner_element.update(world);
        }
       
    }
    pub fn late_update(&mut self, world: &mut World, misslepool: &mut MisslePool)
    {
        for destructible_element in self.leveldata.destructibles.iter_mut()
        {
            destructible_element.late_update(world);
        }
        for spawner_element in self.leveldata.enemy_spawner.iter_mut()
        {
            spawner_element.late_update(world);
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.shoot(misslepool, world);
            turret_element.late_update(world);
        }
    }
    pub fn draw(& mut self)
    {
        for wall_element in self.leveldata.walls.iter()
        {
            wall_element.draw();
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
    pub enemy_spawner: Vec<EnemySpawnerElement>,
    pub destructibles: Vec<DestructibleElement>,
    pub turrets: Vec<TurretElement>,

    pub end_of_level: Option<LevelEndElement>,
}
impl LevelData
{
    pub fn new(scale: f32) -> Self { 
        Self {level_scale: scale, walls: Vec::new(), enemy_spawner: Vec::new(), destructibles: Vec::new(), turrets: Vec::new(), end_of_level: None }
    }
    pub fn load_level_end(&mut self, level_end: Vec<Vec2>, world: &mut World)
    {
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
                println!("{}", pos_y);
            } 
        }
        end_element.entity.transform.set_size( vec2(1.0, y_size));
        end_element.entity.transform.set_scale( self.level_scale );
        end_element.entity.transform.set_position_not_centered(vec2( pos_x , pos_y) * self.level_scale);
        world.set_entity(&mut end_element.entity);
        self.end_of_level = Some(end_element);
    }
    pub fn load_walls(&mut self, walls: Vec<Vec2>, world: &mut World)
    {
        for i in 0..walls.len()
        {
            //println!("LevelData: {}", walls[i]);
            let mut wall = WallElement::new(world);
            wall.entity.transform.set_size( vec2(1.0, 1.0));
            wall.entity.transform.set_scale( self.level_scale );
            wall.entity.transform.set_position_not_centered(walls[i] * self.level_scale);
            world.set_entity(&mut wall.entity);
            self.walls.push(wall);
        }
    }
    pub fn load_destructibles(&mut self, destructibles: Vec<Vec2>, world: &mut World)
    {
        for i in 0..destructibles.len()
        {
            let mut destructible = DestructibleElement::new(world);
            destructible.entity.transform.set_size( vec2(1.0, 1.0));
            destructible.entity.transform.set_scale( self.level_scale );
            destructible.entity.transform.set_position_not_centered(destructibles[i] * self.level_scale);
            world.set_entity(&mut destructible.entity);
            self.destructibles.push(destructible);
        }
    }
    pub fn load_enemyspawner(&mut self, e_spawner: Vec<Vec2>, world: &mut World)
    {
        for i in 0..e_spawner.len()
        {
            let mut spawner = EnemySpawnerElement::new(world);


            for s in i..e_spawner.len()
            {
                if e_spawner[s] != e_spawner[i]
                {
                    if e_spawner[s].x > e_spawner[i].x && e_spawner[s].x < e_spawner[i].x + 2.0
                    {
                        
                    }else if e_spawner[s].y > e_spawner[i].y && e_spawner[s].y < e_spawner[i].y + 2.0
                    {

                    }else {
                        
                    }
                }
            }


        //end_element.entity.transform.set_size( vec2(1.0, y_size));
        //end_element.entity.transform.set_scale( self.level_scale );
        //end_element.entity.transform.set_position_not_centered(vec2( pos_x , pos_y) * self.level_scale);


            spawner.entity.transform.set_size( vec2(1.0, 1.0));
            spawner.entity.transform.set_scale( self.level_scale );
            spawner.entity.transform.set_position_not_centered(e_spawner[i] * self.level_scale);
            world.set_entity(&mut spawner.entity);
            self.enemy_spawner.push(spawner);
        }
    }
    pub fn load_turrets(&mut self, turrets: Vec<Vec2>, world: &mut World)
    {
        for i in 0..turrets.len()
        {
            let mut turret = TurretElement::new(world);
            turret.entity.transform.set_size( vec2(1.0, 1.0));
            turret.entity.transform.set_scale( self.level_scale );
            turret.entity.transform.set_position_not_centered(turrets[i] * self.level_scale);
            world.set_entity(&mut turret.entity);
            self.turrets.push(turret);
        }
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
    pub entity: Entity,
    pub sprite: TextureAsset,
}
impl WallElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("Wall", "Wall", world);
        entity.set_rect_color(WHITE);
        Self { entity: entity, sprite: TextureAsset::new() } 
    }
    pub fn place_wall(&mut self, position: Vec2)
    {
        self.entity.transform.set_position(position);
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
        }else {
            draw_rectangle(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                self.entity.get_rect_color()
            );
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
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;

        entity.entity_params = params;
        entity.set_rect_color(YELLOW);
        entity.hit_feedback_timer = 0.001;
        Self { entity: entity, sprite: TextureAsset::new() } 
    }
    pub fn place_destructible(&mut self, position: Vec2)
    {
        self.entity.transform.set_position(position);
    }
    pub fn late_update(&mut self, world: &mut World) {
        
        if !self.entity.is_active {return;}
        if self.entity.entity_params.health <= 0.0
        {
            self.entity.is_active = false;
            self.entity.entity_params.health = 1.0;
            world.particlesystem_pool.spawn_system_at_position( self.entity.transform.position, 64, Explosion_settings(YELLOW, RED, BLUE));
            self.entity.transform = Transform::zero();
            world.set_entity(&mut self.entity);
            return;
        }
        self.entity.hit_cooldown();
        for entity in world.entities.iter_mut()
        {
            self.on_collision( entity);
        }
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
        }else {
            draw_rectangle(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                self.entity.get_rect_color()
            );
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
            "Enemy Weapon Missle" => {
                //self.entity.hit(&entity.entity_params);
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

    pub spawner: Option<EnemySpawner>,

    color: Color,
}
impl EnemySpawnerElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("EnemySpawner", "EnemySpawner", world);
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        entity.entity_params = params;

        

        Self { 
            entity: entity, 
            sprite: TextureAsset::new(), 
            color: RED,
            spawner: None,
        } 
    }
    pub fn create_spawner(&mut self, world: &mut World)
    {
        self.spawner = Some( EnemySpawner::new(&self.entity.transform));
        self.spawner.as_mut().unwrap().create_pool(5, world);
        self.spawner.as_mut().unwrap().init(world);
    }
    pub fn place_spawner(&mut self, position: Vec2)
    {
        self.entity.transform.set_position(position + vec2( 200.0, 0.0));
    }
    pub fn update(&mut self, world: &mut World)
    {
        if !self.entity.is_active {return;}
        //println!("Spawner");
        match &mut self.spawner
        {
            Some(spawner) => 
            {
                spawner.update(world);
            }
            None => {}
        }
    }
    pub fn late_update(&mut self, world: &mut World) {
        // Only activate inside Windowborder
        //self.entity.transform.set_position(self.entity.transform.position + vec2(5.0, 0.0));

        if !inside_windowborder(self.entity.transform.rect, world.level_offset, 200.0) 
        {
            self.entity.is_active = false; 
            world.set_entity(&mut self.entity);
        } 
        else {
            self.entity.is_active = true; 
            world.set_entity(&mut self.entity);

            if inside_windowborder(self.entity.transform.rect, world.level_offset, 0.0) 
            {
                self.color = GREEN;
            }else {self.color = RED;}
        }
        
        if !self.entity.is_active {return;}
        match &mut self.spawner
        {
            Some(spawner) => 
            {
                spawner.late_update(world);
                self.color = GREEN;
            }
            None => {}
        }
    }
    pub fn draw(&mut self)
    {
        
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
        }
        if !self.entity.is_active {return;}
        if !SHOW_COLLISION {
            draw_rectangle(
                self.entity.transform.rect.x, 
                self.entity.transform.rect.y, 
                self.entity.transform.rect.w, 
                self.entity.transform.rect.h, 
                self.color
            );
        }

        match &mut self.spawner
        {
            Some(spawner) => 
            {
                spawner.draw();
            }
            None => {}
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
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        params.damage = 3.0;
        params.firerate = 10.0;
        params.firespeed = 300.0;

        entity.entity_params = params;
        entity.set_rect_color(BLUE);
        entity.hit_feedback_timer = 0.001;

        let mut weapon = Weapon::new("Turret", "Enemy Weapon", world);
        weapon.entity.entity_params = params;

        Self { entity: entity, sprite: TextureAsset::new(), weapon: weapon} 
    }
    pub fn place_turret(&mut self, position: Vec2)
    {
        self.entity.transform.set_position(position);
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
            world.particlesystem_pool.spawn_system_at_position( self.entity.transform.position, 64, Explosion_settings(YELLOW, RED, BLUE));
            self.entity.transform = Transform::zero();
            world.set_entity(&mut self.entity);
            return;
        }
        self.entity.hit_cooldown();

        
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
        }else {
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