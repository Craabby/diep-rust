use shared::{Coder::Writer::Writer, EntityComponentSystem::Components::{Camera::Camera, Physics::Physics}};

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
    
    simulation.GetEntity(3).camera = Some(Camera::New(3));
    simulation.GetEntity(3).physics = Some(Physics::New(3));

    println!("{:#?}", simulation);
    let mut writer = Writer::New();
    simulation.WriteBinary(&mut writer, None);
    println!("{:#?}", writer.Data());
}

