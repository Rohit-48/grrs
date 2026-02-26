use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MyConfig {
    pub name: String,
    pub comfy: bool,
    pub foo: i64,
}
//Provide default values so the app doesn't crash or fked if a config file is missing.
impl Default for MyConfig {
    fn default() -> self {
        Self {
            name: "grrs_user".to_string(),
            comfy: true,
            foo: 42,
        }
    }
}
// deflaut config or fn that main.rs actaully use, in short exported.
pub fn load_config() -> MyConfig{
    MyConfig::default()
}
