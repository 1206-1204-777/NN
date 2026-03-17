    use std::{fs::OpenOptions, path::Path, thread};
    use std::io::Write;
    use nn::Entity;
    use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
    use std::sync::mpsc::channel;
    use notify::{RecursiveMode, Watcher};
    fn main() {
        let (x, y) = channel();
        
        let mut watcher = notify::recommended_watcher(x).expect("読み込みに失敗");
        let _ = watcher.watch(Path::new("logs.txt"), RecursiveMode::Recursive).expect("msg");
        
        thread::spawn(move || {let mut files = OpenOptions::new()
        .create(true)
        .append(true)
        .open("run_logs.txt")
        .expect("ファイルに書き込めませんでした。");
        for i in y {let _ = writeln!(files, "{:?}", i);println!("{:?}",i)}});
        
        Entity::handle();
        
        thread::spawn(||{let mut v = vec![10, 20, 30];
        v.par_iter_mut().for_each(|x| *x *= 2); let sum: i32 = v.iter().sum();println!("{}", sum);});
        
    }
