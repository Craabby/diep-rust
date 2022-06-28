use super::Types::{EntityUpdateType, MAX_ENTITIES};
use super::EntityComponentSystem::Components::Camera::Camera;
use super::EntityComponentSystem::Entity::Entity;
use super::Coder::{Reader::Reader, Writer::Writer};

pub struct Simulation
{
    // something the size of this really belongs on the heap
    // array of entities where the index is the id of the entity
    pub entities: Box<[Option<Entity>; MAX_ENTITIES as usize]>,
    // the largest id that any entity in this simulation has
    pub maxId: u32
}

impl Simulation
{
    pub fn New() -> Simulation
    {
        // bruh... https://github.com/rust-lang/rust/issues/44796
        const NONE: Option<Entity> = None;
        Simulation{entities: Box::new([NONE; MAX_ENTITIES as usize]), maxId: 0}
    }

    pub fn CreateEntity(&self) -> Entity
    {
        let entity = Entity::New();
        
        // TODO: generate ids

        entity
    }

    fn GetNextId(&self) -> u32
    {
        let mut id = 0;

        loop
        {
            if self.entities[id as usize].is_none()
                { return id; }
            id += 1;
            if id > MAX_ENTITIES
            {
                // TODO: delete old entities and try again
                panic!("out of ids");
            }
        }
    }


    fn FindEntitiesInView(&self, viewer: &Camera) -> Vec<u32>
    {
        let ids: Vec<u32> = vec![];

        ids
    }

    fn GetUpdateType(&self, viewer: &Camera, entity: &Entity) -> EntityUpdateType
    {
        // dont know what to do with the entity = dont send it
        let updateType: EntityUpdateType = EntityUpdateType::Private;

        if entity.camera.is_some() && entity.camera.as_ref().unwrap().ownerId != viewer.ownerId
        { return EntityUpdateType::Private; }
    
        let entitiesInView: Vec<u32> = self.FindEntitiesInView(viewer);

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
