use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone, Copy)]
pub struct EnvConfig {
    pub feature_brick: bool,
    pub accelerate: bool,
    pub width: usize,
    pub height: usize,
}

pub fn load() -> Result<EnvConfig, String> {
    dotenv().ok();

    let feature_brick = match env::var("FEATURE_BRICK") {
        Ok(value) => value
            .parse()
            .map_err(|_| "FEATURE_BRICK has invalid value")?,
        Err(_) => true,
    };
    let accelerate = match env::var("ACCELERATE_MODE") {
        Ok(value) => value
            .parse()
            .map_err(|_| "ACCELERATE_MODE has invalid value")?,
        Err(_) => true,
    };
    let width = match env::var("WIDTH") {
        Ok(value) => value.parse().map_err(|_| "WIDTH has invalid value")?,
        Err(_) => 10,
    };
    let height = match env::var("HEIGHT") {
        Ok(value) => value.parse().map_err(|_| "HEIGHT has invalid value")?,
        Err(_) => 20,
    };

    Ok(EnvConfig {
        feature_brick,
        accelerate,
        width,
        height,
    })
}
