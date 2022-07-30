use crate::error::{Error, Result};
use std::path::{Component, PathBuf};

mod authentication;
mod permissions;

pub use authentication::middleware;
pub use permissions::check_permissions;

/// Sanitize a path, ensuring it does not escape the base directory
pub fn sanitize_path(raw: PathBuf) -> Result<PathBuf> {
    let mut sanitized = Vec::new();

    for component in raw.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::CurDir => continue,
            Component::ParentDir => {
                if sanitized.is_empty() {
                    return Err(Error::InvalidPermissions);
                } else {
                    sanitized.pop();
                }
            }
            Component::Normal(segment) => sanitized.push(segment),
        }
    }

    let path = PathBuf::from_iter(sanitized);
    Ok(path)
}
