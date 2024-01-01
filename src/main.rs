mod docker;
mod plugin;

use crate::{
    docker::docker_ps,
    docker::Container,
    plugin::Plugin,
};

use pop_launcher_toolkit::{
    launcher::Indice,
    launcher::PluginResponse,
    launcher::PluginSearchResult,
    plugin_trait::async_trait,
    plugin_trait::tracing::*,
    plugin_trait::PluginExt,
};
use tokio::time::error::Error;


#[async_trait]
impl PluginExt for Plugin {
    fn name(&self) -> &str {
        "docker"
    }

    async fn search(&mut self, query: &str) {
        info!("received query: ${query}");

        match query.split_once(' ') {

            None => self.handle_single_cmd(query).await,
            Some((_, second)) => {
                match second.is_empty() {
                    true => self.handle_single_cmd(query).await,
                    false => {
                        let result = PluginSearchResult {
                            id: 0 as Indice,
                            name: "No active containers".to_owned(),
                            description: "Would you like to start a recent one?".to_owned(),
                            //SearchResult: category_icon: self.icon.to_owned(),
                            ..Default::default()
                        };
                        self.respond_with(PluginResponse::Append(result)).await
                    }
                }
                // list last active containers maybe and start them with enter?
            }
        }

        self.respond_with(PluginResponse::Finished).await;
    }

    async fn activate(&mut self, id: Indice) {
        info!("Plugin activated");

        todo!()
        //add context: restart, stop, exec, append, etc.
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    info!("docker plugin activated");

    let mut plugin: Plugin = Plugin::default();
    // ToDo: impl. communication inbetween
    // let _ = tokio::join!(
    //     docker_ps(
    //         plugin.docker.clone(),
    //         plugin.containers.clone(),
    //         None,
    //     ),
    //     plugin.run(),
    // );
    let _ = docker_ps(
        plugin.docker.clone(),
        plugin.containers.clone(),
        None,
    ).await;
    let _ = plugin.run().await;

    Ok(())
}
