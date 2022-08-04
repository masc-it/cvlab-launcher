use std::{fs::File, io::{BufReader, BufWriter}, error::Error};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Config {

    
    pub collections: Vec<String>,
    pub od_model: String


}


pub fn load_config() -> Result<Config, Box<dyn Error>>{
    let file = File::open("config.json");

    let file = match file {
        Ok(file) => file,
        Err(_) => {println!("config.json does NOT exist."); std::process::exit(0)}
    };

    let reader = BufReader::new(file);

    let model_config : std::result::Result<Config, serde_json::Error> = serde_json::from_reader(reader);

    let model_config = match model_config {
        Ok(model_config) => model_config,
        Err(_) => {println!("Malformed JSON. Check out the doc!"); std::process::exit(0)}
    };

    if !std::path::Path::new(&model_config.od_model).exists() {
        println!("ONNX model in {model_path} does NOT exist.", model_path=model_config.od_model); 
        std::process::exit(0)
    }

    println!("{model_path}", model_path=model_config.od_model);

    Ok(model_config)
}

pub fn save_config(config: &Config) -> Result<bool, Box<dyn Error>> {

    let file = File::create("config.json");

    let file = match file {
        Ok(file) => file,
        Err(_) => {println!("config.json does NOT exist."); std::process::exit(0)}
    };

    let writer = BufWriter::new(file);

    serde_json::to_writer(writer, config).unwrap();

    Ok(true)
}