use docker_api::{
    opts::ContainerListOpts,
    Docker,
    Result,
};
use pop_launcher_toolkit::{
    plugin_trait::tracing,
    launcher::IconSource,
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
    sync::Mutex,
    borrow::Cow, fmt::format,
};
use strum::{
    Display,
    EnumString,
};

macro_rules! micon{
    ($a:expr) => {
        Some(IconSource::Mime(Cow::Borrowed($a)))
    }
}

pub struct Container {
    pub name: String,
    pub id: String,
    pub image: String,
    pub state: String,
    pub icon: Option<IconSource>,
}

#[derive(Debug, PartialEq, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum StateIcon {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl StateIcon {
    fn get(&self) -> Option<IconSource> {
        use StateIcon::*;
        match self {
            Created => micon!("ello"),
            Restarting => micon!("Hello"),
            Running => micon!("Hello"),
            Removing => micon!("Hello"),
            Paused => micon!("Hello"),
            Exited => micon!("Hello"),
            Dead => micon!("some_icon"),
        }
    }
}

#[cfg(unix)]
pub fn new_docker() -> Result<Docker> {
    tracing::debug!("Connecting to Docker Socket");
    Ok(Docker::unix("/var/run/docker.sock"))
}

pub async fn docker_ps(docker: Arc<Mutex<Docker>>, container_db: Arc<Mutex<HashMap<String, Container>>>) -> Result<()> {
    let opts = ContainerListOpts::builder().all(true).build();
    match docker.lock().unwrap().containers().list(&opts).await {
        Ok(containers) => {
            let mut db = container_db.lock().unwrap();
            containers.into_iter().for_each(|container| {
                let name = get_name(&container.names);
                let state = container.state.unwrap_or_default();
                let icon = StateIcon::from_str(state.as_ref()).unwrap().get();
                db.insert( name.to_owned(), crate::Container {
                    name: name.to_owned(),
                    id: container.id.unwrap_or_default()[..12].to_owned(),
                    image: container.image.unwrap_or_default(),
                    icon,
                    state,
                });
            });
        }
        Err(e) => tracing::error!("Failed to get container list (docker ps).\n${e}"),
    }

    Ok(())
}


/// remove trailing '/' from docker_api::models::ContainerSummary.names
fn get_name<'a>(names: &'a Option<Vec<String>>) -> &'a str {
    let name: &str = names
        .as_ref()
        .map(|n: &'a Vec<String>|
            n[0].as_str()
        )
        .unwrap();
    let fixed_name: &'a str = &name[1..];
    fixed_name
}
