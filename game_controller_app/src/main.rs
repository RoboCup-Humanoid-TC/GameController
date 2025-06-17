//! This crate defines the main program of the GameController application.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::Parser;
use tauri::{async_runtime, generate_context, Manager, RunEvent, WindowBuilder, WindowUrl};

use game_controller_runtime::{
    cli::Args, launch::make_launch_data, shutdown_runtime, RuntimeState,
};

mod handlers;

use handlers::get_invoke_handler;

/// This function runs the tauri app. It first parses command line arguments and displays a
/// launcher in which the user can configure the settings for the game. When the user is done with
/// that, the main window and network services are started and shut down when the app is quit.
fn main() {
    // Parse the command line arguments first. This includes handling the version and help commands
    // and wrong arguments.
    let args = Args::parse();

    // We want to manage an external tokio runtime, mainly to keep dependencies to tauri minimal,
    // but also because I don't know how to do the shutdown correctly otherwise.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    async_runtime::set(runtime.handle().clone());

    let app = tauri::Builder::default()
        .setup(|app| {
            // TODO: This will probably not work in production.
            let config_directory = app
                .path_resolver()
                .resource_dir()
                .unwrap()
                .join("..")
                .join("..")
                .join("config");
            match make_launch_data(&config_directory, args) {
                Ok(launch_data) => {
                    app.manage(launch_data);
                }
                Err(error) => {
                    eprintln!("{error:?}");
                    app.handle().exit(1);
                }
            };

            let _window = WindowBuilder::new(app, "main", WindowUrl::App("index.html".into()))
                .center()
                .inner_size(640.0, 480.0)
                .resizable(false)
                .title("GameController")
                .build()?;
            Ok(())
        })
        .invoke_handler(get_invoke_handler())
        .build(generate_context!())
        .expect("error while running tauri application");

    app.run(move |handle, event| {
        if let RunEvent::Exit = event {
            if let Some(runtime_state) = handle.try_state::<RuntimeState>() {
                runtime.block_on(shutdown_runtime(&runtime_state));
            }
        }
    });
}
