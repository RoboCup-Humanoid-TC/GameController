//! This module defines the launcher backend of the GameController application.

use std::{
    fs::File,
    net::IpAddr,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Context, Result};
use enum_map::enum_map;
use network_interface::NetworkInterfaceConfig;
use serde::{Deserialize, Serialize};

use game_controller_core::types::{
    Color, CompetitionParams, GameParams, League, Side, SideMapping, TeamParams,
};

use crate::cli::Args;

/// This struct describes a single entry in `config/teams.yaml`.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The number of the team. This is a (more or less) unique identifier across competitions.
    pub number: u8,
    /// The given name of the team.
    pub name: String,
    /// The list of jersey colors that this team can use for field players.
    pub field_player_colors: Vec<Color>,
    /// The list of jersey colors that this team can use for the goalkeeper.
    pub goalkeeper_colors: Vec<Color>,
}

/// This struct describes a competition (a subdirectory in `config`).
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    /// The machine-readable name of the competition. This is the name of the competition's
    /// subdirectory in `config`.
    pub id: String,
    /// The "pretty" name of the competition (taken from `config/<id>/params.yaml`).
    pub name: String,
    /// The list of teams (identified by their number) that participate in this competition.
    pub teams: Vec<u8>,
}

/// This struct describes a network interface.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkInterface {
    /// The system-specific name of the network interface.
    pub id: String,
    /// The local address of the network interface.
    pub address: IpAddr,
    /// The broadcast address of the network interface.
    pub broadcast: IpAddr,
}

/// This struct describes settings related to the competition type.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionSettings {
    /// The ID of the competition (must match some [Competition::id]).
    pub id: String,
}

/// This struct describes settings of the game.
pub type GameSettings = GameParams;

/// This struct describes settings related to the main window.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowSettings {
    /// Whether the main window should be started in fullscreen mode.
    pub fullscreen: bool,
}

/// This struct describes settings for the network.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkSettings {
    /// The name of the network interface on which to run network services.
    pub interface: String,
    /// Whether the limited broadcast address (255.255.255.255) should be used.
    pub broadcast: bool,
    /// Whether multicast groups should be joined to listen for simulated team communication.
    pub multicast: bool,
}

/// This struct describes settings for logging.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSettings {
    /// Whether the log file should be synced to the storage device after each entry.
    pub sync: bool,
    /// The path to a log file that should be replayed.
    pub replay: Option<PathBuf>,
}

/// This struct describes settings for logging.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueSettings {
    /// Humanoid or SPL
    pub league: League,
}

/// This represents the overall settings that can be configured in the launcher.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchSettings {
    /// Settings related to the competition type.
    pub competition: CompetitionSettings,
    /// Settings of the game.
    pub game: GameSettings,
    /// Settings related to the main window.
    pub window: WindowSettings,
    /// Settings for the network.
    pub network: NetworkSettings,
    /// Settings for logging.
    pub log: LogSettings,
    /// Settings of the league
    pub league: LeagueSettings,
}

/// The bundle of data that is passed to JavaScript.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchData {
    /// The list of selectable competitions.
    pub competitions: Vec<Competition>,
    /// The list of available teams (but only those in a competition can be selected).
    pub teams: Vec<Team>,
    /// The list of selectable network interfaces.
    pub network_interfaces: Vec<NetworkInterface>,
    /// The initial settings to be modified by the user.
    pub default_settings: LaunchSettings,
}

/// This function creates a list of competitions from the subdirectories of `config/<league>`.
/// The files `params.yaml` and `teams.yaml` must exist within a subdirectory to consider it.
fn get_competitions(config_directory: &Path) -> Result<Vec<Competition>> {
    let mut result: Vec<Competition> = std::fs::read_dir(config_directory)
        .context("could not open config directory")?
        .map(|entry| {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                return Ok(None);
            }

            let params_path = entry.path().join("params.yaml");
            let teams_path = entry.path().join("teams.yaml");
            if !params_path.try_exists()? || !teams_path.try_exists()? {
                return Ok(None);
            }
            let params: CompetitionParams = serde_yaml::from_reader(
                File::open(params_path).context("could not open competition params")?,
            )
            .context("could not parse competition params")?;
            let teams: Vec<u8> = serde_yaml::from_reader(
                File::open(teams_path).context("could not open competition teams")?,
            )
            .context("could not parse competition teams")?;
            Ok(Some(Competition {
                id: entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                name: params.name,
                teams,
            }))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();
    result.sort_by(|c1, c2| c1.name.cmp(&c2.name));
    Ok(result)
}

/// This function reads all teams from `config/<league>/teams.yaml`.
fn get_teams(config_directory: &Path) -> Result<Vec<Team>> {
    serde_yaml::from_reader(
        File::open(config_directory.join("teams.yaml")).context("could not open teams config")?,
    )
    .context("could not parse teams config")
}

/// This function returns the list of available network interfaces with a configured IPv4 address.
/// It can currently not handle network interfaces with multiple addresses (only the first IPv4
/// address is used in that case).
fn get_network_interfaces() -> Result<Vec<NetworkInterface>> {
    let mut result: Vec<NetworkInterface> = network_interface::NetworkInterface::show()
        .context("could not enumerate network interfaces")?
        .into_iter()
        .filter_map(|interface| {
            interface
                .addr
                .iter()
                .find(|addr| addr.ip().is_ipv4())
                .map(|addr| NetworkInterface {
                    id: interface.name,
                    address: addr.ip(),
                    broadcast: addr.broadcast().unwrap_or(addr.ip()),
                })
        })
        .collect();
    result.sort_by(|i1, i2| i1.id.cmp(&i2.id));
    Ok(result)
}

/// This function creates [LaunchData] from a path to the `config` directory and a map of command
/// line arguments that can initialize certain values of the default settings.
pub fn make_launch_data(config_directory: &Path, args: Args) -> Result<LaunchData> {
    let league = if args.league {
        League::Spl
    } else {
        League::Humanoid
    };
    let league_config_directory =
        config_directory.join(if league == League::Spl { "spl" } else { "humanoid" });
    let teams = get_teams(&league_config_directory).context("could not read teams")?;
    if teams.is_empty() {
        bail!("there are no teams");
    }
    if league == League::Spl {
        if teams.iter().any(|team| team.field_player_colors.len() < 2) {
            bail!("not all teams have at least two field player colors");
        }
        if teams.iter().any(|team| team.goalkeeper_colors.len() < 2) {
            bail!("not all teams have at least two goalkeeper colors");
        }
    } else {
        if teams.iter().any(|team| team.field_player_colors.len() != 2) {
            bail!("All teams have to have 2 colors (Red, Blue)");
        }
    }
    // TODO: check that all team numbers are pairwise distinct
    // TODO: check that all team colors are pairwise distinct
    // TODO: check that all team names are pairwise distinct (?)

    let default_team = teams
        .iter()
        .find(|team| team.number == 0)
        .context("could not find the default team")?;

    let competitions =
        get_competitions(&league_config_directory).context("could not read competitions")?;
    if competitions.is_empty() {
        bail!("there are no competitions");
    }
    if competitions
        .iter()
        .any(|competition| !competition.teams.contains(&default_team.number))
    {
        bail!("not all competitions contain the default team");
    }
    if competitions.iter().any(|competition| {
        competition
            .teams
            .iter()
            .any(|number| !teams.iter().any(|team| team.number == *number))
    }) {
        bail!("some competition references a team number that does not exist");
    }
    // TODO: check that competition names are pairwise distinct (?)
    // TODO: check that no competition has duplicate team numbers

    let network_interfaces =
        get_network_interfaces().context("could not get network interfaces")?;
    if network_interfaces.is_empty() {
        bail!("there are no network interfaces");
    }

    let competition_id = if let Some(id) = args.competition {
        if !competitions.iter().any(|competition| competition.id == id) {
            let competition_ids = competitions
                .iter()
                .map(|competition| competition.id.clone())
                .collect::<Vec<String>>();
            bail!("unknown competition type {id}. possible values are: {competition_ids:?}");
        }
        id
    } else {
        competitions[0].id.clone()
    };

    let parse_team = |arg: Option<&String>| {
        if let Some(team_id) = arg {
            let team = match team_id.parse::<u8>() {
                Ok(number) => teams.iter().find(|team| team.number == number),
                _ => teams.iter().find(|team| &team.name == team_id),
            }
            .ok_or(anyhow!("unknown team: {team_id}"))?;
            if !competitions
                .iter()
                .find(|competition| competition.id == competition_id)
                .unwrap()
                .teams
                .contains(&team.number)
            {
                bail!("{} is not part of the selected competition", team.name);
            }
            Ok(Some(TeamParams {
                number: team.number,
                field_player_color: team.field_player_colors[0],
                goalkeeper_color: team.goalkeeper_colors[0],
            }))
        } else {
            Ok(None)
        }
    };

    let default_settings = LaunchSettings {
        competition: CompetitionSettings {
            // competition_id cannot be moved because it is still referenced by parse_team.
            id: competition_id.clone(),
        },
        game: GameSettings {
            teams: enum_map! {
                Side::Home => parse_team(args.home_team.as_ref())
                    .context("could not set home team")?
                    .unwrap_or(TeamParams
                {
                    number: default_team.number,
                    field_player_color: default_team.field_player_colors[0],
                    goalkeeper_color: default_team.goalkeeper_colors[0],
                }),
                Side::Away => parse_team(args.away_team.as_ref())
                    .context("could not set away team")?
                    .unwrap_or(TeamParams
                {
                    number: default_team.number,
                    field_player_color: default_team.field_player_colors[1],
                    goalkeeper_color: default_team.goalkeeper_colors[1],
                }),
            },
            long: args.play_off,
            kick_off_side: Side::Home,
            side_mapping: SideMapping::HomeDefendsLeftGoal,
        },
        window: WindowSettings {
            fullscreen: args.fullscreen,
        },
        network: NetworkSettings {
            interface: {
                if let Some(id) = args.interface {
                    if !network_interfaces
                        .iter()
                        .any(|network_interface| network_interface.id == id)
                    {
                        let network_interface_ids = network_interfaces
                            .iter()
                            .map(|network_interface| network_interface.id.clone())
                            .collect::<Vec<String>>();
                        bail!("unknown network interface {id}. possible values are: {network_interface_ids:?}");
                    }
                    id
                } else {
                    network_interfaces
                        .iter()
                        .find(|interface| {
                            interface.broadcast == IpAddr::from([10u8, 0u8, 255u8, 255u8])
                        })
                        .unwrap_or(&network_interfaces[0])
                        .id
                        .clone()
                }
            },
            broadcast: args.broadcast,
            multicast: args.multicast,
        },
        log: LogSettings {
            sync: args.sync,
            replay: args.replay,
        },
        league: LeagueSettings { league },
    };

    Ok(LaunchData {
        competitions,
        teams,
        network_interfaces,
        default_settings,
    })
}
