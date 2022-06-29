use crate::Coder::{Writer::Writer, Reader::Reader};

#[derive(Debug)]
pub struct Physics
{
    pub ownerId: u32
}

impl Physics
{
    pub const ID: u32 = 1;

    pub fn New(owner: u32) -> Physics
    {
        Physics{ownerId: owner}
    }

    pub fn WriteBinary(&self, writer: &mut Writer)
    {
    }

    pub fn ReadBinary(&mut self, reader: &mut Reader)
    {
    }
}
