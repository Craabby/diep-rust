use std::vec::Vec;

pub struct Writer
{
    data: Vec<u8>,
}

impl Writer
{
    pub fn New() -> Self
    {
        Writer{ data: vec![] }
    }

    pub fn Data(&self) -> &Vec<u8>
    {
        &self.data
    }

    pub fn U8(&mut self, value: u8) -> &mut Self
    {
        self.data.push(value);
        self
    }

    pub fn U16(&mut self, value: u16) -> &mut Self
    {
        self.U8(value as u8);
        self.U8((value >> 8) as u8)
    }

    pub fn U32(&mut self, value: u32) -> &mut Self
    {
        self.U8(value as u8);
        self.U8((value >> 8) as u8);
        self.U8((value >> 16) as u8);
        self.U8((value >> 24) as u8)
    }

    pub fn Float(&mut self, value: f32) -> &mut Self
    {
        self.U32(value.to_bits())
    }

    pub fn Vu(&mut self, mut value: u32) -> &mut Self
    {
        loop
        {
            let mut byte: u8 = value as u8;
            if value != 0
                { byte |= 128; }
            value >>= 7;
            self.U8(byte);
            if value == 0
                { break; }
        }

        self
    }

    pub fn Vi(&mut self, value: i32) -> &mut Self
    {
        let mut bytes = 0;
        bytes |= (value < 0) as u32;
        bytes |= (value << 1) as u32;
        
        self.Vu(bytes)
    }

    pub fn String(&mut self, value: &String) -> &mut Self
    {
        for char in value.chars()
        {
            self.U8(char as u8);
        }

        self.U8(0)
    }
}
