use bevy::ecs::system::Commands;
use burn::prelude::Backend;
use burn::tensor::Tensor;
use crate::Entity;
pub fn execute<B: Backend>(device: &B::Device){
    let tensor = Tensor::<B, 2>::from_floats([[1.5, 3.5], [5.5, 4.5]], &device);
    let output = tensor.clone().matmul(tensor.transpose());

    println!("{}", output);
}

pub fn setup<B: Backend>(mut commands: Commands){
    commands.spawn(Entity{math: 10});
}