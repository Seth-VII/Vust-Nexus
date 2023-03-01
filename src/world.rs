use super::*;

pub struct World
{
    pub assets: AssetLibrary,

    //pub level_loader: LevelLoader,
    //pub available_levels: usize,
    pub level: Option<Level>,
    pub level_offset: f32,
    pub level_completed: bool,
    //selected_level: usize,

    pub entities: Vec<Entity>,
    active_entities: Vec<Entity>,

    collected_scorepoints: i32,

    pub particlesystem_pool: ParticleSystemPool,
}
impl World
{
    pub async fn new() -> Self
    {
        let mut assets = AssetLibrary::new();
        assets.asset_loader_init().await;

        let mut loader = LevelLoader::new();
        loader.level_loader_init().await;

        Self {
            assets: assets,

            //available_levels: loader.levels.len(),
            //level_loader: loader,
            level: None,
            level_offset: 0.0,
            level_completed: false,
            //selected_level: 0,
            
            entities: Vec::new(),
            active_entities: Vec::new(), 
            collected_scorepoints: 0,
            particlesystem_pool: ParticleSystemPool::new(),
        }
    }
    pub fn get_active_level(&self) -> &Level { 
        &self.level.as_ref().unwrap()
    }

    pub fn reload(&mut self)
    {
        self.entities.clear();
        self.active_entities.clear();
        self.collected_scorepoints = 0;
        self.particlesystem_pool.clear();
        self.level = None;
        println!("Reload! {}", self.entities.len());
    }
    pub fn reload_for_next_level(&mut self)
    {
        self.entities.clear();
        self.active_entities.clear();
        //self.collected_scorepoints = 0;
        self.particlesystem_pool.clear();
        self.level = None;
        println!("Load next Level! {}", self.entities.len());
    }

    pub fn fixed_update(&mut self)
    {
        let mut particlesystem_pool = self.particlesystem_pool.clone();
        particlesystem_pool.update(self);
        self.particlesystem_pool = particlesystem_pool.clone();

    } 

    pub fn get_collected_scorepoints(&self) -> i32 { return self.collected_scorepoints; }
    pub fn add_scorepoints(&mut self, value: i32) { self.collected_scorepoints += value;}
    pub fn reset_scorepoints(&mut self) { self.collected_scorepoints = 0;}
    pub fn update_actives(&mut self)
    {
        self.active_entities = self.entities.clone();
        self.active_entities.retain(|e| e.is_active == true);

        if !self.get_active_level().has_reached_level_end(self.level_offset)
        {
            self.level_offset += LEVEL_SPEED * get_frame_time();
        }
    }
    pub fn get_actives(&mut self) -> &mut Vec<Entity>
    {
        &mut self.active_entities
    }

    pub fn add_entity(&mut self, new_entity: &mut Entity)
    {
        self.entities.push(new_entity.clone());
    }

    pub fn remove_entity(&mut self, index: usize)
    {
        self.entities.remove(index);
    }

    pub fn set_entity(&mut self, entity: &mut Entity)
    {
        for i in 0..self.entities.len()
        {
            if self.entities[i].id == entity.id
            {
                self.entities[i] = entity.clone();
                if !self.active_entities.contains(entity) && entity.is_active
                {
                    self.active_entities.push(entity.clone());
                }else if self.active_entities.contains(entity) && !entity.is_active
                {
                    self.active_entities.retain(|a| a != entity);
                }
                return;
            }
        }
    }



    pub fn get_entity_by_id(&mut self, id: usize) -> Option<&Entity>
    {
        for entity in self.entities.iter()
        {
            if entity.id == id
            {
                return Some(entity);
            }
        }
        return None;
    }
    pub fn get_entity_by_tag(&mut self, tag: &str) -> Option<&Entity>
    {
        for entity in self.entities.iter()
        {
            if entity.tag == tag
            {
                return Some(entity);
            }
        }
        return None;
    }


}