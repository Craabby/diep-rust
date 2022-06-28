use std::vec::Vec;

pub struct Reader
{
    data: Vec<u8>,
    at: usize
}

impl Reader
{
    pub fn U8(&mut self) -> u8
    {
        let value = self.data[self.at];
        self.at += 1;
        value
    }

    pub fn U16(&mut self) -> u16
    {
        let value = (self.data[self.at] as u16) | ((self.data[self.at + 1] as u16) << 8);
        self.at += 2;
        value
    }

    pub fn U32(&mut self) -> u32
    {
        let value = (self.data[self.at] as u32) | ((self.data[self.at + 1] as u32) << 8) | ((self.data[self.at + 2] as u32) << 16) | ((self.data[self.at + 3] as u32) << 24);
        self.at += 4;
        value
    }

    pub fn Vu(&mut self) -> u32
    {
        let mut out = 0;
        let mut i = 0;

        while (self.data[self.at] & 0x80) == 0x80
        {
            out |= ((self.U8() & 0x7f) as u32) << i;
            i += 7;
        }

        out
    }

    pub fn Vi(&mut self) -> i32
    {
        let out = self.Vu();
        ((0 - (out & 1)) ^ (out >> 1)) as i32
    }

    pub fn String(&mut self) -> String
    {
        let mut string = String::new();

        while self.data[self.at] != 0
        { string.push(self.U8() as char); }

        string
    }
}
