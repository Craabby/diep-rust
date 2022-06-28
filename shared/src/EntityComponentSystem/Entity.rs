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
}
