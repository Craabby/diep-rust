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

    pub fn CreateEntity(&mut self) -> &Entity
    {
        let entity = Entity::New();
        let mut id = 0;
        
        // TODO: make this not slow
        let maxId = self.maxId + 1;
        for _ in 0..=maxId
        {
            if id >= MAX_ENTITIES
                {panic!("out of entity ids");}
            if self.Exists(id)
                {id += 1; continue;}

            let entity = self.entities[id as usize].insert(entity);
            entity.id = id;

            if id > self.maxId
                {self.maxId = id;}

            break;
        }

        return self.entities[id as usize].as_ref().unwrap();
    }

    fn Exists(&self, id: u32) -> bool
    {
        self.entities[id as usize].is_some()
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
