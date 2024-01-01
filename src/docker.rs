use docker_api::{
    opts::ContainerListOpts,
    Docker,
    Result,
};
use pop_launcher_toolkit::{
    plugin_trait::tracing::*,
    launcher::IconSource,
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
    sync::Mutex,
    borrow::Cow,
};
use strum::{
    Display,
    EnumString,
};

macro_rules! mime_icon{
    ($a:expr) => {
        Some(IconSource::Mime(Cow::Borrowed($a)))
    }
}

#[macro_export]
macro_rules! new_docker{
    ($a:expr) => {
        docker_api::Docker::unix($a)
    }
}

pub struct Container {
    pub name: String,
    pub id: String,
    pub image: String,
    pub state: State,
    pub icon: Option<IconSource>,
}

#[derive(Debug, PartialEq, EnumString, Display)] // ensure lowercase states match enums
#[strum(ascii_case_insensitive)]
pub enum State {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

/// represent state of Docker Container (as far as we know)
/// 'Dead'
impl State {
    fn get_icon(&self) -> Option<IconSource> {
        use State::*;
        let default: &str = "./docker-icon.png";
        match self {
            Created => mime_icon!(&default),
            Restarting => mime_icon!(&default),
            Running => mime_icon!(&default),
            Removing => mime_icon!(&default),
            Paused => mime_icon!(&default),
            Exited => mime_icon!(&default),
            Dead => mime_icon!(&default),
        }
    }

    /// prepend unicode icon to name
    /// until we can provide categ. icon through PluginSearchResult
    fn get_unicode(&self) -> &str {
        use State::*;
        match self {
            Created => "\u{2714}", // ✔ U+2714
            Restarting => "\u{231B}", // ⌛ U+231B
            Running => "\u{1F197}", // 🆗 U+1F197
            Removing => "\u{267B}", // ♻ U+267B
            Paused => "\u{23F8}", // ⏸ U+23F8
            Exited => "\u{1F5D1}__", // 🗑 U+1F5D1
            Dead => "\u{2620}", // ☠ U+2620
        }
    }
}

pub async fn docker_ps<'a>(
    docker: Arc<Mutex<Docker>>,
    container_db: Arc<Mutex<HashMap<String, Container>>>,
    opts: Option<&ContainerListOpts>,
) -> Result<()> {
    // handle default
    let default: ContainerListOpts;
    let opts = match opts {
        Some(provided) => provided,
        None => {
            default = ContainerListOpts::builder().all(true).build();
            &default
        },
    };

    match docker.lock()
                .expect("could not lock onto Plugin.docker")
                .containers().list(opts).await
    {
        Ok(containers) => {
            let mut db = container_db.lock().unwrap();
            containers.into_iter().for_each(|container| {
                let name = get_name(&container.names);
                let state = State::from_str(
                    container.state.unwrap_or_default().as_str()
                ).unwrap_or(State::Dead);

                db.insert( name.to_owned(), crate::Container {
                    name: format!("{} {}",
                        state.get_unicode(),
                        name,
                    ),
                    id: container.id.unwrap_or_default()[..12].to_owned(),
                    image: container.image.unwrap_or_default(),
                    icon: state.get_icon(),
                    state,
                });
            });
        }
        Err(e) => error!("failed to get container list (docker ps).\n${e}"),
    }

    Ok(())
}

fn get_name<'a>(names: &'a Option<Vec<String>>) -> &'a str {
    let name: &str = names
        .as_ref()
        .map(|n: &'a Vec<String>|
            n[0].as_str()
        )
        .unwrap_or_else(|| {
            error!("could not get container name");
            let placeholder: &'a str = "/Error";
            placeholder
        });

    // remove trailing '/' from docker_api::models::ContainerSummary.names
    // remove fn as soon as this has been fixed in docker_api
    let fixed_name: &'a str = &name[1..];
    fixed_name
}
