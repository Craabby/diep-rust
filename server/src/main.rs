fn main()
{
    let mut simulation = shared::Simulation::Simulation::New();
    simulation.CreateEntity();
    println!("{:#?}", simulation);
}

