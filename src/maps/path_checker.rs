use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use chrono;
use crate::{FilePathMemory, OriginalLogger};

pub fn read_path(paths: String) -> FilePathMemory{
    let mut new_paths = Vec::<String>::new();
    new_paths.push(paths.to_string());
    experiment_to_path(paths);
    FilePathMemory { paths: new_paths }
}

pub fn write_path(paths: String) -> Result<(), String>{
    let mut new_path = Vec::<String>::new();
    if !paths.is_empty(){
        let result = paths.replace(" ", "\n");
        new_path.push(result);
    }
    let results = FilePathMemory { paths:  new_path};
    if results.paths.is_empty(){
        return Err(String::from("パスの格納に失敗しました。"));
    }
    FilePathMemory::logger("write_path start: ".to_string() + &paths, chrono::Local::now().to_string());
    let mut write_file = OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open("datas/paths.txt")
    .unwrap();
    writeln!(write_file,"{}" ,results.paths[0]).expect("パスの保存に失敗しました。");
    FilePathMemory::logger("write_path end: ".to_string() + &paths, chrono::Local::now().to_string());
    Ok(())
}

pub fn append_path(paths: String) -> Result<(), String>{
    let mut new_path = Vec::<String>::new();

    if !paths.is_empty(){
        let result = paths.replace(" ", "\n");
        new_path.push(result);
    }
    let results = FilePathMemory { paths:  new_path};
    if results.paths.is_empty(){
        return Err(String::from("パスの格納に失敗しました。"));
    }
    FilePathMemory::logger("append_path start: ".to_string() + &paths, chrono::Local::now().to_string());
    let mut write_file = OpenOptions::new()
    .create(true)
    .append(true)
    .write(true)
    .open("datas/paths.txt")
    .unwrap();
    writeln!(write_file,"{}" ,results.paths[0]).expect("パスの保存に失敗しました。");
    FilePathMemory::logger("append_path end: ".to_string() + &paths, chrono::Local::now().to_string());
    Ok(())
}

fn experiment_to_path(path: String){
    let p = read_to_string(path).unwrap();
    println!("path: {}",p);
}

pub fn path_to_string(env: &str, file: &str) -> Result<String, String>{
    if env.is_empty(){
        return  Err(String::from("パスが取得できません。"))
    }
    if file.is_empty(){
        return  Err(String::from("指定されたファイルが取得できません。"))
    }
    let data = match env {
        "TRAIN_DATA_PATH" => std::env::var("TRAIN_DATA_PATH"),
        "OUTPUT_PATH" => std::env::var("OUTPUT_PATH"),
        _ => return Err(format!("使用できないパスです。{}", env))
    };
    println!("{:?}", data);
    let paths = format!("{}/{}", data.unwrap(), file);
    Ok(paths)
}