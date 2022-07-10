use crate::Coder::{Writer::Writer, Reader::Reader};

use super::Components::{Physics::Physics, Camera::Camera};

#[derive(Debug)]
pub struct Entity
{
    pub camera: Option<Camera>,
    pub physics: Option<Physics>,
    pub id: u32
}

impl Entity
{
    pub fn New() -> Entity
    {
        return Entity{ camera: None, physics: None, id: 0 };
    }

    pub fn WriteBinaryUpdate(&self, writer: &mut Writer)
    {
        self.WriteBinaryComponents(writer)
    }

    pub fn WriteBinaryCreation(&self, writer: &mut Writer)
    {
        let mut componentFlags = 0;

        componentFlags |= (self.camera.as_ref().is_some() as u32) << Camera::ID;
        componentFlags |= (self.physics.as_ref().is_some() as u32) << Physics::ID;

        writer.Vu(componentFlags);

        self.WriteBinaryComponents(writer);
    }

    fn WriteBinaryComponents(&self, writer: &mut Writer)
    {
        if self.camera.is_some()
            {self.camera.as_ref().unwrap().WriteBinary(writer);}
        if self.physics.is_some()
            {self.physics.as_ref().unwrap().WriteBinary(writer);}
    }

    fn ReadBinaryComponents(&mut self, reader: &mut Reader)
    {
        match self.camera.as_mut()
        {
            None => {}
            Some(x) => x.ReadBinary(reader)
        };
        match self.physics.as_mut()
        {
            None => {}
            Some(x) => x.ReadBinary(reader)
        };
    }

    pub fn ReadBinaryUpdate(&mut self, reader: &mut Reader)
    {
        self.ReadBinaryComponents(reader)
    }

    pub fn ReadBinaryCreation(&mut self, reader: &mut Reader)
    {
        let componentFlags = reader.Vu();
        if (componentFlags & (1 << Camera::ID)) != 0
            {self.camera = Some(Camera::New(self.id));}
        if (componentFlags & (1 << Physics::ID)) != 0
            {self.physics = Some(Physics::New(self.id));}
    }
}
