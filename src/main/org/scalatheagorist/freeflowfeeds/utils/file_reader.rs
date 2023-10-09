use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use log::error;
use serde::de::{DeserializeOwned, Error};

use crate::models::CustomSerdeErrors;

pub struct FileReader;

impl FileReader {
    pub fn from_yaml<T>(path: Box<&Path>) -> serde_yaml::Result<T>
        where T: DeserializeOwned {
        let file: String = match fs::read_to_string(&*path) {
            Ok(file) => file,
            Err(err) => {
                error!("could open from path {:?} with error {}", path, err);
                return Err(serde_yaml::Error::custom(CustomSerdeErrors::FileOpenError))
            }
        };

        match serde_yaml::from_str(&file) {
            Ok(t) => Ok(t),
            Err(err) => {
                error!("could read from file with error {}", err);
                return Err(serde_yaml::Error::custom(CustomSerdeErrors::YamlDeserializeError(err)))
            }
        }
    }

    pub fn from_json(path: Box<&PathBuf>) -> serde_json::Result<serde_json::Value> {
        let file: File = match File::open(&*path) {
            Ok(file) => file,
            Err(err) => {
                error!("could open from path {:?} with error {}", path, err);
                return Err(serde_json::Error::custom(CustomSerdeErrors::FileOpenError))
            }
        };
        let mut reader: BufReader<File> = BufReader::new(file);

        serde_json::from_reader(&mut reader)
    }
}
