pub struct Simulation
{
    pub entities: std::vec::Vec<crate::EntityComponentSystem::Entity::Entity>
}

impl Simulation
{
    pub fn New() -> Simulation
    {
        return Simulation{entities: vec![]};
    }
}
