use std::{fs::{File, read_to_string}, io::BufWriter};
use bevy::utils::hashbrown::HashMap;
use chrono;
use serde_json::to_writer_pretty;
use crate::{OriginalTensor};

pub fn create_map(file: &str) -> Result<(), String>{
    let file_to_word = load_and_parse(file)?;
    write_json(&file_to_word)?;
    println!("{:?}", &file_to_word);
    Ok(())
}

fn load_and_parse(file: &str) -> Result<OriginalTensor, String>{

    let words = read_to_string(file).unwrap();
    let mut current_id = 0;
    let mut word_id = Vec::<u32>::new();
    let mut word_map = HashMap::<String, u32>::new();
    let mut result = OriginalTensor { map: HashMap::new(), data: Vec::new(),frequency: Vec::<u32>::new() };
    println!("{}",words);

    words.split_whitespace().for_each(|word|{
        let id = *word_map.entry(word.to_string()).or_insert_with(||{
            let id = current_id;
            current_id += 1;
            word_id.push(0);
            id
        });
        result.data.push(id);
        word_id[id as usize] += 1;
        
    });
    println!("{:?}", word_map);
    result.map.extend(word_map);
    result.frequency.extend(word_id);
    Ok(result)
}

fn write_json(tensor: &OriginalTensor) -> Result<(), String>{ 
    dotenvy::dotenv().ok();
    let output_times = chrono::Local::now().format("%Y_%m").to_string();
    let write_path = std::env::var("OUTPUT_PATH").map_err(|e| e.to_string()).unwrap();
    let file_name = format!("{}.json", output_times);
    let full_path = format!("{}/{}", write_path, file_name);
    match !tensor.data.is_empty() {
        true => {
            let path = File::create(&full_path).map_err(|e|e.to_string())?;
            let writer = BufWriter::new(path);
           to_writer_pretty(writer, &tensor).unwrap();
            Ok(())
        }
        false =>{

            let message = "マッピング作成に失敗しました".to_string();
            Err(message)
        }
    }
    
}
