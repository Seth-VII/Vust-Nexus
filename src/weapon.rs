use super::*;

#[derive(Clone)]
pub struct Weapon
{
    entity: Entity,
    parent: Option<Entity>,
    sprite: Texture2D,
    direction: Vec2,
}
impl Weapon
{
    pub fn new(name: &str, tag: &str, world: &mut World) -> Self
    {
        Self {
            entity: Entity::new(name, tag, world),
            parent: None,
            sprite: Texture2D::empty(),
            direction: vec2(0.0, 0.0),
        }
    }
    pub fn set_parent(&mut self, parent: Option<Entity>)
    {
        self.parent = parent;
    }

    pub fn shoot(&mut self, misslepool: &mut MisslePool, world: &mut World)
    {
        misslepool.fire_missle( self.entity.clone(), self.direction, world);
    }
}
impl GameObject for Weapon
{
    fn init(&mut self) {
        self.entity.transform.set_position( vec2( GAME_SIZE_X as f32 * 0.5, GAME_SIZE_Y as f32 * 0.5 ));
        self.entity.transform.set_size(vec2(50.0,50.0));
        self.entity.entity_params.firespeed = 100.0;
    }
    fn update(&mut self, world: &mut World) {
        
        match self.entity.tag.as_str()
        {
            "Player Weapon" => {
                let dir = vec2(self.entity.transform.position.x - mouse_position().0, self.entity.transform.position.y - mouse_position().1);
                self.direction = dir.normalize() * -1.0;
                // Update from parent
                match &mut self.parent
                {
                    Some(parent) => {
                        self.entity.transform.set_position(parent.transform.position);
                    }
                    None => {}
                }
            }
            "Enemy Weapon" => {
                let player_option =  world.get_entity_by_tag("Player");
                match player_option
                {
                    Some(player) => {
                        let dir = (player.transform.position - self.entity.transform.position);
                        self.direction = dir.normalize();
                    }
                    None => {}
                }
                // Update from parent
                match &mut self.parent
                {
                    Some(parent) => {
                        self.entity.transform.set_position(parent.transform.position);
                    }
                    None => {}
                }
            }
            _ => {}
        }
        let rotation = f32::atan2(self.direction.x, self.direction.y) * -1.0;
        self.entity.transform.rotation = f32::to_radians(rotation.to_degrees() - 270.0);
    }
    fn late_update(&mut self, world: &mut World) {
        
    }
    fn draw(&mut self) {
        
        draw_rectangle(self.entity.transform.rect.x, self.entity.transform.rect.y, self.entity.transform.rect.w, self.entity.transform.rect.h, PURPLE);
    }
}