use std::{
    collections::HashMap,
    sync::Arc,
    sync::Mutex,
};
use docker_api::{
    opts::ContainerListOpts,
    Docker,
    Result,
};
use pop_launcher_toolkit::plugin_trait::tracing;


pub struct Container {
    pub name: String,
    pub id: String,
    pub image: String,
    pub state: String,
}


#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    tracing::info!("Connecting to Docker Socket");
    Ok(Docker::unix("/var/run/docker.sock"))
}

pub async fn docker_ps(docker: Arc<Mutex<Docker>>, container_db: Arc<Mutex<HashMap<String, Container>>>) -> Result<()> {
    let opts = ContainerListOpts::builder().all(true).build();
    match docker.lock().unwrap().containers().list(&opts).await {
        Ok(containers) => {
            let mut db = container_db.lock().unwrap();
            containers.into_iter().for_each(|container| {
                let name = get_name(container.names);
                db.insert( name.clone(), crate::Container {
                    name,
                    id: container.id.unwrap_or_default()[..12].to_owned(),
                    image: container.image.unwrap_or_default(),
                    state: container.state.unwrap_or_default(),
                });
            });
        }
        Err(e) => tracing::error!("Failed to get container list (docker ps).\n${e}"),
    }

    Ok(())
}

fn get_name(names: Option<Vec<String>>) -> String {
    names.map(|n| n[0].clone()).unwrap_or_default()
}
