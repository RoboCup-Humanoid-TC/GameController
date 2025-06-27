use bytes::{BufMut, Bytes, BytesMut};

use game_controller_core::{
    timer::SignedDuration,
    types::{
        ChallengeMode, Color, Game, Params, Penalty, Phase, PlayerNumber, SetPlay, Side,
        SideMapping, State,
    },
};

use crate::bindings::{
    COMPETITION_PHASE_PLAYOFF, COMPETITION_PHASE_ROUNDROBIN, COMPETITION_TYPE_NORMAL,
    GAMECONTROLLER_STRUCT_HEADER, GAMECONTROLLER_STRUCT_SIZE, GAMECONTROLLER_STRUCT_VERSION,
    GAME_PHASE_NORMAL, GAME_PHASE_PENALTYSHOOT, GAME_PHASE_TIMEOUT, MAX_NUM_PLAYERS, PENALTY_NONE,
    PENALTY_SPL_ILLEGAL_BALL_CONTACT, PENALTY_SPL_ILLEGAL_MOTION_IN_SET,
    PENALTY_SPL_ILLEGAL_POSITION, PENALTY_SPL_ILLEGAL_POSITION_IN_SET, PENALTY_SPL_INACTIVE_PLAYER,
    PENALTY_SPL_LEAVING_THE_FIELD, PENALTY_SPL_LOCAL_GAME_STUCK, PENALTY_SPL_PLAYER_PUSHING,
    PENALTY_SPL_PLAYER_STANCE, PENALTY_SPL_REQUEST_FOR_PICKUP, PENALTY_SUBSTITUTE,
    SET_PLAY_CORNER_KICK, SET_PLAY_GOAL_KICK, SET_PLAY_KICK_IN, SET_PLAY_NONE,
    SET_PLAY_PENALTY_KICK, SET_PLAY_PUSHING_FREE_KICK, STATE_FINISHED, STATE_INITIAL,
    STATE_PLAYING, STATE_READY, STATE_SET, TEAM_BLACK, TEAM_BLUE, TEAM_BROWN, TEAM_GRAY,
    TEAM_GREEN, TEAM_ORANGE, TEAM_PURPLE, TEAM_RED, TEAM_WHITE, TEAM_YELLOW,
};

/// This struct corresponds to the `RobotInfo`.
#[derive(Debug)]
pub struct ControlMessagePlayer {
    /// This field corresponds to `RobotInfo::penalty`.
    penalty: u8,
    /// This field corresponds to `RobotInfo::secsTillUnpenalised`.
    secs_till_unpenalized: u8,
}

/// This struct corresponds to the `TeamInfo`.
#[derive(Debug)]
pub struct ControlMessageTeam {
    /// This field corresponds to `TeamInfo::teamNumber`.
    number: u8,
    /// This field corresponds to `TeamInfo::fieldPlayerColour`.
    field_player_color: u8,
    /// This field corresponds to `TeamInfo::goalkeeperColour`.
    goalkeeper_color: u8,
    /// This field corresponds to `TeamInfo::goalkeeper`.
    goalkeeper: u8,
    /// This field corresponds to `TeamInfo::score`.
    score: u8,
    /// This field corresponds to `TeamInfo::penaltyShot`.
    penalty_shot: u8,
    /// This field corresponds to `TeamInfo::singleShots`.
    single_shots: u16,
    /// This field corresponds to `TeamInfo::messageBudget`.
    message_budget: u16,
    /// This field corresponds to `TeamInfo::players`.
    players: [ControlMessagePlayer; MAX_NUM_PLAYERS as usize],
}

/// This struct corresponds to `RoboCupGameControlData`. `RoboCupGameControlData::header` and
/// `RoboCupGameControlData::version` are implicitly added/removed when converting to/from the
/// binary format.
pub struct ControlMessage {
    /// This field specifies if the message is sent to a monitor (`true`) or to the players
    /// (`false`).
    to_monitor: bool,
    /// This field corresponds to `RoboCupGameControlData::packetNumber`.
    packet_number: u8,
    /// This field corresponds to `RoboCupGameControlData::playersPerTeam`.
    players_per_team: u8,
    /// This field corresponds to `RoboCupGameControlData::competitionPhase`.
    competition_phase: u8,
    /// This field corresponds to `RoboCupGameControlData::competitionType`.
    competition_type: u8,
    /// This field corresponds to `RoboCupGameControlData::gamePhase`.
    game_phase: u8,
    /// This field corresponds to `RoboCupGameControlData::state`.
    state: u8,
    /// This field corresponds to `RoboCupGameControlData::setPlay`.
    set_play: u8,
    /// This field corresponds to `RoboCupGameControlData::firstHalf`.
    first_half: bool,
    /// This field corresponds to `RoboCupGameControlData::kickingTeam`.
    kicking_team: u8,
    /// This field corresponds to `RoboCupGameControlData::secsRemaining`.
    secs_remaining: i16,
    /// This field corresponds to `RoboCupGameControlData::secondaryTime`.
    secondary_time: i16,
    /// This field corresponds to `RoboCupGameControlData::teams`.
    teams: [ControlMessageTeam; 2],
}

impl From<ControlMessage> for Bytes {
    fn from(message: ControlMessage) -> Self {
        let mut bytes = BytesMut::with_capacity(GAMECONTROLLER_STRUCT_SIZE);
        bytes.put(if message.to_monitor {
            &b"RGTD"[..4]
        } else {
            &GAMECONTROLLER_STRUCT_HEADER[..4]
        });
        bytes.put_u8(GAMECONTROLLER_STRUCT_VERSION);
        bytes.put_u8(message.packet_number);
        bytes.put_u8(message.players_per_team);
        bytes.put_u8(message.competition_phase);
        bytes.put_u8(message.competition_type);
        bytes.put_u8(message.game_phase);
        bytes.put_u8(message.state);
        bytes.put_u8(message.set_play);
        bytes.put_u8(if message.first_half { 1 } else { 0 });
        bytes.put_u8(message.kicking_team);
        bytes.put_i16_le(message.secs_remaining);
        bytes.put_i16_le(message.secondary_time);
        for team in &message.teams {
            bytes.put_u8(team.number);
            bytes.put_u8(team.field_player_color);
            bytes.put_u8(team.goalkeeper_color);
            bytes.put_u8(team.goalkeeper);
            bytes.put_u8(team.score);
            bytes.put_u8(team.penalty_shot);
            bytes.put_u16_le(team.single_shots);
            bytes.put_u16_le(team.message_budget);
            for player in &team.players {
                bytes.put_u8(player.penalty);
                bytes.put_u8(player.secs_till_unpenalized);
            }
        }
        assert!(bytes.len() == GAMECONTROLLER_STRUCT_SIZE);
        bytes.freeze()
    }
}

fn get_duration(duration: SignedDuration, min: i64, max: i64) -> i64 {
    (duration.whole_seconds()
        + if duration.subsec_nanoseconds() > 0 {
            1
        } else {
            0
        })
    .clamp(min, max)
}

fn get_color(color: Color) -> u8 {
    match color {
        Color::Blue => TEAM_BLUE,
        Color::Red => TEAM_RED,
        Color::Yellow => TEAM_YELLOW,
        Color::Black => TEAM_BLACK,
        Color::White => TEAM_WHITE,
        Color::Green => TEAM_GREEN,
        Color::Orange => TEAM_ORANGE,
        Color::Purple => TEAM_PURPLE,
        Color::Brown => TEAM_BROWN,
        Color::Gray => TEAM_GRAY,
    }
}

impl ControlMessage {
    /// This function creates a new [ControlMessage] from a given
    /// [game_controller_core::types::Game] and [game_controller_core::types::Params]. The caller
    /// must also specify a packet number and if the message is targeted at a monitor application or
    /// the players, since the header signature is different.
    pub fn new(game: &Game, params: &Params, packet_number: u8, to_monitor: bool) -> Self {
        let team_order = match game.sides {
            SideMapping::HomeDefendsLeftGoal => [Side::Home, Side::Away],
            SideMapping::HomeDefendsRightGoal => [Side::Away, Side::Home],
        };
        Self {
            to_monitor,
            packet_number,
            players_per_team: params.competition.players_per_team,
            competition_phase: if params.game.long {
                COMPETITION_PHASE_PLAYOFF
            } else {
                COMPETITION_PHASE_ROUNDROBIN
            },
            competition_type: match params.competition.challenge_mode {
                Some(ChallengeMode::DropIn) => COMPETITION_TYPE_NORMAL,
                Some(ChallengeMode::KidSize) => COMPETITION_TYPE_NORMAL,
                Some(ChallengeMode::AdultSize) => COMPETITION_TYPE_NORMAL,
                None => COMPETITION_TYPE_NORMAL,
            },
            game_phase: match (game.phase, game.state) {
                (_, State::Timeout) => GAME_PHASE_TIMEOUT,
                (Phase::FirstHalf | Phase::SecondHalf, _) => GAME_PHASE_NORMAL,
                (Phase::PenaltyShootout, _) => GAME_PHASE_PENALTYSHOOT,
            },
            state: match game.state {
                State::Initial | State::Timeout => STATE_INITIAL,
                State::Ready => STATE_READY,
                State::Set => STATE_SET,
                State::Playing => STATE_PLAYING,
                State::Finished => STATE_FINISHED,
            },
            set_play: match game.set_play {
                SetPlay::NoSetPlay | SetPlay::KickOff => SET_PLAY_NONE,
                SetPlay::KickIn => SET_PLAY_KICK_IN,
                SetPlay::GoalKick => SET_PLAY_GOAL_KICK,
                SetPlay::CornerKick => SET_PLAY_CORNER_KICK,
                SetPlay::PushingFreeKick => SET_PLAY_PUSHING_FREE_KICK,
                SetPlay::PenaltyKick => SET_PLAY_PENALTY_KICK,
                SetPlay::ThrowIn => SET_PLAY_NONE, // not used in spl
                SetPlay::DirectFreeKick => SET_PLAY_NONE, // not used in spl
                SetPlay::IndirectFreeKick => SET_PLAY_NONE, // not used in spl

            },
            first_half: game.phase == Phase::FirstHalf,
            kicking_team: game
                .kicking_side
                .map_or(255u8, |side| params.game.teams[side].number),
            secs_remaining: get_duration(
                game.primary_timer.get_remaining(),
                i16::MIN as i64,
                i16::MAX as i64,
            ) as i16,
            secondary_time: get_duration(
                game.secondary_timer.get_remaining(),
                i16::MIN as i64,
                i16::MAX as i64,
            ) as i16,
            teams: team_order.map(|side| ControlMessageTeam {
                number: params.game.teams[side].number,
                field_player_color: get_color(params.game.teams[side].field_player_color),
                goalkeeper_color: get_color(params.game.teams[side].goalkeeper_color),
                goalkeeper: game.teams[side].goalkeeper.map_or_else(
                    || {
                        game.teams[side]
                            .players
                            .iter()
                            .enumerate()
                            .find(|player| player.1.penalty == Penalty::Substitute)
                            .map(|player| (player.0 as u8) + PlayerNumber::MIN)
                            .unwrap_or(0u8)
                    },
                    |goalkeeper| goalkeeper.into(),
                ),
                score: game.teams[side].score,
                penalty_shot: game.teams[side].penalty_shot,
                single_shots: game.teams[side].penalty_shot_mask,
                message_budget: game.teams[side].message_budget,
                players: game.teams[side]
                    .players
                    // The alternative to this clone is doing iter() here, and collecting into a
                    // Vec in the end, and then try_into() that Vec into the fixed size array.
                    .clone()
                    .map(|player| ControlMessagePlayer {
                        penalty: match player.penalty {
                            Penalty::NoPenalty => PENALTY_NONE,
                            Penalty::Substitute => PENALTY_SUBSTITUTE,
                            Penalty::PickedUp => PENALTY_SPL_REQUEST_FOR_PICKUP,
                            Penalty::IllegalPositionInSet => PENALTY_SPL_ILLEGAL_POSITION_IN_SET,
                            Penalty::IllegalPosition => PENALTY_SPL_ILLEGAL_POSITION,
                            Penalty::MotionInSet => PENALTY_SPL_ILLEGAL_MOTION_IN_SET,
                            Penalty::FallenInactive => PENALTY_SPL_INACTIVE_PLAYER,
                            Penalty::LocalGameStuck => PENALTY_SPL_LOCAL_GAME_STUCK,
                            Penalty::BallHolding | Penalty::PlayingWithArmsHands => {
                                PENALTY_SPL_ILLEGAL_BALL_CONTACT
                            }
                            Penalty::PlayerStance => PENALTY_SPL_PLAYER_STANCE,
                            Penalty::PlayerPushing => PENALTY_SPL_PLAYER_PUSHING,
                            Penalty::LeavingTheField => PENALTY_SPL_LEAVING_THE_FIELD,
                        },
                        secs_till_unpenalized: get_duration(
                            player.penalty_timer.get_remaining(),
                            u8::MIN as i64,
                            u8::MAX as i64,
                        ) as u8,
                    }),
            }),
        }
    }
}
