fn main()
{
    let mut simulation = shared::Simulation::Simulation::New();
    simulation.CreateEntity();
    simulation.CreateEntity();
    simulation.CreateEntity();
    simulation.DeleteEntity(1);
    simulation.DeleteEntity(2);
    simulation.CreateEntity();
    simulation.CreateEntity();
    simulation.CreateEntity();
    
    println!("{:#?}", simulation);
}

