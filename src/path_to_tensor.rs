use std::{env::var};
use std::path::Path;
use walkdir::WalkDir;


pub fn walk_dir(env: &str, file: &str) -> Result<Vec<String>, String>{
    let dir_path = var(env);
    println!("{}", &dir_path.clone().unwrap());
    let result_path = WalkDir::new(dir_path.unwrap());
    println!("{:?}", &result_path);
    let file_data:Vec<String> = result_path.into_iter().filter_map(|e| e.ok())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect();
    println!("{:?}", file_data);
    collect_file(file_data, file)
}

fn collect_file(map: Vec<String>, file: &str)-> Result<Vec<String>, String>{
    if map.is_empty(){
        return Err("ディレクトリが指定されていません".to_string())
    }
    if file.is_empty(){
        println!("指定されたディレクトリには{}が存在しません", file);
        return Err("パス指定エラー".to_string())
    }
    let mut result = Vec::<String>::new();
    if let Some(target) =  map.into_iter().find(|s|Path::new(s).file_name().unwrap() == file){
        result.push(target);
    }
    println!("{:?}", result);
    if result.is_empty(){
        return Err("ファイルの取得ができませんでした。".to_string())
    }
    Ok(result)
}