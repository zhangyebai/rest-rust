use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig{
    pub host: String,
    pub port: u64,
    pub workers: u64,
}

pub fn find_server_config(active: &str) -> ServerConfig{
    match active{
        "dev" | "test" => ServerConfig{
            host: "0.0.0.0".to_string(),
            port: 10001,
            workers: 1
        },
        "master" => ServerConfig{
            host: "0.0.0.0".to_string(),
            port: 10001,
            workers: 16
        },
        _ => panic!("illegal profile active value")
    }
}