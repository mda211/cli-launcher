use crate::minecraft::launch::download::DownloadTask;
use crate::minecraft::metadata::library::Library;
use std::path::Path;

pub fn resolve_libraries(libraries: &[Library], base_path: &Path) -> Vec<DownloadTask> {
    libraries
        .iter()
        .map(|lib| {
            let full_path = base_path.join(&lib.downloads.artifact.path);
            let normalized = full_path
                .components()
                .map(|c| c.as_os_str().to_string_lossy())
                .collect::<Vec<_>>()
                .join("/");

            DownloadTask {
                dest: normalized,
                url: lib.downloads.artifact.url.clone(),
            }
        })
        .collect()
}
