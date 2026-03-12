 use bevy::prelude::default;
use bevy::color::Srgba;
use bevy::core_pipeline::core_3d::Camera3d;
use bevy::math::primitives::{Sphere};
use bevy::math::vec3;
use bevy::pbr::{MeshMaterial3d, PointLight};
use bevy::prelude::Assets;
use bevy::ecs::system::{Commands, ResMut};
use bevy::prelude::{Mesh, StandardMaterial, Color};
use bevy::render::mesh::Mesh3d;
use bevy::transform::components::Transform;
use burn::prelude::Backend;
use burn::tensor::Tensor;
use crate::Entity;


pub fn execute<B: Backend>(device: &B::Device){
    let tensor = Tensor::<B, 2>::from_floats([[1.5, 3.5], [5.5, 4.5]], &device);
    let output = tensor.clone().matmul(tensor.transpose());

    println!("{}", output);
}

pub fn setup<B: Backend>(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){
    let x = 0.0;
    let y = 0.5;
    let z= 0.0;
    commands.spawn((
        Entity{math: 10}, Mesh3d(meshes.add(Sphere::new(1.8))), 
        MeshMaterial3d(materials.add(StandardMaterial::from_color(Srgba{red: 0.5, green: 1.0, blue: 1.0, alpha: 1.0}))),
        Transform::from_xyz(4.0, 8.0, 4.0)
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(x + 0.5, y + 0.5, z + 1.0).looking_at(vec3(5.0, 10.0, 4.0), vec3( 8.0, -2.0, 5.0))
    ));
    commands.spawn((
        PointLight{
            intensity: 50_000_00.0, 
            color:Color::from(Srgba{red: 0.5, green: 1.0, blue: 1.0, alpha: 1.0}), 
            range: 10.0, 
            radius: 5.0, 
            shadows_enabled: true, 
            shadow_depth_bias: default(), 
            shadow_map_near_z: default(), 
            shadow_normal_bias: 0.5},
        Transform::from_xyz(x + 1.0, 0.5, z + 0.5)
    ));
}