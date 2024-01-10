use crate::{
    docker::{
        Container,
        ContainerFilter,
        State, self,
    },
    filter_all,
    filter_default,
    filter_non_default,
    new_docker,
};

use docker_api::Docker;
use pop_launcher_toolkit::launcher::{
    async_stdout,
    ContextOption,
    Indice,
    PluginResponse,
    PluginSearchResult,
};
use pop_launcher_toolkit::{
    plugin_trait::tracing::*,
    plugins::send,
};
use std::{
    collections::HashMap,
    env::current_dir,
    sync::Arc,
    sync::Mutex,
    time::SystemTime,
};
use tokio::task::JoinSet;


#[macro_export]
macro_rules! icon_borrowed{
    ( $a:expr ) => {
        Some(pop_launcher_toolkit::launcher::IconSource::Name(std::borrow::Cow::Borrowed($a)))
    }
}

#[macro_export]
macro_rules! icon_owned{
    ( $a:expr ) => {
        Some(pop_launcher_toolkit::launcher::IconSource::Name(std::borrow::Cow::Owned($a)))
    }
}

#[macro_export]
/// log & panic if mutex is poisoned
macro_rules! lock{
    ( $mutex:expr ) => {
        match $mutex.lock() {
            Ok(guard) => guard,
            Err(why) => {
                error!(
                    "could not lock on to {:#?}\n{}",
                    $mutex,
                    &why,
                );
                panic!()
            },
        }
    }
}

#[macro_export]
/// log error and return
macro_rules! error_return{
    ( $msg:expr $(, $x:expr)* $(,)? ) => {
        {
            error!($msg, $( $x, )*);
            return
        }
    };
}

#[macro_export]
/// log info and return
macro_rules! info_return{
    ( $msg:expr $(, $x:expr)* $(,)? ) => {
        {
            info!($msg, $( $x, )*);
            return
        }
    };
}


pub type ContainerMap = Arc<Mutex<HashMap<String, Container>>>;
pub type ResultMap = Arc<Mutex<HashMap<Indice, Arc<WrappedResult>>>>;
pub type ContextMap = Arc<Mutex<HashMap<Indice, Arc<WrappedContext>>>>;


pub struct Plugin {
    //pub out: Arc<tokio::io::Stdout>,
    pub icon: String,
    pub docker: Arc<Mutex<Docker>>,
    pub containers: ContainerMap,
    pub timestamp: SystemTime,
    pub results: ResultMap,
    pub tasks: JoinSet<()>,
}

#[derive(Debug, Default)]
pub struct WrappedResult {
    pub action: Option<Action>,
    pub complete: Option<String>,
    pub container_id: Option<String>,
    pub context_options: Option<ContextMap>,
    pub result: PluginSearchResult,
    //pub exec: dyn Fn(),
}

#[derive(Debug)]
pub struct WrappedContext {
    pub name: String,
    pub exec: Option<docker::Action>,
}


/// Default Action for a PluginSearchResult (on enter)
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Complete/Replace the search with the provided string
    Complete,
    /// Show Context Options for the search result
    Context,
    // TODO: Execute the provided function
    //Execute,
}


impl Default for Plugin {
    fn default() -> Self {
        Self {
            //out: Arc::new(async_stdout()),
            // TODO: handle icon as option
            icon: current_dir()
                .unwrap_or_default()
                .join("docker-icon.png")
                .to_str()
                .unwrap_or_default()
                .to_owned(),
            docker: Arc::new(Mutex::new(new_docker!("/var/run/docker.sock"))),
            containers: Arc::new(Mutex::new(HashMap::new())),
            timestamp: SystemTime::now(),
            results: Arc::new(Mutex::new(HashMap::new())),
            tasks: JoinSet::new(),
        }
    }
}

impl Plugin {
    fn description(&self, container: &Container) -> String {
        format!("{}, image: {}, id: {}",
            container.state,
            container.image,
            container.id,
        )
    }

    /// handles single command string that matches `docker-ps/plugin.ron`.
    /// valid commands: dps, cps, dl, cl, dla, cla
    pub async fn handle_single_cmd(&mut self, command: &str) {
        info!(" - process 1-word-command");

        match command {
            "dl" | "cl" | "dps" | "cps" => self.list().await,
            "dla" | "cla" => self.list_all().await,
            _ => unreachable!(), // TODO: handle & display error
        };
    }

    async fn list(&mut self) {
        let filter: ContainerFilter = filter_default!();
        match self.view_containers(&filter).await {
            None => self.no_active_notice().await,
            Some(viewed) => {
                let containers = lock!(self.containers).len() as Indice;
                match containers {
                    0 => self.no_active_notice().await,
                    _ if viewed < containers => self.no_visible_notice().await,
                    _ => (),
                }
            },
        }
    }

    async fn list_all(&mut self) {
        let filter: ContainerFilter = filter_all!();
        if self.view_containers(&filter).await.is_none() {
            self.no_active_notice().await
        }
    }

    pub async fn handle_query(&mut self, _query: &str) {
        info!(" - process space separated query");

        let filter: ContainerFilter = filter_non_default!();
        match self.view_containers(&filter).await {
            None => self.no_active_notice().await,
            Some(viewed) => {
                let containers = lock!(self.containers).len() as Indice;
                match containers {
                    0 => self.no_active_notice().await,
                    _ if viewed < containers => self.no_visible_notice().await,
                    _ => (),
                }
            },
        }
    }

    pub async fn no_active_notice(&mut self) {
        info!(" - display no active containers notice");

        let result = PluginSearchResult {
            id: 0 as Indice,
            name: "No active containers".to_owned(),
            description: "Would you like to start a recent one?".to_owned(),
            icon: icon_borrowed!("dialog-error"),
            //category_icon: self.icon.to_owned(),
            ..Default::default()
        };

        {
            let result = result.clone();
            let results = self.results.clone();

            self.tasks.spawn( async move {
                add(results, 0 as Indice, WrappedResult {
                    action: Some(Action::Complete),
                    complete: Some("dla".to_owned()),
                    result,
                    ..Default::default()
                }).await;
            });
        }
        send(&mut async_stdout(),PluginResponse::Append(result)).await;
    }

    pub async fn no_visible_notice(&mut self) {
        info!(" - display no visible containers notice");

        let result = PluginSearchResult {
            id: 0 as Indice,
            name: "No visible containers".to_owned(),
            description: "Would you like to see all containers?".to_owned(),
            icon: icon_borrowed!("dialog-error"),
            //category_icon: self.icon.to_owned(),
            ..Default::default()
        };

        {
            let result = result.clone();
            let results = self.results.clone();
            self.tasks.spawn( async move {
                add(results, 0 as Indice, WrappedResult {
                    action: Some(Action::Complete),
                    complete: Some("dla".to_owned()),
                    result,
                    ..Default::default()
                }).await;
            });
        }
        send(&mut async_stdout(),PluginResponse::Append(result)).await;
    }

    /// Display containers that match the provided filter.
    /// Return an option of the number of containers.
    pub async fn view_containers(&mut self, filter: &ContainerFilter) -> Option<Indice> {
        info!("   - filtering containers");

        let mut id: Indice = 0;

        for (container_id, container) in lock!(self.containers)
            .iter()
            .filter(|container|{
                filter.contains(&container.1.state)
            })
        {
            // reserve index 0 for notices
            id += 1;

            let result = PluginSearchResult {
                id,
                name: container.name.to_owned(),
                description: self.description(container).to_owned(),
                icon: icon_owned!(format!("{}", &self.icon)),
                ..Default::default()
            };

            {
                info!(" - scheduling returning result");

                let result = result.clone();
                self.tasks.spawn( async move {
                    send(&mut async_stdout(),
                        PluginResponse::Append(
                            PluginSearchResult::clone(
                                &result
                    ))).await;
                });
            }
            {
                info!(" - scheduling adding result to hashmap");

                let results = Arc::clone(&self.results);
                let context_options = container
                    .state
                    .actions()
                    .map(|actions| {
                        let mut context_options: HashMap<Indice, Arc<WrappedContext>> = HashMap::new();
                        for (i, action) in
                            actions.iter().enumerate()
                        {
                            context_options.insert(i as Indice, Arc::new( WrappedContext {
                                name: action.to_string(),
                                exec: Some(action.clone()),
                            }));
                        }
                        Arc::new(Mutex::new(context_options))
                });

                let result = WrappedResult{
                    result,
                    container_id: Some(container_id.to_owned()),
                    context_options,
                    action: Some(Action::Context),
                    ..Default::default()
                };
                self.tasks.spawn( async move {
                    add(results, id, result).await;
                });
            }
        }

        match id {
            0 => None,
            _ => Some(id),
        }
    }

    pub async fn view_context_options(&mut self, id: &Indice, options: ContextMap) {
        info!(" - scheduling returning context options");

        let context_options: Vec<ContextOption>;
        {
            context_options = lock!(options)
                .iter()
                .map(|(i, context)| ContextOption {
                    id: *i,
                    name: context.name.to_owned(),
                })
                .collect();
        }

        send(&mut async_stdout(), PluginResponse::Context {
            id: *id,
            options: context_options,
        }).await;

    }
}

async fn add(db: ResultMap, id: Indice, wrapped: WrappedResult) {
    info!(" - adding result");

    let mut guard_results = lock!(db);
    guard_results.insert(id, Arc::new(wrapped));
}
