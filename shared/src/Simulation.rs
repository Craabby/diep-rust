use super::Types::{EntityUpdateType, MAX_ENTITIES};
use super::EntityComponentSystem::Components::Camera::Camera;
use super::EntityComponentSystem::Entity::Entity;
use super::Coder::{Reader::Reader, Writer::Writer};

#[derive(Debug)]
pub struct Simulation
{
    // something the size of this really belongs on the heap
    // array of entities where the index is the id of the entity
    pub entities: Box<[Option<Entity>; MAX_ENTITIES as usize]>,
    // the id to start at when looping through all ids.
    // this eliminates looping through all of the first valid entities
    startingId: u32
}

impl Simulation
{
    pub fn New() -> Simulation
    {
        // bruh... https://github.com/rust-lang/rust/issues/44796
        const NONE: Option<Entity> = None;
        Simulation{entities: Box::new([NONE; MAX_ENTITIES as usize]), startingId: 0}
    }

    pub fn DeleteEntity(&mut self, id: u32)
    {
        if !self.Exists(id)
            {panic!("deleting nonexistant entity");}
        
        self.entities[id as usize] = None;
    }

    pub fn CreateEntity(&mut self) -> u32
    {
        let mut i = 0;
        let mut id = 0;
        for _ in 0..MAX_ENTITIES
        {
            id = (self.startingId + i) % MAX_ENTITIES;
            if self.Exists(id)
                {i += 1; continue;}

            let entity = Entity::New();
            let entity = self.entities[id as usize].insert(entity);
            entity.id = id;

            break;
        }

        self.startingId = (self.startingId + 1) % MAX_ENTITIES;

        return self.entities[id as usize].as_ref().unwrap().id;
    }

    fn Exists(&self, id: u32) -> bool
    {
        self.entities[id as usize].is_some()
    }

    fn FindEntitiesInView(&self, viewer: &Camera) -> Vec<u32>
    {
        let ids = vec![];



        ids
    }

    fn GetUpdateType(&self, viewer: &Camera, entity: &Entity) -> EntityUpdateType
    {
        // dont know what to do with the entity = dont send it
        let updateType = EntityUpdateType::Private;

        if entity.camera.is_some() && entity.camera.as_ref().unwrap().ownerId != viewer.ownerId
        { return EntityUpdateType::Private; }
    
        let entitiesInView = self.FindEntitiesInView(viewer);

        EntityUpdateType::Private
    }

    // if the viewer is None, the entire simulation will be sent. if it is Some, then only the entities that the viewer can see will be sent
    pub fn WriteBinary(&self, writer: &mut Writer, viewer: Option<&Camera>)
    {
        let deletions: Vec<&Entity>;
        let updates: Vec<&Entity>;
        let creations: Vec<&Entity>;
    }

    pub fn ReadBinary(&mut self, reader: &mut Reader)
    {
        let _ = reader;
    }
}
