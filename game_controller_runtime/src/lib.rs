//! This module defines the main runtime of the GameController application.

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    net::{IpAddr, Ipv4Addr},
    path::Path,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Error, Result};
use serde::Serialize;
use serde_with::{serde_as, BoolFromInt};
use time::{macros::format_description, OffsetDateTime};
use tokio::{
    fs::create_dir_all,
    select,
    sync::{broadcast, mpsc, watch, Mutex, Notify},
    task::JoinSet,
    time::{sleep_until, Instant},
};
use tokio_util::sync::CancellationToken;

use game_controller_core::{
    action::VAction,
    actions::{TeamMessage, HlSetGoalkeeper},
    log::{
        LogEntry, LoggedMetadata, LoggedMonitorRequest, LoggedStatusMessage, LoggedTeamMessage,
        TimestampedLogEntry,
    },
    types::{ActionSource, Game, League, Params, PlayerNumber, Side},
    GameController,
};
use game_controller_msgs::{HlStatusMessage, MonitorRequest, RefereeMessage, StatusMessage};
use game_controller_net::{
    ControlMessageSender, Event, MonitorRequestReceiver, RefereeReceiver, RefereeSender,
    StatusMessageForwarder, StatusMessageReceiver, TeamMessageReceiver,
};

pub mod cli;
mod connection_status;
pub mod launch;
mod logger;

use connection_status::{
    get_connection_status_map, get_next_connection_status_change, AlivenessTimestampMap,
    ConnectionStatusMap,
};
use launch::{LaunchSettings, NetworkInterface, Team};
use logger::FileLogger;

/// This struct represents the state that is sent to the UI.
#[serde_as]
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UiState {
    /// The current connection status of all players.
    connection_status: ConnectionStatusMap,
    /// The game state.
    game: Game,
    /// The mask of legal actions in the order they were subscribed.
    #[serde_as(as = "Vec<BoolFromInt>")]
    legal_actions: Vec<bool>,
    /// The list of the most recent actions that can be undone.
    undo_actions: Vec<VAction>,
}

/// This struct encapsulates state that must be mutated.
struct MutableState {
    /// The join set of main runtime tasks (event loop and logger).
    runtime_join_set: JoinSet<Result<()>>,
    /// The join set of network tasks.
    network_join_set: JoinSet<()>,
}

/// This struct represents the state that exposes GameController services to the UI.
pub struct RuntimeState {
    /// The sender for actions to the runtime.
    pub action_sender: mpsc::UnboundedSender<VAction>,
    /// The sender for subscribed actions of the UI.
    pub subscribed_actions_sender: watch::Sender<Vec<VAction>>,
    /// The notify object with which the UI tells the runtime thread that it can start its loop.
    pub ui_notify: Arc<Notify>,
    /// The combined parameters of the game and competition.
    pub params: Params,
    /// The sender for the shutdown signal.
    shutdown_token: CancellationToken,
    /// The mutable state behind a mutex. It is a tokio mutex because it is held across await.
    mutable_state: Mutex<MutableState>,
}

/// This function starts all network services that are not tied to a specific monitor. It returns a
/// receiver for incoming network events, a sender for the game state (that will be published to the
/// players) and a join set in which all tasks were spawned.
async fn start_network(
    initial_game: Game,
    params: Params,
    broadcast_address: IpAddr,
    local_address: IpAddr,
    multicast: bool,
    teams: Vec<u8>,
) -> Result<(
    mpsc::UnboundedReceiver<Event>,
    watch::Sender<Game>,
    JoinSet<()>,
)> {
    let (event_sender, event_receiver) = mpsc::unbounded_channel();
    let (control_sender, control_receiver) = watch::channel(initial_game);

    let mut join_set = JoinSet::new();

    if params.competition.league == League::Spl {
        for team in teams {
            let team_message_receiver =
                TeamMessageReceiver::new(local_address, multicast, team, event_sender.clone())
                    .await
                    .context("could not create team message receiver")?;
            join_set.spawn(async move { team_message_receiver.run().await.unwrap() });
        }
    }

    let control_message_sender =
        ControlMessageSender::new(broadcast_address, params, control_receiver, false)
            .await
            .context("could not create control message sender")?;

    join_set.spawn(async move { control_message_sender.run().await });

    let status_message_receiver = StatusMessageReceiver::new(local_address, event_sender.clone())
        .await
        .context("could not create status message receiver")?;
    join_set.spawn(async move { status_message_receiver.run().await.unwrap() });

    let referee_receiver = RefereeReceiver::new(local_address, event_sender.clone())
        .await
        .context("could not create referee receiver")?;
    join_set.spawn(async move { referee_receiver.run().await.unwrap() });

    let monitor_request_receiver = MonitorRequestReceiver::new(local_address, event_sender)
        .await
        .context("could not create monitor request receiver")?;
    join_set.spawn(async move { monitor_request_receiver.run().await.unwrap() });

    Ok((event_receiver, control_sender, join_set))
}

/// This function is the interfaces the GameController to external events. Each loop iteration
/// consists of three parts: First, the current state is sent to the UI and published for network
/// senders. Then, given the current state, a timestamp is calculated at which the next externally
/// visible state change happens. Finally, the next event is awaited, which can be either that the
/// previously calculated deadline was reached, an incoming network event, an action from the UI,
/// or a shutdown request.
#[allow(clippy::too_many_arguments)]
async fn event_loop(
    mut game_controller: GameController,
    mut event_receiver: mpsc::UnboundedReceiver<Event>,
    mut action_receiver: mpsc::UnboundedReceiver<VAction>,
    mut subscribed_actions_receiver: watch::Receiver<Vec<VAction>>,
    ui_notify: Arc<Notify>,
    shutdown_token: CancellationToken,
    control_sender: watch::Sender<Game>,
    send_ui_state: Box<dyn Fn(UiState) -> Result<()> + Send>,
) -> Result<()> {
    let mut last = Instant::now();
    let mut monitors = HashMap::<IpAddr, JoinSet<Result<()>>>::new();
    let mut players = HashSet::<IpAddr>::new();
    let mut aliveness_timestamps = AlivenessTimestampMap::new();
    let (status_forward_sender, _) = broadcast::channel(16);
    let (true_control_sender, _) = watch::channel(game_controller.get_game(false).clone());

    // We must wait for the main window before sending the first UI state.
    select! {
        _ = ui_notify.notified() => {},
        _ = shutdown_token.cancelled() => {
            return Ok(());
        }
    }

    loop {
        send_ui_state(UiState {
            connection_status: get_connection_status_map(&aliveness_timestamps, &last),
            game: game_controller.get_game(false).clone(),
            legal_actions: {
                let context = game_controller.get_context(false);
                subscribed_actions_receiver
                    .borrow_and_update()
                    .iter()
                    .map(|action| action.is_legal(&context))
                    .collect()
            },
            undo_actions: game_controller.get_undo_actions(5),
        })?;
        control_sender.send(game_controller.get_game(true).clone())?;
        let _ = true_control_sender.send(game_controller.get_game(false).clone());

        let next_connection_status_change =
            get_next_connection_status_change(&aliveness_timestamps, &last);
        let dt = game_controller
            .clip_next_timer_wrap(game_controller.clip_next_timer_expiration(
                next_connection_status_change.unwrap_or(Duration::MAX),
            ));

        let deadline = last.checked_add(dt);

        select! {
            // We can't use deadline.unwrap() because it's still evaluated even if the branch is
            // disabled. Therefore we supply some Instant that we have already lying around.
            _ = sleep_until(deadline.unwrap_or(last)), if deadline.is_some() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
            },
            event = event_receiver.recv() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
                // TODO: DANIEL CONNECTION VALIDATION
                match event {
                    Some(Event::MonitorRequest { host, data, .. }) => {
                        game_controller.log_now(LogEntry::MonitorRequest(LoggedMonitorRequest {
                            host,
                            data: data.to_vec(),
                        }));
                        // Requests are ignore if they come from hosts that have previously sent
                        // status messages, the monitor request is not ill-formed, or the host is
                        // already registered as a monitor.
                        if !players.contains(&host) && MonitorRequest::try_from(data).is_ok()
                        {
                            // If the host is already registered as monitor, cancel all tasks
                            // first. This is because the tasks can have crashed in the meantime
                            // (because the host disappeared) and they need to be restarted now.
                            if let Some(mut monitor_state) = monitors.remove(&host) {
                                monitor_state.abort_all();
                            }
                            let mut monitor_join_set = JoinSet::new();
                            {
                                let params = game_controller.params.clone();
                                let receiver = true_control_sender.subscribe();
                                monitor_join_set.spawn(async move {
                                    ControlMessageSender::new(
                                        host,
                                        params,
                                        receiver,
                                        true
                                    )
                                    .await
                                    .unwrap()
                                    .run()
                                    .await;
                                    Ok(())
                                });
                            }
                            {
                                let receiver = status_forward_sender.subscribe();
                                monitor_join_set.spawn(async move {
                                    StatusMessageForwarder::new(
                                        host,
                                        receiver,
                                    )
                                    .await
                                    .unwrap()
                                    .run()
                                    .await
                                });
                            }
                            monitors.insert(host, monitor_join_set);
                        }
                    },
                    Some(Event::StatusMessage { host, data, .. }) => {
                        game_controller.log_now(LogEntry::StatusMessage(LoggedStatusMessage {
                            host,
                            data: data.to_vec(),
                        }));
                        // If the host (which is now presumed to be a player) had previously sent a
                        // monitor request, it must not get any true data anymore.
                        if let Some(mut monitor_state) = monitors.remove(&host) {
                            monitor_state.abort_all();
                        }
                        players.insert(host);
                        // Status messages are forwarded to monitors even if they are ill-formed
                        // because then the monitor can display this fact. We must ignore errors
                        // here because it is possible that nobody is subscribed at the moment.
                        let _ = status_forward_sender.send((host, data.clone()));
                        // TODO: DANIEL
                        if game_controller.params.competition.league == League::Spl {
                            if let Ok(status_message) = StatusMessage::try_from(data) {
                                if let Some(side)
                                    = game_controller.params.game.get_side(status_message.team_number)
                                {
                                    aliveness_timestamps.insert(
                                        (side, PlayerNumber::new(status_message.player_number)),
                                        now,
                                    );
                                }
                            }
                        } else {
                            if let Ok(status_message) = HlStatusMessage::try_from(data) {
                                if let Some(side)
                                    = game_controller.params.game.get_side(status_message.team_number)
                                {
                                    aliveness_timestamps.insert(
                                        (side, PlayerNumber::new(status_message.player_number)),
                                        now,
                                    );
                                    if status_message.message == 3 
                                    {
                                        game_controller.apply(VAction::HlSetGoalkeeper(HlSetGoalkeeper {
                                            side,
                                            player: PlayerNumber::new(status_message.player_number),
                                        }), ActionSource::Network);
                                    }
                                }
                            }
                        }
                    },
                    Some(Event::TeamMessage { host, team, data, too_long }) => {
                        game_controller.log_now(LogEntry::TeamMessage(LoggedTeamMessage {
                            team,
                            host,
                            data: data.to_vec(),
                        }));
                        game_controller.apply(VAction::TeamMessage(TeamMessage {
                            // We only started a team message receiver for the two playing teams,
                            // so the unwrap is justified.
                            side: game_controller.params.game.get_side(team).unwrap(),
                            illegal: too_long,
                        }), ActionSource::Network);
                    },
                    Some(Event::RefereeMessage { data }) => {
                        println!("Referee Message {:?}", data);
                        if let Ok(referee_message) = RefereeMessage::try_from(data) {
                            game_controller.automated_referee(referee_message.command_1, referee_message.command_2, referee_message.command_3, referee_message.command_4, referee_message.command_5);
                        }
                    },
                    _ => {},
                }
            },
            action = action_receiver.recv() => {
                let now = Instant::now();
                game_controller.seek(now - last);
                last = now;
                if let Some(action) = action {
                    game_controller.apply(action, ActionSource::User);
                }
            },
            _ = subscribed_actions_receiver.changed() => {},
            _ = shutdown_token.cancelled() => {
                for mut monitor_state in monitors.into_values() {
                    monitor_state.shutdown().await;
                }
                // This last seek is done so that the end timestamp in the log is more accurate
                // (the end entry is added when the GameController is dropped).
                game_controller.seek(Instant::now() - last);
                return Ok(());
            },
        };
    }
}

/// This function starts all runtime tasks.
pub async fn start_runtime(
    config_directory: &Path,
    log_directory: &Path,
    settings: &LaunchSettings,
    teams: &[Team],
    network_interfaces: &[NetworkInterface],
    send_ui_state: Box<dyn Fn(UiState) -> Result<()> + Send>,
) -> Result<RuntimeState> {
    let mut runtime_join_set = JoinSet::new();

    // If we should start by replaying a log file, it is loaded into memory now. This should
    // definitely not be done in an async function...
    let replay_data = settings
        .log
        .replay
        .as_ref()
        .map(|path| {
            serde_yaml::from_reader::<_, Vec<TimestampedLogEntry>>(
                File::open(path).context("could not open log file to be replayed")?,
            )
            .context("could not parse log file to be replayed")
        })
        .map_or(Ok(None), |r: Result<_>| r.map(Some))?;

    // If the first entry in the log file to be replayed is metadata, then we use the parameters
    // from there. Otherwise (or if we're not replaying a log file at all), the parameters are
    // loaded from the settings supplied by the user.
    let params = replay_data
        .as_ref()
        .and_then(|data| data.first())
        .and_then(|entry| match &entry.entry {
            LogEntry::Metadata(metadata) => Some(Ok::<Params, Error>(*metadata.params.clone())),
            _ => None,
        })
        .unwrap_or_else(|| {
            Ok(Params {
                competition: serde_yaml::from_reader(
                    File::open(
                        config_directory
                            .join(match settings.league.league {
                                League::Spl => "spl",
                                League::Humanoid => "humanoid",
                            })
                            .join(&settings.competition.id)
                            .join("params.yaml"),
                    )
                    .context("could not open competition params")?,
                )
                .context("could not parse competition params")?,
                game: settings.game.clone(),
            })
        })?;

    let team_names = params.game.teams.clone().map(|_side, team| {
        teams
            .iter()
            .find(|t| team.number == t.number)
            .unwrap()
            .name
            .replace(' ', "-")
    });
    let date_time = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    create_dir_all(log_directory)
        .await
        .context("could not create log directory")?;
    let logger = FileLogger::new(
        log_directory.join(format!(
            "log_{}_{}_{}.yaml",
            date_time
                .format(format_description!(
                    "[year]-[month]-[day]_[hour]-[minute]-[second]"
                ))
                .context("could not create log file name")?,
            team_names[Side::Home],
            team_names[Side::Away],
        )),
        &mut runtime_join_set,
        settings.log.sync,
    )
    .await
    .context("could not create logger")?;

    let mut game_controller = GameController::new(params.clone(), Box::new(logger));

    game_controller.log_now(LogEntry::Metadata(LoggedMetadata {
        creator: "GameController".into(),
        version: 1,
        timestamp: date_time,
        params: Box::new(params.clone()),
    }));

    // Replay log entries. This should definitely not be done in an async function...
    if let Some(data) = replay_data {
        let mut iter = data.into_iter().peekable();
        // Skip metadata if present and take its timestamp as start (even though it is always zero
        // if written by the GameController).
        let mut last_timestamp = iter
            .next_if(|entry| matches!(entry.entry, LogEntry::Metadata(_)))
            .map_or(Duration::ZERO, |entry| entry.timestamp);
        for entry in iter {
            game_controller.seek(entry.timestamp - last_timestamp);
            last_timestamp = entry.timestamp;
            match entry.entry {
                LogEntry::Action(action) => {
                    if action.source != ActionSource::Timer {
                        game_controller.apply(action.action, action.source);
                    }
                }
                LogEntry::End => {}
                // We could check that the logged state matches the current, but because of timer
                // actions this is not that simple.
                LogEntry::GameState(_) => {}
                LogEntry::Metadata(_) => panic!("metadata can only occur as first entry in a log"),
                _ => {
                    game_controller.log_now(entry.entry);
                }
            };
        }
    }

    let network_interface = network_interfaces
        .iter()
        .find(|network_interface| network_interface.id == settings.network.interface)
        .unwrap();

    let (event_receiver, control_sender, network_join_set) = {
        let game = game_controller.get_game(true).clone();
        let params = game_controller.params.clone();
        start_network(
            game,
            params,
            if settings.network.broadcast {
                IpAddr::V4(Ipv4Addr::BROADCAST)
            } else {
                network_interface.broadcast
            },
            network_interface.address,
            settings.network.multicast,
            settings
                .game
                .teams
                .values()
                .map(|team| team.number)
                .collect(),
        )
        .await
        .context("could not start network services")?
    };

    let (action_sender, action_receiver) = mpsc::unbounded_channel();
    let (subscribed_actions_sender, subscribed_actions_receiver) = watch::channel(vec![]);
    let ui_notify = Arc::new(Notify::new());
    let shutdown_token = CancellationToken::new();

    runtime_join_set.spawn(event_loop(
        game_controller,
        event_receiver,
        action_receiver,
        subscribed_actions_receiver,
        ui_notify.clone(),
        shutdown_token.clone(),
        control_sender,
        send_ui_state,
    ));

    Ok(RuntimeState {
        action_sender,
        subscribed_actions_sender,
        ui_notify,
        params,
        shutdown_token,
        mutable_state: Mutex::new(MutableState {
            runtime_join_set,
            network_join_set,
        }),
    })
}

/// This function tells the runtime to shut down and waits until all tasks have finished.
pub async fn shutdown_runtime(runtime_state: &RuntimeState) {
    runtime_state.shutdown_token.cancel();

    let mut mutable_state = runtime_state.mutable_state.lock().await;

    // There are two tasks in this join set. If there have not been any errors during logging, the
    // event loop will finish first because the logger still waits on the channel. But once the
    // event loop returns, the channel is dropped, and after writing the last log entry, the logger
    // will return, too.
    while mutable_state.runtime_join_set.join_next().await.is_some() {}

    mutable_state.network_join_set.shutdown().await;
}
