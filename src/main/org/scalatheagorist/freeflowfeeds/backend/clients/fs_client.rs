use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::vec::IntoIter;

use futures_util::{Stream, StreamExt};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::{DirEntry, ReadDir};
use tokio_stream::Iter;
use tokio_stream::wrappers::ReadDirStream;

use crate::utils::hash_value::hash_value;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileStoreConfig {
    pub path: String,
    pub(crate) suffix: String
}

pub struct FileStoreClient;

impl FileStoreClient {
    pub async fn save_in_dir<R>(config: &FileStoreConfig, values: Iter<IntoIter<R>>, suffix: String)
        where R: for<'de> serde::de::Deserialize<'de> + Send + 'static + serde::Serialize + Hash + Clone {
        if let Err(err) = tokio::fs::create_dir_all(Path::new(&config.clone().path)).await {
            error!("Could not create directory: {}", err);
            return;
        }

        values.for_each_concurrent(None, |value| {
            let suffix_clone = suffix.clone();
            async move {
                let hashed: u64 = hash_value::<R>(value.clone());
                let filename: String = format!("{}/data_{}{}", &config.clone().path, hashed.to_owned(), &suffix_clone);
                let file_path: &Path = Path::new(&filename);

                if file_path.exists() {
                    debug!("file already exists: {}", filename);
                    return;
                }

                if let Some(json) = serde_json::to_string::<R>(&value).ok() {
                    match tokio::fs::write(file_path, json).await {
                        Ok(_) => debug!("write to filesystem successfully"),
                        Err(err) => error!("could not write to filesystem {}", err)
                    }
                }
            }
        }).await
    }

    pub async fn load_from_dir<W>(config: &FileStoreConfig) -> impl Stream<Item = W>
        where W: for<'de> serde::de::Deserialize<'de> + Send + 'static + serde::Serialize {
        let dir: ReadDir =
            tokio::fs::read_dir(Path::new(&config.path))
                .await
                .expect("could not read from dir");

        ReadDirStream::new(dir)
            .filter_map(|entry_result| async move {
                match entry_result {
                    Ok(entry) => Some(entry),
                    Err(err) => {
                        error!("Error reading directory entry: {}", err);
                        None
                    }
                }
            })
            .filter_map(|entry: DirEntry| async move {
                match entry.file_type().await {
                    Ok(file_type) if file_type.is_file() => Some(entry.path()),
                    _ => None,
                }
            })
            .filter_map(|file_path: PathBuf| async move {
                match fs::read_to_string(&file_path).await {
                    Ok(data) => match serde_json::from_str::<W>(&data) {
                        Ok(w) => Some(w),
                        Err(err) => {
                            error!("Could not deserialize JSON from file {:?}: {}", file_path, err);
                            None
                        }
                    },
                    Err(err) => {
                        error!("Could not read file {:?}: {}", file_path, err);
                        None
                    }
                }
            })
    }
}
