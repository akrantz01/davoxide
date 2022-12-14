use crate::error::{Error, Result};
use async_graphql::{Enum, SimpleObject};
use std::{
    fs::FileType,
    path::{Path, PathBuf},
    time::SystemTime,
};
use time::OffsetDateTime;
use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

/// Get a list of all items specified directory
pub async fn list(base: &Path, path: PathBuf) -> Result<Vec<Entry>> {
    let full_path = base.join(&path);
    if !full_path.exists() {
        return Err(Error::NotFound);
    } else if !full_path.is_dir() {
        return Err(Error::NotADirectory);
    }

    // List the directory
    let read_dir = fs::read_dir(full_path).await?;
    let mut stream = ReadDirStream::new(read_dir);

    let mut entries = Vec::new();

    while let Some(entry) = stream.next().await {
        let entry = entry?;
        let meta = entry.metadata().await?;

        // Extract information about the entry
        let name = entry.file_name().to_string_lossy().into_owned();
        let created_at = meta.created().unwrap_or(SystemTime::UNIX_EPOCH).into();
        let last_modified = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH).into();

        // Add the entry
        entries.push(Entry {
            kind: meta.file_type().into(),
            path: path.join(&name).display().to_string(),
            name,
            created_at,
            last_modified,
            size: meta.len(),
        });
    }

    Ok(entries)
}

#[derive(Copy, Clone, Enum, Eq, PartialEq)]
#[graphql(name = "EntryType")]
pub enum Type {
    Directory,
    File,
    Unknown,
}

impl From<FileType> for Type {
    fn from(t: FileType) -> Self {
        if t.is_dir() {
            Type::Directory
        } else if t.is_file() {
            Type::File
        } else {
            Type::Unknown
        }
    }
}

#[derive(SimpleObject)]
pub struct Entry {
    #[graphql(name = "type")]
    kind: Type,
    name: String,
    path: String,
    created_at: OffsetDateTime,
    last_modified: OffsetDateTime,
    size: u64,
}
