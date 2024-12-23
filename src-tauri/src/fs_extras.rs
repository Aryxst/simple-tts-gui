use serde::Serialize;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    path: String,
    is_file: bool,
    size: Option<u64>,
}

#[tauri::command]
pub fn list_files(path: String) -> Result<Vec<FileInfo>, String> {
    // Check if path exists
    if !Path::new(&path).exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let mut files = Vec::new();
    // Iterate over entries in the provided path
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
    {
        let metadata = entry.metadata().map_err(|err| err.to_string())?;

        files.push(FileInfo {
            path: entry.path().display().to_string(),
            is_file: metadata.is_file(),
            size: Some(metadata.len()),
        });
    }
    Ok(files)
}
