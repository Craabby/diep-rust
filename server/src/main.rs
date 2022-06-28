fn main()
{
    let mut simulation = shared::Simulation::Simulation::New();
    for _ in 0..shared::Types::MAX_ENTITIES
        {simulation.CreateEntity();}
    // println!("{:#?}", simulation);
}

