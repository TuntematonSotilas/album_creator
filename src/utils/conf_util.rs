use std::collections::HashMap;

pub fn parse_conf() -> HashMap<String, String> {
    let mut config = HashMap::new();
    let conf_str = include_str!("../../config.ini");
    let lines: Vec<&str> = conf_str.split("\r\n").collect();
    for line in lines {
        let line_vec: Vec<&str> = line.split("=").collect();
        if line_vec.first().is_some() && line_vec.last().is_some() {
            config.insert(
                line_vec.first().unwrap().to_string(), 
                line_vec.last().unwrap().to_string());
        }
    } 
    config
}