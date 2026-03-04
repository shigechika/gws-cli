// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! File-system utilities.

use std::io;
use std::path::Path;

/// Write `data` to `path` atomically.
///
/// The data is first written to a temporary file alongside the target
/// (e.g. `credentials.enc.tmp`) and then renamed into place.  On POSIX
/// systems `rename(2)` is atomic with respect to crashes, so a reader of
/// `path` will always see either the old or the new content — never a
/// partially-written file.
///
/// # Errors
///
/// Returns an `io::Error` if the temporary file cannot be written or if the
/// rename fails.
pub fn atomic_write(path: &Path, data: &[u8]) -> io::Result<()> {
    // Derive a sibling tmp path, e.g. `/home/user/.config/gws/credentials.enc.tmp`
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?;
    let tmp_name = format!("{}.tmp", file_name.to_string_lossy());
    let tmp_path = path
        .parent()
        .map(|p| p.join(&tmp_name))
        .unwrap_or_else(|| std::path::PathBuf::from(&tmp_name));

    std::fs::write(&tmp_path, data)?;
    std::fs::rename(&tmp_path, path)?;
    Ok(())
}

/// Async variant of [`atomic_write`] for use with tokio.
pub async fn atomic_write_async(path: &Path, data: &[u8]) -> io::Result<()> {
    let file_name = path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "path has no file name"))?;
    let tmp_name = format!("{}.tmp", file_name.to_string_lossy());
    let tmp_path = path
        .parent()
        .map(|p| p.join(&tmp_name))
        .unwrap_or_else(|| std::path::PathBuf::from(&tmp_name));

    tokio::fs::write(&tmp_path, data).await?;
    tokio::fs::rename(&tmp_path, path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_atomic_write_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("credentials.enc");
        atomic_write(&path, b"hello").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"hello");
    }

    #[test]
    fn test_atomic_write_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("credentials.enc");
        fs::write(&path, b"old").unwrap();
        atomic_write(&path, b"new").unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"new");
    }

    #[test]
    fn test_atomic_write_leaves_no_tmp_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("credentials.enc");
        atomic_write(&path, b"data").unwrap();
        let tmp = dir.path().join("credentials.enc.tmp");
        assert!(!tmp.exists(), "tmp file should be cleaned up by rename");
    }

    #[tokio::test]
    async fn test_atomic_write_async_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("token_cache.json");
        atomic_write_async(&path, b"async hello").await.unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"async hello");
    }
}
