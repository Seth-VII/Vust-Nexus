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
        let mut leveldata = LevelData::new(100.0);
        leveldata.load_walls(loaded.walls, world);
        leveldata.load_destructibles(loaded.destructibles, world);
        leveldata.load_enemyspawner(loaded.enemy_spawner, world);
        leveldata.load_turrets(loaded.turrets, world);
        Self { leveldata: leveldata }
    }

    pub fn get_visible_walls(&self) -> Vec<WallElement>
    {
        let elements = self.leveldata.walls.clone();
        let mut visibles = Vec::new();
        for element in elements.iter()
        {
            if !resolve_windowborder(element.entity.transform.rect)
            {
                visibles.push(element.clone());
            }
        }
        visibles
    }
    pub fn init(&mut self, world: &mut World)
    {
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.init(world);
        }
    }
    pub fn late_update(&mut self, world: &mut World, misslepool: &mut MisslePool)
    {
        for destructible_element in self.leveldata.destructibles.iter_mut()
        {
            destructible_element.late_update(world);
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
        for enemyspawner_element in self.leveldata.enemy_spawner.iter()
        {
            enemyspawner_element.draw();
        }
        for turret_element in self.leveldata.turrets.iter_mut()
        {
            turret_element.draw();
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
}
impl LevelData
{
    pub fn new(scale: f32) -> Self { 
        Self {level_scale: scale, walls: Vec::new(), enemy_spawner: Vec::new(), destructibles: Vec::new(), turrets: Vec::new() }
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
}
impl EnemySpawnerElement
{
    pub fn new(world: &mut World) -> Self { 
        let mut entity = Entity::new("EnemySpawner", "EnemySpawner", world);
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.armor = 5.0;
        entity.entity_params = params;
        entity.set_rect_color(RED);
        Self { entity: entity, sprite: TextureAsset::new() } 
    }
    pub fn place_spawner(&mut self, position: Vec2)
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