use crate::{de_required_field, ParserTasks};
use json::{Deserialize, JsonValue};
use logger::{debug, info, warning};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

#[cfg(not(debug_assertions))]
pub const CONFIG_PATH: &str = "/etc/lpm/conf";

#[cfg(debug_assertions)]
pub const CONFIG_PATH: &str = "conf";

const DEFAULT: &str = r#"{
    "modules": []
}
"#;

/// Used for parts that doesn't necessarily need to be in
/// sql database if they don't have relation with any other data.
/// e.g: repositories, package kinds, utils, etc.
pub struct LpmConfig {
    pub modules: Vec<Module>,
}

pub struct Module {
    pub name: String,
    pub dylib_path: String,
}

pub fn create_default_config_file() -> Result<(), io::Error> {
    if Path::new(CONFIG_PATH).exists() {
        warning!("'{CONFIG_PATH}' already exists, the creation process has been skipped.");
        return Ok(());
    };

    let mut file = File::create(CONFIG_PATH)?;
    info!("creating '{CONFIG_PATH}' file.");
    debug!("default values for '{CONFIG_PATH}' -> {DEFAULT}");
    file.write_all(DEFAULT.as_bytes())?;

    Ok(())
}

impl json::Deserialize for Module {
    type Error = String;

    fn from_json_object(json: &json::JsonValue) -> Result<Self, Self::Error> {
        Ok(Self {
            name: de_required_field!(json["name"].to_string(), "name"),
            dylib_path: de_required_field!(json["dylib_path"].to_string(), "dylib_path"),
        })
    }

    fn from_json_array(json: &json::JsonValue) -> Result<Vec<Self>, Self::Error> {
        let mut object_array = vec![];
        match json {
            JsonValue::Array(array) => {
                for item in array {
                    let object = Self::from_json_object(item)?;
                    object_array.push(object);
                }
            }
            _ => return Err("Wrong input, expected an array".to_string()),
        };

        Ok(object_array)
    }
}

impl json::Deserialize for LpmConfig {
    type Error = String;

    fn from_json_object(json: &json::JsonValue) -> Result<Self, Self::Error> {
        let modules = Module::from_json_array(&json["modules"])?;

        Ok(Self { modules })
    }

    fn from_json_array(json: &json::JsonValue) -> Result<Vec<Self>, Self::Error> {
        let mut object_array = vec![];
        match json {
            JsonValue::Array(array) => {
                for item in array {
                    let object = Self::from_json_object(item)?;
                    object_array.push(object);
                }
            }
            _ => return Err("Wrong input, expected an array".to_string()),
        };

        Ok(object_array)
    }
}

impl ParserTasks for LpmConfig {
    fn deserialize(path: &str) -> Self {
        let data_as_str = fs::read_to_string(path).unwrap_or_else(|_| {
            super::log_and_panic!("{} could not found.", path);
        });

        let json = json::Json::new(&data_as_str)
            .parse()
            .unwrap_or_else(|_error| {
                logger::debug!("Error: {}", _error);
                super::log_and_panic!(
                    "Package is either invalid or corrupted. Failed deserializing lpm config."
                );
            });

        Self::from_json_object(&json).unwrap_or_else(|error| {
            super::log_and_panic!("INTERNAL: {}", error);
        })
    }
}
