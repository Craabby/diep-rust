use crate::Simulation::Simulation;
use crate::Types::MAX_SIMULATIONS;

#[derive(Debug)]
pub struct SimulationFactory
{
    simulations: [Option<Simulation>; MAX_SIMULATIONS as usize]
}

impl SimulationFactory
{
    pub fn New() -> Self
    {
        const NONE: Option<Simulation> = None;
        Self{simulations: [NONE; MAX_SIMULATIONS as usize]}
    }

    pub fn CreateSimulation(&mut self) -> u32
    {
        let mut id = 0;
        for i in 0..MAX_SIMULATIONS
        {
            id = i;
            if self.simulations[id as usize].is_none()
                {break;}
        }
        self.simulations[id as usize] = Some(Simulation::New(id));
        id
    }

    pub fn GetConstSimulation(&self, id: u32) -> &Simulation
    {
        self.simulations[id as usize].as_ref().unwrap()
    }

    pub fn GetSimulation(&mut self, id: u32) -> &mut Simulation
    {
        self.simulations[id as usize].as_mut().unwrap()
    }

    pub fn DeleteSimulation(&mut self, id: u32)
    {
        self.simulations[id as usize] = None;
    }
}
