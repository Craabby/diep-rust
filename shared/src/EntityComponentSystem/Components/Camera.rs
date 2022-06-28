pub struct Camera
{
    pub ownerId: u32
}

impl Camera
{
    fn New(owner: u32) -> Camera
    {
        Camera{ownerId: owner}
    }
}
