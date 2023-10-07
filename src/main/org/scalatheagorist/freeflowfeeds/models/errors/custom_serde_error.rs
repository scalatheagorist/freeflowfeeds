use std::error::Error;
use std::fmt;

use serde_yaml;

#[derive(Debug)]
pub enum CustomSerdeErrors {
    FileOpenError,
    YamlDeserializeError(serde_yaml::Error),
    JsonDeserializeError(serde_json::Error),
}

impl fmt::Display for CustomSerdeErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomSerdeErrors::FileOpenError =>
                write!(f, "Failed to open file"),
            CustomSerdeErrors::YamlDeserializeError(ref err) =>
                write!(f, "Failed to deserialize YAML: {}", err),
            CustomSerdeErrors::JsonDeserializeError(ref err) =>
                write!(f, "Failed to deserialize JSON: {}", err)
        }
    }
}

impl Error for CustomSerdeErrors {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            CustomSerdeErrors::FileOpenError => None,
            CustomSerdeErrors::YamlDeserializeError(ref err) => Some(err),
            CustomSerdeErrors::JsonDeserializeError(ref err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for CustomSerdeErrors {
    fn from(err: serde_yaml::Error) -> Self {
        CustomSerdeErrors::YamlDeserializeError(err)
    }
}

impl From<serde_json::Error> for CustomSerdeErrors {
    fn from(err: serde_json::Error) -> Self {
        CustomSerdeErrors::JsonDeserializeError(err)
    }
}
