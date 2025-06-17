//! This module defines handlers that can be called from JavaScript.

use std::sync::Arc;

use anyhow::{anyhow, Context};
use tauri::{
    command, generate_handler, AppHandle, InvokeHandler, LogicalSize, Manager, State, Window, Wry,
};
use tokio::sync::Notify;

use game_controller_core::{action::VAction, types::Params};
use game_controller_runtime::{
    launch::{LaunchData, LaunchSettings},
    start_runtime, RuntimeState,
};

/// This struct is used as state so that the [launch] function can communicate to
/// [sync_with_backend] that the full [RuntimeState] is managed now.
struct SyncState(Arc<Notify>);

/// This function is called by the launcher to obtain its data. The data is read from a state
/// variable that is created by [game_controller_runtime::launch::make_launch_data] and put there by
/// [crate::main].
#[command]
fn get_launch_data(launch_data: State<LaunchData>) -> LaunchData {
    launch_data.inner().clone()
}

/// This function is called when the user finishes the launcher dialog. It creates a game state and
/// network services, and spawns tasks to handle events.
#[command]
async fn launch(settings: LaunchSettings, window: Window, app: AppHandle) {
    // The notify object must be managed before the window is created.
    let runtime_notify = Arc::new(Notify::new());
    app.manage(SyncState(runtime_notify.clone()));

    // Unfortunately we cannot use the number of players per team here.
    let size = LogicalSize::<f64>::new(
        1024.0,
        if settings.competition.id == "champions_cup" {
            768.0
        } else {
            620.0
        },
    );
    let _ = window.set_min_size(Some(size));
    #[cfg(target_os = "windows")]
    let _ = window.set_size(size);
    let _ = window.set_fullscreen(settings.window.fullscreen);
    let _ = window.set_resizable(true);
    let _ = window.center();

    let send_ui_state = move |ui_state| {
        if let Err(error) = window.emit("state", ui_state) {
            Err(anyhow!(error))
        } else {
            Ok(())
        }
    };

    let launch_data = app.state::<LaunchData>();
    match start_runtime(
        // TODO: This will probably not work in production.
        &app.path_resolver()
            .resource_dir()
            .unwrap()
            .join("..")
            .join("..")
            .join("config"),
        &app.path_resolver()
            .resource_dir()
            .unwrap()
            .join("..")
            .join("..")
            .join("logs"),
        &settings,
        &launch_data.teams,
        &launch_data.network_interfaces,
        Box::new(send_ui_state),
    )
    .await
    .context("could not start runtime")
    {
        Ok(runtime_state) => {
            app.manage(runtime_state);
        }
        Err(error) => {
            eprintln!("{error:?}");
            app.exit(1);
        }
    }

    // Now that the RuntimeState is managed, we can tell the UI that it can proceed.
    runtime_notify.notify_one();
}

/// This function should be called once by the UI after it listens to UI events, but before it
/// calls [apply_action] or [declare_actions]. The caller gets the combined parameters of the game
/// and competition. It is wrapped in a [Result] as a tauri workaround.
#[command]
async fn sync_with_backend(app: AppHandle, state: State<'_, SyncState>) -> Result<Params, ()> {
    // Wait until manage has been called.
    state.0.notified().await;
    // Now we can obtain a handle to the RuntimeState to notify the runtime thread that it can
    // start sending UI events.
    let runtime_state = app.state::<RuntimeState>();
    runtime_state.ui_notify.notify_one();
    Ok(runtime_state.params.clone())
}

/// This function enqueues an action to be applied to the game.
#[command]
fn apply_action(action: VAction, state: State<RuntimeState>) {
    let _ = state.action_sender.send(action);
}

/// This function lets the UI declare actions for which it wants to know whether they are legal.
#[command]
fn declare_actions(actions: Vec<VAction>, state: State<RuntimeState>) {
    let _ = state.subscribed_actions_sender.send(actions);
}

/// This function returns a handler that can be passed to [tauri::Builder::invoke_handler].
/// It must be boxed because otherwise its size is unknown at compile time.
pub fn get_invoke_handler() -> Box<InvokeHandler<Wry>> {
    Box::new(generate_handler![
        apply_action,
        declare_actions,
        get_launch_data,
        launch,
        sync_with_backend,
    ])
}
