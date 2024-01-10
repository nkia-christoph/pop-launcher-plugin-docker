mod docker;
mod plugin;

use crate::{
    docker::docker_ps,
    plugin::{
        ContextMap,
        Plugin,
        WrappedResult,
    },
};

use pop_launcher_toolkit::{
    launcher::{
        Indice,
        PluginResponse,
    },
    plugin_trait::{
        async_trait,
        PluginExt,
        tracing::*,
    },
};
use std::sync::Arc;
use tokio::time::error::Error;


#[async_trait]
impl PluginExt for Plugin {
    fn name(&self) -> &str {
        "docker"
    }

    async fn search(&mut self, query: &str) {
        info!("search: received query: ${query}");

        match query.split_once(' ') {
            None => self.handle_single_cmd(query).await,
            Some((_, second)) => {
                match second.is_empty() {
                    true => self.handle_single_cmd(query).await,
                    false => self.handle_query(query).await,
                }
                // list last active containers maybe and start them with enter?
            },
        }

        while (self.tasks.join_next().await).is_some() {};
        self.respond_with(PluginResponse::Finished).await;
    }

    async fn activate(&mut self, id: Indice) {
        info!("activate request received for result: ${id}");

        let result: Arc<WrappedResult>;
        {
            let guard_results = lock!(self.results);
            match guard_results.get(&id) {
                None => error_return!(
                    "could not get result: {}\n{:#?}",
                    &id,
                    &guard_results,
                ),
                Some(r) => result = Arc::clone(r),
            }
        }

        if result.action.is_none() {
            info_return!("no action for result: {}\n{:#?}", &id, &result,)
        };

        use plugin::Action::*;
        match result.action.unwrap() {
            Complete => {
                // todo extract method so we only need to lock & read mutex once
                self.complete(id).await;
            },
            Context => {
                let result = result.clone();
                match &result.context_options {
                    None => info_return!("no context for result: {}", &id),
                    Some(options) => {
                        info!("returning context");
                        self.view_context_options(&id, options.clone()).await;
                    },
                }
            },
        }
    }

    /// Define how the plugin should handle [`Request::ActivateContext`] request.
    /// Typically run the requested entry with the provided context (for instance using [`super::plugins::xdg_open`])
    /// and close the client with a [`PluginResponse::Close`]
    async fn activate_context(&mut self, result_id: Indice, context_id: Indice) {
        info!(
            "received context activation request for result: {} \
            and context: {}",
            &result_id,
            &context_id,
        );

        let result: Arc<WrappedResult>;
        match lock!(self.results).get(&result_id) {
            None => error_return!(
                "could not get context_option with id {} for result {}. \
                this should not be possible)",
                &context_id,
                &result_id,
            ),
            Some(r) => result = Arc::clone(r),
        }

        // can't do stuff without container_id
        if result.container_id.is_none() {
            error_return!(
                "cannot perform action for context: {} of result: {} \
                due to missing container_id. this should not be possible",
                &context_id,
                &result_id,
            )
        }

        let action: Option<docker::Action>;
        match &result.context_options {
            None => error_return!(
                "no context_options for this result: {}. \
                this should not be possible",
                &result_id,
            ),
            Some(guard_context) => {
                match lock!(guard_context).get(&context_id)
                {
                    None => error_return!(
                        "no context_option: {} for result: {}. \
                        this should not be possible",
                        &context_id,
                        &result_id,
                    ),
                    Some(context) => action = context.exec.clone(),
                }
            }
        }

        match action {
            None => info_return!(
                "no action set for context: {} of result: {}",
                &context_id,
                &result_id,
            ),
            Some(action) => {
                info!(
                    "executing action: {} for result: {}",
                    &action,
                    &result_id,
                );
                let container_id = result.container_id.clone().unwrap();
                action.execute(self, container_id.as_str(), None ).await;
            },
        }
    }

    /// Handle an autocompletion request from the client (on tab key press)
    async fn complete(&mut self, id: Indice) {
        info!("autocomplete request for result: {}", &id);

        let fill: String;
        match lock!(self.results).get(&id) {
            None => error_return!(
                "could not get result: {} for completion. \
                this should not happen",
                &id,
            ),
            Some(result) => {
                match &result.complete {
                    None => info_return!(
                        "no completion for result: {}\n{:#?}",
                        &id,
                        &result.complete,
                    ),
                    Some(complete) => fill = complete.to_owned(),
                }
            },
        }

        let response = PluginResponse::Fill(fill);
        self.respond_with(response).await;
    }

    /// Handle context request from the client (on right click)
    ///
    /// `pop-launcher` request the context for the given [`SearchResult`] id.
    /// to send the requested context use [`PluginResponse::Context`]
    async fn context(&mut self, _id: Indice) {
        warn!("context");

        let options: ContextMap;
        match lock!(self.results).get(&_id) {
            None => error_return!(
                "could not get result: {}. \
                this should not happen",
                &_id,
            ),
            Some(result) => {
                match &result.context_options {
                    None => info_return!(
                        "no context for result: {}\n{:#?}",
                        &_id,
                        &result.context_options,
                    ),
                    Some(context) => options = Arc::clone(context),
                }
            },
        }

        self.view_context_options(&_id, options).await;
    }

    /// Whenever a new search query is issued, `pop-launcher` will send a [`Request::Interrupt`]
    /// so we can stop any ongoing computation before handling the next query.
    /// This is especially useful for plugins that rely on external services
    /// to get their search results (a HTTP endpoint for instance)
    async fn interrupt(&mut self) {
        info!("interrupt");
        self.tasks.abort_all();
        self.tasks.detach_all();
        lock!(self.results).clear();
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    info!("docker plugin activated");

    let mut plugin: Plugin = Plugin::default();
    // ToDo: impl. communication/concurrency
    // tokio::join!(
    //     docker_ps(
    //         plugin.docker.clone(),
    //         plugin.containers.clone(),
    //         None,
    //     ),
    //     plugin.run(),
    // );
    docker_ps(
        plugin.docker.clone(),
        plugin.containers.clone(),
    ).await?;
    plugin.run().await;

    Ok(())
}
