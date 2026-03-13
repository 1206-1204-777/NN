use std::fs::OpenOptions;
use std::io::Write;
use bevy::ecs::component::Component;
use bevy::{DefaultPlugins, app::{App, Startup}};
use burn_cuda::{Cuda};
use crate::inspection::gpu::setup;
use chrono::Local;

pub mod inspection;
#[derive(Component)]
pub struct Entity{
    pub math:u32
}

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