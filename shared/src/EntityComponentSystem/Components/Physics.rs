#[derive(Debug)]
pub struct Physics
{
    pub ownerId: u32
}

impl Physics
{
    fn New(owner: u32) -> Physics
    {
        Physics{ownerId: owner}
    }
}
