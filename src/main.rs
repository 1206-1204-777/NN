    use std::{fs::OpenOptions, path::Path, thread};
    use std::io::Write;
    use nn::{Entity};
    use nn::mapping::create_map;
    use std::sync::mpsc::channel;
    use notify::{RecursiveMode, Watcher};
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
        Entity::handle();
    }