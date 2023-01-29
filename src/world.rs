use super::*;

pub struct World
{
    pub entities: Vec<Entity>,
    active_entities: Vec<Entity>,
}
impl World
{
    pub fn new() -> Self
    {
        Self {entities: Vec::new(),active_entities: Vec::new()}
    }

    pub fn update_actives(&mut self)
    {
        self.active_entities = self.entities.clone();
        self.active_entities.retain(|e| e.is_active == true);
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