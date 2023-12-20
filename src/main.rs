mod docker;
mod container;
use docker_api::Docker;
use pop_launcher_toolkit::launcher::{
    Indice,
    IconSource,
    PluginResponse,
    PluginSearchResult,
};
use pop_launcher_toolkit::plugins::send;
use pop_launcher_toolkit::plugin_trait::{
    async_trait,
    tracing,
    PluginExt,
};
use tokio::{
    task::JoinSet,
    time::error::Error,
};
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::Arc,
    sync::Mutex,
    time::SystemTime,
};
use crate::container::Container;


pub struct Plugin {
    pub icon: Option<IconSource>,
    pub docker: Docker,
    pub containers: Arc<Mutex<HashMap<String, Container>>>,
    pub timestamp: SystemTime,
    pub results: Arc<Mutex<Vec<PluginSearchResult>>>,
}

impl Default for Plugin {
    fn default() -> Self {
        Self {
            icon: Some(IconSource::Mime(Cow::Borrowed("docker-icon.png"))),
            docker: docker::new_docker().unwrap(),
            containers: Arc::new(Mutex::new(HashMap::new())),
            timestamp: SystemTime::now(),
            results: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Plugin {
    #[allow(unused_variables)]
    fn get_description(&self, container: &Container) -> &'static str {
        "test description"
    }

    async fn handle_single_cmd(&mut self, _: &str) {
        // ps, dps, dcps, cps
        tracing::info!(" - process 1-word-command");
        let mut task_set = JoinSet::new();

        for (id, (name, container)) in
            self.containers.lock().unwrap().iter().enumerate()
        {
            let result = Arc::new(PluginSearchResult {
                id: id as Indice,
                name: name.to_owned(),
                description: self.get_description(container).to_owned(),
                icon: self.icon.to_owned(),
                ..Default::default()
            });

            let result_1 = result.clone();
            task_set.spawn( async move {
                send(&mut tokio::io::stdout(),
                    PluginResponse::Append(
                        PluginSearchResult::clone(
                            result_1.as_ref()
                ))).await;
            });

            let results = self.results.clone();
            let result_2 = result.clone();
            task_set.spawn( async move {
                add(results, id, result_2).await;
            });
        }
        while (task_set.join_next().await).is_some() {};
    }
}

#[async_trait]
impl PluginExt for Plugin {

    fn name(&self) -> &str {
        "docker"
    }

    async fn search(&mut self, query: &str) {
        tracing::info!("Received query: ${query}");

        match query.split_once(' ') {
            None => self.handle_single_cmd(query.as_ref()).await,
            Some(split_query) => {
                let result = PluginSearchResult {
                    id: 0 as Indice,
                    name: "Docker is down.".to_owned(),
                    description: "No active containers. Would you like to start a recent one?".to_owned(),
                    icon: self.icon.to_owned(),
                    ..Default::default()
                };
                self.respond_with(PluginResponse::Append(result)).await
                // list last active containers maybe and start them with enter?
            }
        }

        self.respond_with(PluginResponse::Finished).await;
    }

    async fn activate(&mut self, id: Indice) {
        tracing::info!("Plugin activated");

        todo!()
        //add context: restart, stop, exec, append, etc.
    }
}

async fn add(db: Arc<Mutex<Vec<PluginSearchResult>>>, id: usize, result: Arc<PluginSearchResult>) {
    let mut results = db.lock().unwrap();
    results.insert(id, PluginSearchResult::clone(&result));
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {

    let mut plugin: Plugin = Plugin::default();
    tracing::info!("Started docker plugin");

    plugin.run().await;

    Ok(())
}
