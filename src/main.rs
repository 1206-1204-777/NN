use bevy::{DefaultPlugins, app::{App, Startup}};
use nn::inspection::{gpu::setup};
use burn_cuda::{Cuda};
fn main() {
    type Backend = Cuda;
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup::<Backend>).run();
}
