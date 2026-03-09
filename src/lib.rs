use bevy::ecs::component::Component;
use bevy::{DefaultPlugins, app::{App, Startup}};
use burn_cuda::{Cuda};
use crate::inspection::gpu::setup;

pub mod inspection;
#[derive(Component)]
pub struct Entity{
    pub math:u32
}

impl Entity {
    pub fn handle(){
        type Backend = Cuda;
        App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup::<Backend>).run();
    }
}