use std::fs::Metadata;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::vec::IntoIter;

use futures_util::StreamExt;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::fs::ReadDir;
use tokio_stream::Iter;

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
        if let Err(err) = fs::create_dir_all(Path::new(&config.clone().path)).await {
            error!("Could not create directory: {}", err);
            return;
        }

        values.for_each_concurrent(None, |value| {
            let suffix_clone = suffix.clone();
            async move {
                let hashed: u64 = hash_value::<R>(value.clone());
                let filename: String = format!("{}/data_{}{}", &config.clone().path, hashed.to_string(), &suffix_clone);
                let file_path: &Path = Path::new(&filename);

                if file_path.exists() {
                    debug!("file already exists: {}", filename);
                    return;
                }

                if let Some(json) = serde_json::to_string::<R>(&value).ok() {
                    match fs::write(file_path, json).await {
                        Ok(_) => debug!("write to filesystem successfully"),
                        Err(err) => error!("could not write to filesystem {}", err)
                    }
                }
            }
        }).await
    }

    pub async fn load_from_dir<W>(config: &FileStoreConfig) -> Iter<IntoIter<W>>
        where W: for<'de> serde::de::Deserialize<'de> + Send + 'static + serde::Serialize{
        let mut dir: ReadDir = match fs::read_dir(Path::new(&config.path)).await {
            Ok(dir) => dir,
            Err(err) => {
                error!("Error reading directory: {}", err);
                return tokio_stream::iter(vec![]);
            }
        };
        let mut files: Vec<(Metadata, W)> = Vec::new();

        while let Some(entry) = dir.next_entry().await.ok().flatten() {
            if let Ok(file_type) = entry.file_type().await {
                if file_type.is_file() {
                    let file_path: PathBuf = entry.path();
                    let metadata: Metadata = entry.path().metadata().unwrap();

                    match fs::read_to_string(file_path.clone()).await.ok() {
                        Some(data) => {
                            if let Some(w) = serde_json::from_str(&data).ok() {
                                files.push((metadata, w))
                            } else {
                                error!("could not deserialize json: {}", data)
                            }
                        },
                        None => error!("could not read file {:?}", file_path)
                    }

                }
            }
        }

        files.sort_by(|(entry1, _), (entry2, _)| {
            let m1: SystemTime = entry1.created().unwrap();
            let m2: SystemTime = entry2.created().unwrap();
            m1.cmp(&m2)
        });

        tokio_stream::iter(files.into_iter().map(|(_, data)| data).collect::<Vec<_>>())
    }
}