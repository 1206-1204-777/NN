use std::{fs::OpenOptions, path::Path, thread};
use std::io::Write;
use nn::{Entity};
use std::sync::mpsc::channel;
use notify::{RecursiveMode, Watcher};
use nn::maps::{mapping::*, path_checker::*, path_to_tensor::*};
fn main() {
    dotenvy::dotenv().ok();
    let (x, y) = channel();
    
    let mut watcher1 = notify::recommended_watcher(x).expect("読み込みに失敗");
    watcher1.watch(Path::new("logs.txt"), RecursiveMode::Recursive).expect("msg");
    
    thread::spawn(move || {let mut files = OpenOptions::new()
    .create(true)
    .append(true)
    .open("run_logs.txt")
    .expect("ファイルに書き込めませんでした。");
    for i in y {let _ = writeln!(files, "{:?}", i);println!("{:?}",i)}});

    let files = "datas/input.txt";
    let result = create_map(files);
    println!("{:?}", result);
    let paths = path_to_string("TRAIN_DATA_PATH", "paths.txt");

    let _ = write_path("outputs/2026_05.json outputs/2026_06.json".to_string());
    let _ = append_path("outputs/2026_07.json outputs/2026_08.json".to_string());
    let _ = read_path(paths.unwrap());

    let _ = walk_dir("TRAIN_DATA_PATH", "paths.txt");
    Entity::handle();
}