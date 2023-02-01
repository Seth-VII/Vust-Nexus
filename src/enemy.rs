use super::*;


#[derive(Clone)]
pub enum EnemyType {
    Default, Heavy, Gunner,
}

#[derive(Clone)]
pub struct EnemyVariant 
{
    params: EntityParams,
    size: Vec2,
    color: Color,
    weapon: Option<Weapon>,
    points: i32,
}
impl EnemyVariant 
{
    pub fn get_variant (e_type: EnemyType, world: &mut World) -> Self 
    {
        match e_type
        {
            EnemyType::Default =>   {return EnemyVariant::default_variant(world);}
            EnemyType::Heavy =>     {return EnemyVariant::heavy_variant(world);}
            EnemyType::Gunner =>    {return EnemyVariant::gunner_variant(world);}
        }
    }
    fn default_variant(world: &mut World) -> Self
    {
        let mut params = EntityParams::default();
        params.health = 10.0;
        params.speed = 50.0;
        params.armor = 2.0;
        params.damage = 2.0;

        let size = vec2(60.0, 60.0);
        let color = GREEN;

        let mut weapon = None;

        let points = 20;

        Self { params: params, size: size, color: color, weapon: weapon, points: points }
    }
    fn heavy_variant(world: &mut World) -> Self
    {
        let mut params = EntityParams::default();
        params.health = 20.0;
        params.speed = 30.0;
        params.armor = 5.0;
        params.damage = 3.0;

        let size = vec2(80.0, 80.0);
        let color = RED;

        let mut weapon = None;

        let points = 50;

        Self { params: params, size: size, color: color, weapon: weapon, points: points }
    }
    fn gunner_variant(world: &mut World) -> Self
    {
        let mut params = EntityParams::default();
        params.health = 5.0;
        params.speed = 40.0;
        params.armor = 1.5;
        params.damage = 1.0;
        params.firerate = 1.0;
        params.firespeed = 200.0;

        let size = vec2(40.0, 40.0);
        let color = PURPLE;

        let mut weapon = Weapon::new("Gunner Weapon", "Enemy Weapon", world);
        weapon.set_stats(params.damage, params.firerate, params.firespeed);

        let points = 10;

        Self { params: params, size: size, color: color, weapon: Some(weapon), points: points }
    }
}


#[derive(Clone)]
pub struct Enemy
{
    pub enemy_id: usize,
    pub entity: Entity,
    sprite: Texture2D,

    pub variant: EnemyVariant,

}
impl Enemy
{
    pub fn new(index: usize,world: &mut World) -> Self
    {
        Self { 
            enemy_id: index,
            entity: Entity::new("Enemy", "Enemy", world), 
            sprite: Texture2D::empty(), 
            variant: EnemyVariant::get_variant(EnemyType::Default, world),
        }
    }
    

    fn reset(&mut self)
    {
        self.entity.entity_params = EntityParams::default();
        self.entity.transform.set_position(vec2(0.0,0.0));
        self.entity.is_active = false;
    }
    pub fn set_enemytype(&mut self,e_type: EnemyType, world: &mut World)
    {
        self.variant = EnemyVariant::get_variant(e_type, world);
        self.entity.entity_params = self.variant.params;
        self.entity.transform.set_size(self.variant.size);
        self.entity.set_rect_color(self.variant.color);
    }
    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.shoot(misslepool, world);
            }
            None => {}
        }
    }
}

impl GameObject for Enemy
{
    fn init(&mut self, world: &mut World) {
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.init(world);
            }
            None => {}
        }
    }
    fn update(&mut self, world: &mut World) {

        if self.entity.entity_params.health <= 0.0
        {
            self.reset();
            world.add_scorepoints( self.variant.points);
            world.set_entity(&mut self.entity);
            return;
        }

        self.entity.hit_cooldown();

        // MOVEMENT
        //println!("active {}", self.entity.is_active);
        let dir = (self.entity.transform.position - world.get_entity_by_tag("Player").as_ref().unwrap().transform.position).normalize();
        let position = self.entity.transform.position - (dir * self.entity.entity_params.speed * get_frame_time());
        self.entity.transform.set_position(position);
        

        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.set_parent(Some(self.entity.clone()));
                weapon.update(world);
            }
            None => {}
        }

       

    }
    fn late_update(&mut self, world: &mut World) {
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.late_update(world);
            }
            None => {}
        }
    }
    fn draw(&mut self, viewspace: &Viewspace) {
        if !self.entity.is_active || !self.entity.sprite_is_active
        {
            return;
        }
        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, self.entity.get_rect_color());
        draw_text(
            format!("E: {} | HP : {}",self.enemy_id , self.entity.entity_params.health).as_str(), 
            self.entity.transform.position.x, 
            self.entity.transform.position.y, 
            16.0, 
            BLACK);
        match &mut self.variant.weapon
        {
            Some(weapon) => {
                weapon.draw(viewspace);
            }
            None => {}
        }
    }
}
impl Collision for Enemy
{
    fn on_collision(&mut self, entity: &mut Entity) {
        if !resolve_intersection(self.entity.transform.rect,entity.transform.rect)
        {
            //self.rect_color = GREEN;
            return;
        }
        match entity.tag.as_str()
        {
            "Player" => {
                self.entity.entity_params.health = 0.0;
            }
            "Player Weapon Missle" => {
                //println!("HIT!!!");
                if self.entity.hit_feedback_timer <= 0.0
                {
                    self.entity.hit(&entity.entity_params);
                }

            }
            _ => {}
        }
    }
} 