# GameController

This is the GameController software for robot soccer games in the RoboCup Humanoid League.

## Compilation

### Prerequisites

- Rust and other platform-specific tauri dependencies as [listed here](https://tauri.app/v1/guides/getting-started/prerequisites)
- nodejs and npm (or a compatible package manager)
- libclang (for bindgen)
    - On Windows:
    ```powershell
    winget install LLVM
    set LIBCLANG_PATH 'C:\Program Files\LLVM\bin\'
    ```

### Commands

First, the frontend must be compiled:

```bash
cd frontend
npm ci
npm run build
```

The Rust code is compiled with cargo:

```bash
cargo build [-r]
```

#### Dev

First, thank you for contributing to development.

This will install the tauri CLI:

```bash
cargo install tauri-cli
```

This will start the frontend and backend in dev mode:

```bash
cargo tauri dev
```

### Creating Binary Distributions

For Linux, macOS and Windows, there are scripts in the directory `dist/` to create binary distributions.
They all expect a version number as first argument, and can optionally take the target as second argument (otherwise the host target is assumed on Linux and Windows, while macOS creates a Universal binary).
They build a special profile called `release-dist` which mainly tries to create a small binary.
It is recommended to run the script only on a clean working tree.

## Configuration

Configuration files that are read at runtime are located in the directory `config`.
The global `teams.yaml` is a list of all teams in the SPL, their names, and their available jersey colors both for field players and goalkeepers.
Each (sub)competition has a subdirectory with two files:
`params.yaml` corresponds to the Rust struct `game_controller_core::types::CompetitionParams` and sets (mostly numeric) constants for the competition.
`teams.yaml` is a list of team numbers of the teams that participate in the competition.
Only those teams can be selected when playing in this competition.
Therefore, for a new team to appear in the UI, an entry must be added both to the global `teams.yaml` (with an unused team number) and in the competition's `teams.yaml` (referencing the team number).

## Network Communication

Currently, all network communication with the GameController uses IPv4, although most parts of the code can also handle IPv6.

The GameController communicates with robot players via three channels:
- It sends control messages at a rate of 2 hertz (UDP broadcast on port 3838, format specified in the struct `RoboCupGameControlData` in `game_controller_msgs/headers/RoboCupGameControlData.h`).
    These control messages do not always represent the true game state, specifically after a goal or a transition to the `playing` state.
    After these events, they continue to maintain the state before the event for up to 15 seconds, or until another event happens that could not have happened in this "fake" state.
    Note that this behavior differs from the old GameController, which would always keep the state attribute (and some others) at the old value for 15 seconds, even when other attributes already clearly indicated that it was the new state (e.g. players are unpenalized although their timers aren't at zero yet, or set plays starting during the "fake" `set` state when it is actually already `playing`).
- It receives status messages from the robot players which must send them at a rate between 0.5 hertz and 2 hertz (UDP unicast on port 3939, format specified in the struct `RoboCupGameControlReturnData` in `game_controller_msgs/headers/RoboCupGameControlData.h`).
- It receives team messages from the robot players (UDP broadcast on port 10000 + team number, up to 128 bytes of payload with arbitrary format).

In addition, the GameController offers an interface for monitor applications (such as the TeamCommunicationMonitor or the EventRecorder):
- It receives monitor requests (UDP unicast on port 3636, 4 bytes header magic `RGTr` + 1 byte version number `0`).
    It refuses to accept monitor requests from hosts that have previously sent status messages, as those are presumed to be robot players which should not get true data.
    Similarly, if a host that had previously sent a monitor request sends a status message, it will not receive monitor data anymore.
- Each registered monitor host will get:
    - control messages with the true game state at a rate of 2 hertz (UDP unicast on port 3838, with the same format as regular control messages, but with the header magic `RGTD`).
    - forwarded status messages (UDP unicast on port 3940, prefixed by the IPv4 address of the original sender).
        The forwarded payload has not been validated.

The user must ensure that all of the aforementioned network communication channels are allowed to be used by the firewall.
The GameController runs on a specific network interface, which generally specifies where packets are sent and from where they are received.
The exceptions are that control messages can be configured to be sent to the limited broadcast address (`255.255.255.255`) instead of the interface's broadcast address, and that monitor requests and team messages are received from any address.

## Usage

### Start

The binary distributions on the [GitHub releases page](https://github.com/RoboCup-Humanoid-TC/GameController/releases) come with scripts that can be executed in a platform-typical way.
On macOS, you may want to call `xattr -c <path to GameController.app>` before the first run to clear the quarantine flag.

If the GameController should be run from the source code, the most convenient way to do it is by executing

```bash
cargo run [-r]
```

from a command line within any directory of this workspace.
The program accepts command line arguments which can be passed to `cargo` after `--`.
They override the defaults of the launcher.
A list of arguments (that is always up to date, in contrast to what would be written here) can be obtained by running with the `-h` option:

```bash
cargo run -- -h
```

Note that release builds on Windows do not output any text.

### Launcher

When the GameController is started, a launcher is shown first.
Some fields will be pre-filled by command line arguments, if they were specified.

The following settings are exposed via the launcher:
- Competition: This box selects the competition type of the game. It influences the behavior and constants of the GameController and narrows down the set of teams that can be selected.
- Play-off: This checkbox selects if the game time stops during all Ready and Set states. Should be checked if the game is a quarterfinal, semifinal, final or 3rd place game.
- Teams:
    - Kick-off for (home / away) team: This box selects which team has the first kick-off, as a result of the coin tosses before the game.
    - The main box selects the team on the respective side.
    - Field player color and goalkeeper color: These boxes select the jersey colors of the team.
- Mirror: This checkbox selects if the home (first on the schedule) team defends the right side (from the GameController's perspective) instead of the left side, as a result of the coin tosses before the game.
- Fullscreen: This checkbox selects if the window should be switched to fullscreen mode when started.
- Interface: This box selects the network interface to run on (see [above](#network-communication)). Not all interfaces that are listed will necessarily work.
- Broadcast: This checkbox selects if control messages are sent to the limited broadcast address (`255.255.255.255`) instead of the interface's broadcast address. Should only be used when it is required that those messages are sent on all interfaces.
- Multicast: This checkbox selects whether team communication is also received from a certain multicast group. Should only be used for simulated games, *never* for real competition games.

The launcher allows to start a game only if the two teams are distinct and their jersey colors don't conflict, i.e. all four colors must be pairwise distinct, except for the goalkeeper colors which may be the same for both teams.
Note that changing the sides or the kick-off team is not possible afterwards, so the switch to the main interface can only be done after the coin tosses.

### Main Interface

#### Substitution

To substitute players, first click the "Substitute" button on the team's side.
Then, click the player which should be removed from play.
The list of players will now change to the list of available substitutes (note that this list is scrollable).
From this list, the player that shall replace the previously selected player is clicked.
Depending on the game state, the new player gets a penalty or inherits the penalty of the substituted player.
The goalkeeper property is tranferred to the substitute, i.e. when the goalkeeper is substituted, the substitute is expected to wear a goalkeeper jersey and inherits the privileges of the goalkeeper.

This feature must also be used before the start of a half in order to match the set of players in the GameController to the players that are actually on the field.
If a team wants to play with a goalkeeper with a number from 2-7 (or 2-5 in the Challenge Shield), this must be done using three substitutions (e.g. if a team wants to play with players 1-7, but have the 3 be the goalkeeper, it must substitute 8 for 1, 1 for 3, 3 for 8).

#### Penalty Shoot-out

A penalty shoot-out can only be started after two halves have been played and the score is equal.
However, it does not depend on the game mode, because penalty shoot-outs can already be needed in games in which the clock does not stop during the Ready and Set states.
It is only allowed to switch to the next shot as long as the result of the game is not clear yet.

The Playing state can be entered once players on both sides are selected.
This is done by clicking the "Select" button (in place of the "Substitute" button).
First, the color of the selected player must be selected.
This is because in a penalty shoot-out, the jersey color does not necessarily match the designation as goalkeeper or kicker, and the GameController cannot know which players are wearing which jersey.
After the color has been selected, a scrollable list of players appears (similar to substitution).

#### Undo

At the bottom of the window, a timeline of the last five actions applied by the user is filled from right to left.
Clicking on one of the actions there restores the entire game to the state immediately before that action was applied.
It is not possible to undo individual actions that have been followed by other actions.
The undo history is actually not limited to the last five actions, i.e. previous actions appear once some actions are undone.

Actions that were applied automatically (e.g. because a timer elapsed) do not appear in the undo history.
This is because they would be applied again immediately if they were undone.

## Logs

The GameController writes log files to the directory `logs`.
They can get quite large because they are YAML.
The main reason for YAML is that it is human-readable and can be appended (in contrast to JSON which requires a closing bracket in the end to be well-formed).

At the moment, there are no tools to process these log files, but eventually, the tools from the old GameController should be ported.

# Automated Referee Interface

There is a basic interface to communicate with the GameController for the automated referee.

## Structure


For the communication with the GameController the programmer needs 5 bytes:

| Global | Team | Player | Playernumber | Side |

## Current methods

### Global

1 State initial to Ready
2 State Ready to Set
3 State Set to Playing
4 State Playing to Finished
5 State Finished to Initial (second phase)
6 State Timeout
7 Add Extra time

### Team

1 Goal kick
2 Throw in
3 Corner kick
4 Penalty kick
5 Direct free kick
6 Indirect free kick
7 Retake
8 Abort
9 Goal

### Player

1 Substitute
2 Pushing
3 Pick up
4 Ball manipulation
5 Unpenalize
6 Red card
7 Yellow card
8 Warning

### Playernumber

The number of the player (e.g. 3)

### Side

0 Home
1 Away

