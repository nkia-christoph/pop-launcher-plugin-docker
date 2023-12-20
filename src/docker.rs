use docker_api::{
    Docker,
    Result
};
use pop_launcher_toolkit::plugin_trait::tracing;


#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    tracing::info!("Connecting to Docker Socket");
    Ok(Docker::unix("/var/run/docker.sock"))
}
