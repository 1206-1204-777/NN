use std::fs::OpenOptions;
use std::io::Write;
use bevy::ecs::component::Component;
use bevy::utils::HashMap;
use bevy::{DefaultPlugins, app::{App, Startup}};
use burn_cuda::{Cuda};
use crate::inspection::gpu::setup;
use chrono::Local;
use serde::Serialize;

pub mod path_checker;
pub mod mapping;
pub mod inspection;
pub mod path_to_tensor;

#[derive(Serialize)]
#[derive(Debug)]
pub struct OriginalTensor{
    pub map: HashMap<String, u32>,
    pub data: Vec<u32>,
    pub frequency: Vec<u32>
}

#[derive(Component)]
pub struct Entity{
    pub math:u32
}

#[derive(Debug)]
pub struct FilePathMemory{
    pub paths: Vec<String>
}

impl OriginalLogger for FilePathMemory {}

impl Entity {
    pub fn handle(){
        type Backend = Cuda;
        let now = Local::now();
        Self::logger("Bevy_boot staar_time".to_string(), now.to_string());
        App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup::<Backend>).run();
        let now = Local::now();
        Self::logger("Bevy_boot end_time".to_string(), now.to_string());
       
    }
}

pub trait OriginalLogger {
    fn logger(log_data: String, now: String){
        if log_data.is_empty(){
            eprint!("データがありません。");
            return;
        }
        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.txt").expect("ファイル接続に問題あり。");
    let _ = writeln!(file, "{}:{}", log_data, now);
    }
}

impl OriginalLogger for Entity {}