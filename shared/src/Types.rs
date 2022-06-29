pub enum EntityUpdateType
{
    Updated,
    Deleted,
    Created,
    // dont send the entity
    Private
}

pub const MAX_ENTITIES: u32 = 4;
