use std::ops::DerefMut;

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
            if id == 0
                {i += 1; continue;}
            if self.Exists(id)
                {i += 1; continue;}

            let entity = self.entities[id as usize].insert(Entity::New());
            entity.id = id;

            break;
        }

        self.startingId = (self.startingId + 1) % MAX_ENTITIES;

        id
    }

    fn Exists(&self, id: u32) -> bool
    {
        self.entities[id as usize].is_some()
    }

    fn FindEntitiesInView(&self, viewer: &Camera) -> Vec<u32>
    {
        let mut ids = vec![];

        // TODO: only send entities near
        // TODO: spatial hashing
        for id in 0..MAX_ENTITIES
        {
            if !self.Exists(id)
                {break;}
            ids.push(id);
        }

        ids
    }

    pub fn GetConstEntity(&self, id: u32) -> &Entity
    {
        self.entities[id as usize].as_ref().expect("tried to get entity that does not exist")
    }

    pub fn GetEntity(&mut self, id: u32) -> &mut Entity
    {
        self.entities[id as usize].as_mut().expect("tried to get entity that does not exist")
    }

    fn GetUpdateType(&self, viewer: &mut Camera, entity: &Entity) -> EntityUpdateType
    {
        if entity.camera.is_some() && entity.camera.as_ref().unwrap().ownerId != viewer.ownerId
        { return EntityUpdateType::Private; }
    
        let entitiesInView = self.FindEntitiesInView(viewer);
        if !entitiesInView.contains(&entity.id)
        {
            viewer.view.remove(viewer.view.iter().position(|x| x == &entity.id).expect("trying to remove entity from view that is not in the view"));
            return EntityUpdateType::Deleted;
        }

        let isCreation = viewer.view.contains(&entity.id);

        if isCreation
        {
            viewer.view.push(entity.id);
            return EntityUpdateType::Created;
        }
        else
            {return EntityUpdateType::Updated;}
    }

    // if the viewer is None, the entire simulation will be sent. if it is Some, then only the entities that the viewer can see will be sent
    pub fn WriteBinary(&mut self, writer: &mut Writer, mut viewer: Option<&mut Camera>)
    {
        let mut deletions = vec![];
        let mut updates = vec![];
        let mut creations = vec![];
        let entitiesInView = match viewer
        {
            Some(ref x) => self.FindEntitiesInView(x),
            None => {
                let mut entities = vec![];
                for id in 0..MAX_ENTITIES
                {
                    if self.Exists(id)
                        {entities.push(id);}
                }

                entities
            }
        };

        // TODO: spatial hashing
        for i in entitiesInView
        {
            if !self.Exists(i)
                {continue;}
            let entityType = match viewer
            {
                Some(ref mut viewer) => self.GetUpdateType(viewer, self.GetConstEntity(i)),
                None => EntityUpdateType::Created
            };
            let entity = self.GetEntity(i);

            match entityType
            {
                EntityUpdateType::Deleted => deletions.push(entity.id),
                EntityUpdateType::Created => creations.push(entity.id),
                EntityUpdateType::Updated => updates.push(entity.id),
                EntityUpdateType::Private => {}
            }
        }

        for id in deletions
            {writer.Vu(id);}
        writer.Vu(0);
        for id in updates
        {
            let entity = self.GetConstEntity(id);
            writer.Vu(id);
            entity.WriteBinaryUpdate(writer);
        }
        writer.Vu(0);
        for id in creations
        {
            let entity = self.GetConstEntity(id);
            writer.Vu(id);
            entity.WriteBinaryCreation(writer);
        }

        writer.Vu(0);
    }

    pub fn ReadBinary(&mut self, reader: &mut Reader)
    {
        loop
        {
            let id = reader.Vu();
            if id == 0
                {break;}
            self.DeleteEntity(id)
        }

        loop
        {
            let id = reader.Vu();
            if id == 0
                {break;}
            if self.Exists(id)
                {panic!("tried to update nonexistant entity");}
            self.GetEntity(id).ReadBinaryUpdate(reader);    
        }


        loop
        {
            let id = reader.Vu();
            if id == 0
                {break;}

            self.entities[id as usize] = Some(Entity::New());
            self.GetEntity(id).id = id;
            self.GetEntity(id).ReadBInaryCreation(reader);
        }
    }
}
