
use raylib::prelude::{Color, KeyboardKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    // general parameters
    pub window_width:  i32,
    pub window_height: i32,
    pub window_name:   String,
    pub world_file:    String,
    pub draw_outlines: bool,

    // colours
    pub wall_colour:     String,
    pub player_colour:   String,
    pub empty_colour:    String,
    pub outline_colour:  String
}
impl Config {

    pub fn load_config() -> Self {
        // load config from file
        // return a new config
        // load ./config.ron into a string
        let config_data = std::fs::read_to_string("./config.ron").expect("Unable to read file");
        // parse config data
        ron::from_str(&config_data).expect("Unable to parse config file")
    }
    pub fn wall_colour(&self) -> Color {
        // return the wall colour as a raylib colour
        Color::from_hex(self.wall_colour.as_str()).expect("Unable to parse wall colour from config")
    }
    pub fn player_colour(&self) -> Color {
        // return the player colour as a raylib colour
        Color::from_hex(self.player_colour.as_str()).expect("Unable to parse player colour from config")
    }
    pub fn empty_colour(&self) -> Color {
        // return the empty colour as a raylib colour
        Color::from_hex(self.empty_colour.as_str()).expect("Unable to parse empty colour from config")
    }
    pub fn outline_colour(&self) -> Color {
        // return the outline colour as a raylib colour
        Color::from_hex(self.outline_colour.as_str()).expect("Unable to parse outline colour from config")
    }
    pub fn config_checks(&self) {
        // check the width and height are multiples of 32
        if self.window_width % 32 != 0 {
            panic!("Window width must be a multiple of 32");
        }
        if self.window_height % 32 != 0 {
            panic!("Window height must be a multiple of 32");
        }
    }
}

