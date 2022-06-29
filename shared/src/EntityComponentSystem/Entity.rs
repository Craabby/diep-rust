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

    }

    pub fn WriteBinaryCreation(&self, writer: &mut Writer)
    {
        let mut componentFlags = 0;

        componentFlags |= (self.camera.as_ref().is_some() as u32) << Camera::ID;
        componentFlags |= (self.physics.as_ref().is_some() as u32) << Physics::ID;

        writer.Vu(componentFlags);

        self.WriteComponents(writer);
    }

    fn WriteComponents(&self, writer: &mut Writer)
    {
        if self.camera.is_some()
            {self.camera.as_ref().unwrap().WriteBinary(writer);}
        if self.physics.is_some()
            {self.physics.as_ref().unwrap().WriteBinary(writer);}
    }

    pub fn ReadBinary(&mut self, reader: &mut Reader)
    {

    }
}
