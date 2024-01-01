use crate::{
    docker::Container,
    new_docker,
};

use docker_api::Docker;
use pop_launcher_toolkit::launcher::{
    Indice,
    IconSource,
    PluginResponse,
    PluginSearchResult,
};
use pop_launcher_toolkit::{
    plugin_trait::tracing::*,
    plugins::send,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
    sync::Mutex,
    time::SystemTime,
};
use tokio::task::JoinSet;


pub type ContainerMap = Arc<Mutex<HashMap<String, Container>>>;
pub type ResultMap = Arc<Mutex<HashMap<Indice, PluginSearchResult>>>;

pub struct Plugin {
    pub icon: Option<IconSource>,
    pub docker: Arc<Mutex<Docker>>,
    pub containers: ContainerMap,
    pub timestamp: SystemTime,
    pub results: ResultMap,
}


impl Default for Plugin {
    fn default() -> Self {
        Self {
            icon: Some(IconSource::Name(Cow::Borrowed("./docker-icon.png"))),
            docker: Arc::new(Mutex::new(new_docker!("/var/run/docker.sock"))),
            containers: Arc::new(Mutex::new(HashMap::new())),
            timestamp: SystemTime::now(),
            results: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Plugin {
    #[allow(unused_variables)]
    fn get_description(&self, container: &Container) -> String {
        format!("{}, image: {}, id: {}",
            container.state,
            container.image,
            container.id
        )
    }

    pub async fn handle_single_cmd(&mut self, _: &str) {
        // dps, dcps, cps
        info!(" - process 1-word-command");
        let mut task_set = JoinSet::new();

        for (id, (name, container)) in
            self.containers.lock()
                .expect("could not lock on to Plugin.containers")
                .iter().enumerate()
        {
            let result = Arc::new(PluginSearchResult {
                id: id as Indice,
                name: name.to_owned(),
                description: self.get_description(container).to_owned(),
                icon: self.icon.to_owned(),
                ..Default::default()
            });

            {
                info!(" - scheduling returning result");
                let result = result.clone();
                task_set.spawn( async move {
                    send(&mut tokio::io::stdout(),
                        PluginResponse::Append(
                            PluginSearchResult::clone(
                                result.as_ref()
                    ))).await;
                });
            }
            {
                info!(" - scheduling adding result to hashmap");
                let results = self.results.clone();
                let result = result.clone();
                task_set.spawn( async move {
                    add(results, id, result).await;
                });
            }
        }
        while (task_set.join_next().await).is_some() {};
    }
}

async fn add(db: ResultMap, id: usize, result: Arc<PluginSearchResult>) {
    info!(" - adding result");

    let mut results = db.lock().unwrap();
    results.insert(id as Indice, PluginSearchResult::clone(&result));
}
