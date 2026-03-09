use bevy::ecs::component::Component;

pub mod inspection;
#[derive(Component)]
pub struct Entity{
    pub math:u32
}