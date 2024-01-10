use crate::{
    icon_borrowed,
    lock,
    Plugin,
    plugin::ContainerMap,
};

use docker_api::{
    models::ContainerSummary,
    opts::*,
    Docker,
};
use pop_launcher_toolkit::{
    plugin_trait::tracing::*,
    launcher::IconSource,
};
use std::{
    str::FromStr,
    sync::Arc,
    sync::Mutex,
};
use strum::{
    Display,
    EnumString,
};
use tokio::time::error::Error;


#[macro_export]
macro_rules! new_docker{
    ($a:expr) => {
        docker_api::Docker::unix($a)
    }
}

#[macro_export]
macro_rules! filter_all{
    () => {
        vec![
            State::Created,
            State::Restarting,
            State::Running,
            State::Removing,
            State::Paused,
            State::Exited,
            State::Dead,
        ]
    }
}

#[macro_export]
macro_rules! filter_default{
    () => {
        vec![
            State::Created,
            State::Restarting,
            State::Running,
            State::Removing,
            State::Paused,
        ]
    }
}

#[macro_export]
macro_rules! filter_non_default{
    () => {
        vec![
            State::Paused,
            State::Exited,
            State::Dead,
        ]
    }
}

pub type ContainerFilter = Vec<State>;

#[derive(Debug)]
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

#[derive(Debug, Display, Clone)]
pub enum Action {
    Attach,
    Start,
    Stop,
    Kill,
    Restart,
    Pause,
    Unpause,
    Remove,
    Inspect,
    Exec,
    Logs,
}


impl Action {
    pub async fn execute(&self,
        plugin: &mut Plugin,
        id: &str,
        opts: Option<String>,
    ) {
        let guard_docker = lock!(plugin.docker);
        let _container = docker_api::Container::new(guard_docker.to_owned(), id);
        use Action::*;
        match self {
            Attach => {
                // container
                //     .attach()
                //     .await
                //     .expect("failed to attach to container");
                todo!();
            },
            Start => {
                // container
                //     .start()
                //     .await
                //     .expect("failed to start container");
                todo!();
            },
            Stop => {
                let _stop_opts = opts
                    .map(|_| {
                        ContainerStopOpts::builder().build()
                    })
                    .unwrap();
                // container
                //     .stop(&stop_opts)
                //     .await
                //     .expect("msg");
                todo!();
            },
            Kill => {
                // container
                //     .kill(None)
                //     .await
                //     .expect("failed to kill container");
                todo!();
            },
            Restart => {
                let _restart_opts = opts
                    .map(|_| {
                        ContainerRestartOpts::builder().build()
                    })
                    .unwrap();
                // container
                //     .restart(&restart_opts)
                //     .await
                //     .expect("failed to restart container");
                todo!();
            },
            Pause => {
                // container
                //     .pause()
                //     .await
                //     .expect("failed to pause container");
                todo!();
            },
            Unpause => {
                // container
                //     .unpause()
                //     .await
                //     .expect("failed to unpause container");
                todo!();
            },
            Remove => {
                let _remove_opts = opts
                    .map(|_| {
                        ContainerRemoveOpts::builder().build()
                    })
                    .unwrap();
                // container
                //     .remove(&remove_opts)
                //     .await
                //     .expect("failed to remove container");
                todo!();
            },
            Inspect => {
                // container
                //     .inspect()
                //     .await
                //     .expect("failed to inspect container");
                todo!();
            },
            Exec => {
                let _create_opts = ExecCreateOpts::builder().build();
                let _start_opts = opts
                    .map(|_| {
                        ExecStartOpts::builder()
                            .detach(false)
                            .tty(false)
                            .build()
                    })
                    .unwrap();
                // container
                //     .exec(&create_opts, &start_opts)
                //     .await
                //     .expect("failed to execute command");
                todo!();
            },
            Logs =>  {
                let _log_opts = opts
                    .map(|_| {
                        LogsOpts::builder()
                            .follow(true)
                            .stdout(true)
                            .stderr(true)
                            //.since((o));
                            //.n_lines((o));
                            .build()
                    })
                    .unwrap_or_else(|| {
                        LogsOpts::builder().build()
                    });

                plugin.tasks.spawn( async move {
                    // container
                    //     .logs(&log_opts)
                    todo!()
                });
                todo!();
            },
        }
    }
}

/// represent state of Docker Container (as far as we know)
impl State {
    /// get possible actions for container of this state
    pub fn actions(&self) -> Option<Vec<Action>>{
        use State::*;
        use Action::*;
        match self {
            Created => Some(vec![Attach, Start, Remove]),
            Restarting => None,
            Running => Some(vec![Attach, Stop, Kill,
                                Restart, Pause, Unpause,
                                Remove, Inspect, Exec, Logs]),
            Removing => None,
            Paused => Some(vec![Unpause, Remove]),
            Exited => Some(vec![Start, Remove]),
            Dead => Some(vec![Start, Remove]),
        }
    }

    fn icon(&self) -> Option<IconSource> {
        use State::*;
        let default: &str = "./docker-icon.png";
        match self {
            Created => icon_borrowed!(&default),
            Restarting => icon_borrowed!(&default),
            Running => icon_borrowed!(&default),
            Removing => icon_borrowed!(&default),
            Paused => icon_borrowed!(&default),
            Exited => icon_borrowed!(&default),
            Dead => icon_borrowed!(&default),
        }
    }

    /// prepend unicode icon to name
    /// until we can provide category icon through PluginSearchResult
    fn unicode(&self) -> &str {
        use State::*;
        match self {
            Created => "\u{2714}", // ✔ U+2714
            Restarting => "\u{231B}", // ⌛ U+231B
            Running => "\u{1F197}", // 🆗 U+1F197
            Removing => "\u{267B}", // ♻ U+267B
            Paused => "\u{23F8}", // ⏸ U+23F8
            Exited => "\u{1F5D1}", // 🗑 U+1F5D1
            Dead => "\u{2620}", // ☠ U+2620
        }
    }
}

pub async fn docker_ps<'a>(
    docker: Arc<Mutex<Docker>>,
    container_db: ContainerMap
) -> Result<(), Error> {
    let opts = ContainerListOpts::builder().all(true).build();
    let containers: Vec<ContainerSummary>;

    #[allow(clippy::await_holding_lock)]
    {
        let guard_docker = lock!(docker);
        containers = guard_docker
            .containers()
            .list(&opts)
            .await
            .expect("failed to get container list (docker ps)");
    }
    {
        let mut guard_db = lock!(container_db);

        containers.into_iter().for_each(|container| {
            let name = name(&container.names);
            let state = State::from_str(container
                .state
                .unwrap_or_default()
                .as_str()
            ).unwrap_or(State::Dead);

            let full_id = container.id.unwrap();
            guard_db.insert( full_id.to_owned(), Container {
                name: format!("{} {}",
                    state.unicode(),
                    name,
                ),
                id: full_id[..12].to_owned(),
                image: container.image.unwrap_or_default(),
                icon: state.icon(),
                state,
            });
        });
    }

    Ok(())
}

fn name<'a>(names: &'a Option<Vec<String>>) -> &'a str {
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
    // TODO: remove fn as soon as this has been fixed in docker_api
    let fixed_name: &'a str = &name[1..];
    fixed_name
}
