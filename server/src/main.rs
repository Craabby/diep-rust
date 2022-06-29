use shared::{Coder::{Writer::Writer, Reader::Reader}, EntityComponentSystem::Components::{Camera::Camera, Physics::Physics}};

fn main()
{
    let mut simulation = shared::Simulation::Simulation::New();
    let mut simulation2 = shared::Simulation::Simulation::New();
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

    let mut writer = Writer::New();
    let mut writer2 = Writer::New();
    simulation.WriteBinary(&mut writer, None);
    let mut reader = Reader::New(writer.Data().to_vec());
    simulation2.ReadBinary(&mut reader);
    simulation2.WriteBinary(&mut writer2, None);
    println!("{:#?}", writer.Data());
    println!("{:#?}", writer2.Data());
}

