//! This module defines types which constitute the state of a game. It is quite SPL-specific.

use std::{
    ops::{Index, IndexMut, Neg},
    time::Duration,
};

use enum_map::{Enum, EnumMap};
use serde::{Deserialize, Serialize};

use crate::timer::Timer;

/// This enumerates the special GameController modes for technical challenges.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChallengeMode {
    /// DropIn
    DropIn,
    /// KidSize
    KidSize,
    /// AdultSize
    AdultSize,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum League {
    /// SPL League
    Spl,
    /// Humanoid League
    Humanoid,
}

/// This struct contains constant parameters of a penalty type.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PenaltyParams {
    /// The base duration of the penalty.
    pub duration: Duration,
    /// Whether this penalty increases its duration with each previous incremental penalty of a
    /// team.
    pub incremental: bool,
}

/// This struct contains constant parameters of a set play type.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPlayParams {
    /// The duration of the (restricted) Playing state of this set play.
    pub duration: Duration,
    /// The duration of the Ready state of this set play (or 0 this set play doesn't have a Ready
    /// state).
    pub ready_duration: Duration,
}

/// This struct contains constant parameters of a (sub)competition.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompetitionParams {
    /// A "pretty" version of the competition's name.
    pub name: String,
    /// The mode (if this is a technical challenge).
    pub challenge_mode: Option<ChallengeMode>,
    /// The number of players per team that can play at the same time.
    pub players_per_team: u8,
    /// The parameters of each penalty type.
    pub penalties: EnumMap<Penalty, PenaltyParams>,
    /// The additional penalty duration for each previous incremental penalty of a team.
    pub penalty_duration_increment: Duration,
    /// The parameters of each set play type.
    pub set_plays: EnumMap<SetPlay, SetPlayParams>,
    /// The duration of each half.
    pub half_duration: Duration,
    /// The duration of the half-time break.
    pub half_time_break_duration: Duration,
    /// The duration of a timeout taken by a team.
    pub timeout_duration: Duration,
    /// The number of timeouts a team can take during a game.
    pub timeouts_per_team: u8,
    /// The duration of a referee timeout.
    pub referee_timeout_duration: Duration,
    /// The number of team messages a team can send during a game.
    pub messages_per_team: u16,
    /// The number of team messages by which a team's budget is increased per minute of extra time.
    pub messages_per_team_per_extra_minute: u16,
    /// The score difference at which a game is finished automatically.
    pub mercy_rule_score_difference: u8,
    /// The number of regular penalty shots each team takes in a penalty shoot-out.
    pub penalty_shots: u8,
    /// The number of sudden death penalty shots that each team can take before a coin is tossed.
    pub sudden_death_penalty_shots: u8,
    /// The duration of a penalty kick in a penalty shoot-out.
    pub penalty_shot_duration: Duration,
    /// The duration for which the true game state is hidden after a goal.
    pub delay_after_goal: Duration,
    /// The duration for which the true game state is hidden after switching to the Playing state.
    pub delay_after_playing: Duration,
    /// league for the config
    pub league: League,
}

/// This struct contains constant parameters for one team.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamParams {
    /// The number which globally identifies the team within the league.
    pub number: u8,
    /// The jersey color of the field players.
    pub field_player_color: Color,
    /// The jersey color of the goalkeeper.
    pub goalkeeper_color: Color,
}

/// This struct contains constant parameters that are specific to a game.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameParams {
    /// The team parameters for both teams.
    pub teams: EnumMap<Side, TeamParams>,
    /// If true, the time during Ready/Set states is not counted as part of the duration of a half.
    /// Otherwise (if false), all Ready/Set states except for the time before the first kick-off in
    /// a half count as playing time.
    pub long: bool,
    /// The side which has kick-off in the first half.
    pub kick_off_side: Side,
    /// The side mapping for the first half.
    pub side_mapping: SideMapping,
}

impl GameParams {
    /// This function returns the side on which a given team number is playing or [None] if the
    /// team is not one of the playing teams.
    pub fn get_side(&self, team_number: u8) -> Option<Side> {
        if team_number == self.teams[Side::Home].number {
            Some(Side::Home)
        } else if team_number == self.teams[Side::Away].number {
            Some(Side::Away)
        } else {
            None
        }
    }
}

/// This struct contains the combined parameters of the competition in general and the current game.
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    /// The parameters of the competition.
    pub competition: CompetitionParams,
    /// The parameters of the game.
    pub game: GameParams,
}

/// This enumerates the phases in which a game can be.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    /// The first half of the regular playing time.
    FirstHalf,
    /// The second half of the regular playing time.
    SecondHalf,
    /// A penalty shoot-out to decide the outcome of a game.
    PenaltyShootout,
}

/// This enumerates the states in which a game can be.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum State {
    /// This state is active before each half and before a penalty shoot-out.
    Initial,
    /// This state is active when certain set plays are set up.
    Ready,
    /// This state is active after certain set plays have been set up and before each penalty shot.
    Set,
    /// This state is active during normal play (a set play can be going on).
    Playing,
    /// This state is active after each half and each penalty shot.
    Finished,
    /// This state is active during a timeout (either for a team or by the referee).
    Timeout,
}

/// This enumerates the second state that the can have (HL)
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SecState {
    /// normal
    Normal,
    /// Penaltyshoot
    Penaltyshoot,
    /// Overtime
    Overtime,
    /// Timeout
    Timeout,
    /// Direct Freekick
    DirectFreekick,
    /// Indirect Freekick
    IndirectFreekick,
    /// Penaltykick
    Penaltykick,
    /// Cornerkick
    CornerKick,
    /// Goal Kick
    GoalKick,
    /// Throw in
    ThrowIn,
}

/// This enumerates the set plays which can be active.
#[derive(Clone, Copy, Debug, Deserialize, Enum, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SetPlay {
    /// No set play is active.
    NoSetPlay,
    /// A kick-off is in progress.
    KickOff,
    /// A kick-in is in progress.
    KickIn,
    /// A goal kick is in progress.
    GoalKick,
    /// A corner kick is in progress.
    CornerKick,
    /// A pushing free kick is in progress.
    PushingFreeKick,
    /// A penalty kick is in progress.
    PenaltyKick,
}

/// This enumerates the jersey colors. Values may be added to match actually submitted jersey designs.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Color {
    Red,
    Blue,
    Yellow,
    Black,
    White,
    Green,
    Orange,
    Purple,
    Brown,
    Gray,
}

/// This enumerates the reasons why a player can be penalized.
#[derive(Clone, Copy, Debug, Deserialize, Enum, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Penalty {
    /// The player is not penalized.
    NoPenalty,
    /// The player is currently a substitute.
    Substitute,
    /// The player has been picked up by the team captain.
    PickedUp,
    /// The player has been illegally positioned when the game state switched to the Set state.
    IllegalPositionInSet,
    /// The player has moved to an illegal position.
    IllegalPosition,
    /// The player has moved during the Set state.
    MotionInSet,
    /// The player has fallen or become inactive for too long.
    FallenInactive,
    /// The player has caused a local game stuck.
    LocalGameStuck,
    /// The player has committed a ball holding offence.
    BallHolding,
    /// The player has stayed in a wide stance for too long.
    PlayerStance,
    /// The player has committed a pushing offence.
    PlayerPushing,
    /// The player has illegally played the ball with its arms or hands.
    PlayingWithArmsHands,
    /// The player has tried to leave the field.
    LeavingTheField,
}

/// This enumerates the possible referee calls for penalties. They mostly correspond to [Penalty],
/// but there are some calls that map to different penalties in different states
/// ([PenaltyCall::IllegalPosition]) and there are calls that map to the same penalty but with
/// different side effects ([PenaltyCall::Foul], [PenaltyCall::PenaltyKick]).
#[derive(Clone, Copy, Debug, Deserialize, Enum, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PenaltyCall {
    RequestForPickUp,
    IllegalPosition,
    MotionInSet,
    FallenInactive,
    LocalGameStuck,
    BallHolding,
    PlayerStance,
    Pushing,
    Foul,
    PenaltyKick,
    PlayingWithArmsHands,
    LeavingTheField,
}

/// This enumerates the two opposing teams of the game. The name `Side` may be slightly misleading
/// since it doesn't refer to the side of the field of play, but the order of teams in the
/// schedule. A team never changes its home/away designation during a game which is useful for
/// interpreting log files. The actual sides on the field of play are represented by [SideMapping].
#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Side {
    /// The team listed first on the schedule.
    Home,
    /// The team listed second on the schedule.
    Away,
}

impl Neg for Side {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Home => Self::Away,
            Self::Away => Self::Home,
        }
    }
}

/// This enumerates the possible assignments of teams to sides of the field of play.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SideMapping {
    /// The home team defends the left goal (and the away team defends the right goal).
    HomeDefendsLeftGoal,
    /// The home team defends the right goal (and the away team defends the left goal).
    HomeDefendsRightGoal,
}

impl Neg for SideMapping {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::HomeDefendsLeftGoal => Self::HomeDefendsRightGoal,
            Self::HomeDefendsRightGoal => Self::HomeDefendsLeftGoal,
        }
    }
}

/// This is a structure that wraps a player number.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlayerNumber(pub u8);

impl PlayerNumber {
    /// The lowest possible player number.
    pub const MIN: u8 = 1;

    /// The highest possible player number.
    pub const MAX: u8 = 20;

    /// This function creates a new player number while checking that it is within the allowed
    /// range of player numbers.
    pub fn new(number: u8) -> Self {
        assert!((Self::MIN..=Self::MAX).contains(&number));
        Self(number)
    }
}

impl From<PlayerNumber> for u8 {
    fn from(number: PlayerNumber) -> u8 {
        number.0
    }
}

/// This struct contains the dynamic state of a game.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    /// The current mapping of the home/away team to the left/right side of the field.
    pub sides: SideMapping,
    /// The current phase of the game.
    pub phase: Phase,
    /// The current state of the game.
    pub state: State,
    /// Second state for the HL
    pub sec_state: SecondaryState,
    /// The current set play.
    pub set_play: SetPlay,
    /// The side which may play the ball during the current set play or penalty shot.
    pub kicking_side: Side,
    /// The timer which counts down the duration of a half or the current penalty shot.
    pub primary_timer: Timer,
    /// The timer which counts down set plays, timeouts, half-time break etc.
    pub secondary_timer: Timer,
    /// A timer that counts how much the primary timer has to be rewound when taking a timeout.
    pub timeout_rewind_timer: Timer,
    /// A timer that counts down until the half is switched.
    #[serde(skip)]
    pub switch_half_timer: Timer,
    /// The two competing teams.
    pub teams: EnumMap<Side, Team>,
}

impl Game {
    /// This function returns an iterator over all timers in the game.
    pub fn timers(&self) -> impl Iterator<Item = &Timer> {
        self.teams
            .values()
            .flat_map(|team| team.players.iter().map(|player| &player.penalty_timer))
            .chain([
                &self.primary_timer,
                &self.secondary_timer,
                &self.timeout_rewind_timer,
                &self.switch_half_timer,
            ])
    }

    /// This function returns a mutable iterator over all timers in the game.
    pub fn timers_mut(&mut self) -> impl Iterator<Item = &mut Timer> {
        self.teams
            .values_mut()
            .flat_map(|team| {
                team.players
                    .iter_mut()
                    .map(|player| &mut player.penalty_timer)
            })
            .chain([
                &mut self.primary_timer,
                &mut self.secondary_timer,
                &mut self.timeout_rewind_timer,
                &mut self.switch_half_timer,
            ])
    }
}

/// This struct contains the dynamic state of a team.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    /// The player number of the goalkeeper. Can be [None] (only) during a penalty shoot-out.
    pub goalkeeper: Option<PlayerNumber>,
    /// The score of the team.
    pub score: u8,
    /// The penalty counter of the team.
    pub penalty_counter: u32,
    /// The remaining number of timeouts the team can take.
    pub timeout_budget: u8,
    /// The remaining number of team messages the team can send.
    pub message_budget: u16,
    /// Whether the team has sent illegal team messages.
    pub illegal_communication: bool,
    /// The current penalty shot index.
    pub penalty_shot: u8,
    /// The mask of all penalty shot by this team (bit i means that penalty shot i was successful).
    pub penalty_shot_mask: u16,
    /// The players of the team.
    pub players: [Player; (PlayerNumber::MAX - PlayerNumber::MIN + 1) as usize],
}

impl Index<PlayerNumber> for Team {
    type Output = Player;

    fn index(&self, i: PlayerNumber) -> &Self::Output {
        &self.players[(i.0 - PlayerNumber::MIN) as usize]
    }
}

impl IndexMut<PlayerNumber> for Team {
    fn index_mut(&mut self, i: PlayerNumber) -> &mut Self::Output {
        &mut self.players[(i.0 - PlayerNumber::MIN) as usize]
    }
}

/// This enumerats the cards that a player can be shown in the Humanoid League.
#[derive(Clone, Copy, Debug, Deserialize, Enum, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum HlCard {
    Warning,
    Yellow,
    Red,
}

/// This struct contains the dynamic state of a player.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    /// The current penalty of the player.
    pub penalty: Penalty,
    /// The timer which counts down until the penalty is over.
    pub penalty_timer: Timer,
    /// The cards that this player has been shown.
    pub cards: EnumMap<HlCard, u8>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SecondaryState {
    /// Kind of the secondary State
    pub state: SecState,
    /// Side who benefits from that
    pub side: Side,
    /// phase of the secondary state
    pub phase: u8,
}

/// This enumerates the possible sources that can trigger actions.
#[derive(Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionSource {
    /// The action was triggered by a network packet. It should be replayed and even kept if
    /// actions are undone.
    Network,
    /// The action was triggered by a timer expiration (and thus should not be replayed on its
    /// own because it implicitly happens when replaying everything else).
    Timer,
    /// The action was triggered by the user (and should be replayed).
    User,
    /// The action was triggered by the (automatic) referee.
    Referee,
}
