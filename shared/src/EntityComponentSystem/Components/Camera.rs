use crate::Coder::{Writer::Writer, Reader::Reader};

#[derive(Debug)]
pub struct Camera
{
    
    pub ownerId: u32,
    pub view: Vec<u32>
}

impl Camera
{
    pub const ID: u32 = 0;
    pub fn New(owner: u32) -> Camera
    {
        Camera{ownerId: owner, view: vec![]}
    }

    pub fn WriteBinary(&self, writer: &mut Writer)
    {
    }

    pub fn ReadBinary(&mut self, reader: &mut Reader)
    {
    }
}
