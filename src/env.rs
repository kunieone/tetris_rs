use dotenv::dotenv;
use std::env;

#[derive(Debug, Clone, Copy)]
pub struct EnvConfig {
    pub feature_brick: bool,
    pub accelerate: bool,
    pub width: usize,
    pub height: usize,
    pub texture: [char; 4],
}

pub fn load() -> Result<EnvConfig, String> {
    dotenv().ok();

    let feature_brick = match env::var("FEATURE_BRICK") {
        Ok(value) => value
            .parse()
            .map_err(|_| "FEATURE_BRICK should be a bool")?,
        Err(_) => true,
    };
    let accelerate = match env::var("ACCELERATE_MODE") {
        Ok(value) => value
            .parse()
            .map_err(|_| "ACCELERATE_MODE should be a bool")?,
        Err(_) => true,
    };
    let width = match env::var("WIDTH") {
        Ok(value) => value.parse().map_err(|_| "WIDTH should be a number")?,
        Err(_) => 10,
    };
    let height = match env::var("HEIGHT") {
        Ok(value) => value.parse().map_err(|_| "HEIGHT should be a number")?,
        Err(_) => 20,
    };
    let full = match env::var("TEXTURE_FULL") {
        Ok(value) => value
            .parse()
            .map_err(|_| "TEXTURE_FULL should be a char")?,
        Err(_) => '#',
    };

    let wall = match env::var("TEXTURE_WALL") {
        Ok(value) => value
            .parse()
            .map_err(|_| "TEXTURE_WALL should be a char")?,
        Err(_) => 'H',
    };

    let empty = match env::var("TEXTURE_EMPTY") {
        Ok(value) => value
            .parse()
            .map_err(|_| "TEXTURE_EMPTY should be a char")?,
        Err(_) => ' ',
    };

    let shadow = match env::var("TEXTURE_SHADOW") {
        Ok(value) => value
            .parse()
            .map_err(|_| "TEXTURE_SHADOW should be a char")?,
        Err(_) => '.',
    };

    Ok(EnvConfig {
        feature_brick,
        accelerate,
        width,
        height,
        texture: [full, wall, empty, shadow],
    })
}
