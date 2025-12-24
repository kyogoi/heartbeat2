use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub database_path: String,
}

impl Config {
    fn new() -> Self {
        dotenv()?;

        let mqtt_host = env::var("MQTT_HOST").expect("MQTT_HOST must be set");
        let mqtt_port = env::var("MQTT_PORT")
            .expect("MQTT_PORT must be set")
            .parse()
            .expect("MQTT_PORT must be a valid number");
        let database_path = env::var("DATABASE_PATH").expect("DATABASE_PATH must be set");

        Self {
            mqtt_host,
            mqtt_port,
            database_path,
        }
    }
}
