//! Utility functions for managing Docker containers
//! If the docker container for SurrealDB and minio isn't running the
//! `ensure_containers_running` funtion will check and execute `docker compose up -d`
//! to start the containers.

use std::error::Error;
use std::process::Command;

/// ensure_containers_running checks if the required Docker
/// containers are running and starts them if they are not.
///
/// # returns
/// Returns Ok(()) if the containers are running or were started successfully.
///
/// # Errors
/// Returns an error if the docker-compose.yml file is not found,
/// if the docker command fails to execute, or if the containers fail to start.
///
/// # Examples
/// ```
/// use server::utility::docker::ensure_containers_running;
/// tokio_test::block_on(async {
///     ensure_containers_running().await.unwrap();
/// });
/// ```
pub async fn ensure_containers_running() -> Result<(), Box<dyn Error>> {
    let docker_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("docker");

    if !docker_dir.join("docker-compose.yml").exists() {
        return Err("docker-compose.yml not found in docker directory".into());
    }

    let output = Command::new("docker")
        .args(["compose", "up", "-d"])
        .current_dir(&docker_dir)
        .output()
        .map_err(|_| "Failed to execute docker compose command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Failed to start Docker containers: {}", stderr.trim()).into());
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    Ok(())
}
