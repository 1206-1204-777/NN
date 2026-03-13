use std::{path::Path, thread};
use nn::Entity;
use std::sync::mpsc::channel;
use notify::{RecursiveMode, Watcher};
fn main() {
    let (x, y) = channel();
    thread::spawn(move || {let mut watcher = notify::recommended_watcher(x).expect("読み込みに失敗");
    let _ = watcher.watch(Path::new("logs.txt"), RecursiveMode::Recursive).expect("msg");});
    Entity::handle();
    for i in y {println!("{:?}", i);}
}
