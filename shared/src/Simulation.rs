use super::EntityComponentSystem::Entity::Entity;
use std::rc::Rc;

pub struct Simulation<'a>
{
    pub entities: std::vec::Vec<&'a Entity<'a>>
}

impl<'a> Simulation<'a>
{
    pub fn New() -> Simulation<'a>
    {
        return Simulation{entities: vec![]};
    }

    pub fn CreateEntity(&self) -> Rc<Entity>
    {
        let entity: Rc<Entity> = Rc::new(Entity::New(&self));

        entity
    }
}
