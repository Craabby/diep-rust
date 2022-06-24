use super::super::Simulation::Simulation;

pub struct Entity<'a>
{
    simulation: &'a Simulation<'a>
}

impl<'a> Entity<'a>
{
    pub fn New(simulation: &'a Simulation) -> Entity<'a>
    {
        return Entity{simulation};
    }
}
