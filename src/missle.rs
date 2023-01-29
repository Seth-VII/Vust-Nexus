use super::*;

pub struct MisslePool
{
    pool: Vec<Missle>,
    active_pool: Vec<Missle>
}
impl MisslePool
{
    pub fn new() -> Self
    {
        Self { pool: Vec::new(), active_pool: Vec::new() }
    }
    pub fn create_pool(&mut self, count: usize, world: &mut World)
    {
        for i in 0..count
        {
            let mut missle = Missle::new(i, world);
            missle.reset_missle();
            world.set_entity(&mut missle.entity);
            self.pool.push(missle);
        }
    }
    pub fn fire_missle(&mut self, from_weapon: Entity, dir: Vec2, world: &mut World)
    {
        let free_slot = self.get_free_slot();
        match free_slot
        {
            Some(slot) => {
                self.pool[slot].setup_missle(from_weapon, dir);
                self.pool[slot].fire();
                self.active_pool.push(self.pool[slot].clone());
                world.set_entity(&mut self.pool[slot].entity);
            }
            None => {
                println!("No Free Missle Slot Found!");
            }
        }
    }
    fn get_free_slot(&self) -> Option<usize>
    {
        for i in 0..self.pool.len()
        {
            if !self.pool[i].entity.is_active
            {
                return Some(i);
            }
        }
        return None
    }
   
   
}
impl GameObject for MisslePool
{
    fn init(&mut self) {
    }
    fn update(&mut self, world: &mut World) {
        self.active_pool = self.pool.clone();
        self.active_pool.retain(|m| m.entity.is_active == true); 
        for missle in self.active_pool.iter_mut()
        {
            //let pool_missle = &mut self.pool[self.active_pool[i].misslepool_id];
            //pool_missle.update(world);

            //self.active_pool[i] = pool_missle.clone();
            //world.set_entity(&mut pool_missle.entity);
            missle.update(world);
            self.pool[missle.misslepool_id] = missle.clone();
            world.set_entity(&mut missle.entity);
        }
        //println!("count : {}", self.active_pool.len());
    }
    fn late_update(&mut self, world: &mut World) {
        for missle in self.active_pool.iter_mut()
        {
            //let pool_missle = &mut self.pool[self.active_pool[i].misslepool_id];
            //pool_missle.late_update(world);

            //self.active_pool[i] = pool_missle.clone();
            //world.set_entity(&mut pool_missle.entity);
            missle.late_update(world);
            self.pool[missle.misslepool_id] = missle.clone();
            world.set_entity(&mut missle.entity);
        }
    }
    fn draw(&mut self) {
        for missle in self.pool.iter_mut()
        {
            missle.draw();
        }
    }
}

#[derive(Clone)]
pub struct Missle
{
    misslepool_id: usize, 
    entity: Entity,
    weapon: Option<Entity>,
    dir: Vec2
}
impl Missle
{
    pub fn new(pool_id: usize,world: &mut World) -> Self
    {
        Self {
            misslepool_id: pool_id,
            entity: Entity::new("Missle","Missle", world),
            weapon: None,
            dir: vec2(0.0, 0.0),
        }
    }

    pub fn setup_missle(&mut self,from_weapon: Entity, dir: Vec2)
    {
        self.entity.transform.set_size( vec2(30.0,30.0));
        self.entity.transform.set_scale( 1.0);
        self.entity.transform.set_position(from_weapon.transform.position);
        self.entity.entity_params = from_weapon.entity_params.clone();
        self.weapon = Some(from_weapon);
        self.dir = dir;
    }
    pub fn fire(&mut self)
    {
        self.entity.SetActive(true);
        
    }
    pub fn reset_missle(&mut self)
    {
        self.weapon = None;
        self.dir = vec2(0.0, 0.0);

        self.entity.transform = Transform::zero();
        self.entity.SetActive(false);
    }

}
impl GameObject for Missle
{
    fn init(&mut self) {
        
    }
    fn update(&mut self, world: &mut World) {
        if !self.entity.is_active
        {
            return;
        }
        let position = self.entity.transform.position + (self.dir * self.entity.entity_params.firespeed * get_frame_time());
        self.entity.transform.set_position(position);
        if resolve_windowborder(self.entity.transform.rect)
        {
            self.reset_missle();
        }

    }

    fn late_update(&mut self, world: &mut World) {
        //let mut filtered_world = world.entities.clone();
        //filtered_world.retain(|e| e.tag != "Missle");
        for entity in world.entities.iter_mut()
        {
            if !entity.tag.contains("Missle")
            {
                self.on_collision(entity);
            }
        }
    }

    fn draw(&mut self) {
        if !self.entity.is_active
        {
            return;
        }
        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, RED);
    }
}
impl Collision for Missle
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            //self.rect_color = GREEN;
            return;
        }
        match entity.tag.as_str()
        {
            "Enemy" => {
                self.reset_missle();
                entity.transform = Transform::zero();
            },
            _ => {}
        }
    }
}