use reqwest::Client;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct DownloadTask {
    pub dest: String,
    pub url: String,
}

pub async fn download_files(tasks: Vec<DownloadTask>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    for task in tasks {
        let path = Path::new(&task.dest);

        if path.exists() {
            println!("Skipping {}, already exists", task.dest);
            continue;
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        println!("Downloading {} -> {}", task.url, task.dest);
        let response = client.get(&task.url).send().await?;
        let bytes = response.bytes().await?;

        let mut file = fs::File::create(path).await?;
        file.write_all(&bytes).await?;
    }

    Ok(())
}
