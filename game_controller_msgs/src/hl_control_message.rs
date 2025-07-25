use bytes::{BufMut, Bytes, BytesMut};

use game_controller_core::{
    timer::SignedDuration,
    types::{
        ChallengeMode, Color, Game, HlCard, Params, Penalty, Phase, PlayerNumber, SetPlay, Side,
        SideMapping, State,
    },
};

use crate::bindings::{
    DROPBALL, GAMECONTROLLER_STRUCT_HEADER, GAME_ADULT, GAME_DROPIN, GAME_KID_SIZE,
    HL_BALL_MANIPULATION, HL_GAMECONTROLLER_STRUCT_SIZE, HL_MAX_NUM_PLAYERS, HL_PHYSICAL_CONTACT,
    HL_PICKUP_OR_INCAPABLE, MAX_NUM_PLAYERS, PENALTY_NONE, STATE2_CORNER_KICK,
    STATE2_DIRECT_FREEKICK, STATE2_GOAL_KICK, STATE2_INDIRECT_FREEKICK, STATE2_NORMAL,
    STATE2_OVERTIME, STATE2_PENALTYKICK, STATE2_PENALTYSHOOT, STATE2_THROW_IN, STATE2_TIMEOUT,
    STATE_FINISHED, STATE_INITIAL, STATE_PLAYING, STATE_READY, STATE_SET, SUBSTITUTE, TEAM_BLACK,
    TEAM_BLUE, TEAM_BROWN, TEAM_GRAY, TEAM_GREEN, TEAM_ORANGE, TEAM_PURPLE, TEAM_RED, TEAM_WHITE,
    TEAM_YELLOW, UNKNOWN,
};

/// This struct corresponds to the `RobotInfo`.
#[derive(Debug)]
pub struct ControlMessagePlayer {
    /// This field corresponds to `RobotInfo::penalty`.
    penalty: u8,
    /// This field corresponds to `RobotInfo::secsTillUnpenalised`.
    secs_till_unpenalized: u8,
    /// number of warnings
    number_of_warnings: u8,
    /// yellow card count
    yellow_card_count: u8,
    /// red card count
    red_card_count: u8,
    // goalkeeper
    goalkeeper: u8,
}

/// This struct corresponds to the `TeamInfo`.
/// TODO: fixed array sizes as constants
#[derive(Debug)]
pub struct ControlMessageTeam {
    /// This field corresponds to `TeamInfo::teamNumber`.
    number: u8,
    /// This field corresponds to `TeamInfo::fieldPlayerColour`.
    field_player_color: u8,
    /// This field corresponds to `TeamInfo::score`.
    score: u8,
    /// This field corresponds to `TeamInfo::penaltyShot`.
    penalty_shot: u8,
    /// This field corresponds to `TeamInfo::singleShots`.
    single_shots: u16,
    /// coach sequence
    coach_sequence: u8,
    /// coach message
    coach_message: [u8; 253],
    /// coach
    coach: ControlMessagePlayer,
    /// This field corresponds to `TeamInfo::players`.
    players: [ControlMessagePlayer; MAX_NUM_PLAYERS as usize],
}

/// This struct corresponds to `RoboCupGameControlData`. `RoboCupGameControlData::header` and
/// `RoboCupGameControlData::version` are implicitly added/removed when converting to/from the
/// binary format.
pub struct HlControlMessage {
    /// This field specifies if the message is sent to a monitor (`true`) or to the players
    /// (`false`).
    version: u16,
    /// This field corresponds to `RoboCupGameControlData::packetNumber`.
    packet_number: u8,
    /// This field corresponds to `RoboCupGameControlData::playersPerTeam`.
    players_per_team: u8,
    /// This field corresponds to `RoboCupGameControlData::competitionType`.
    competition_type: u8,
    /// This field corresponds to `RoboCupGameControlData::state`.
    state: u8,
    /// This field corresponds to `RoboCupGameControlData::firstHalf`.
    first_half: bool,
    /// This field corresponds to `RoboCupGameControlData::kickingTeam`.
    kicking_team: u8,
    /// secondary gamestate
    sec_game_state: u8,
    /// secondary gamestate info
    sec_game_state_info: [u8; 4],
    /// drop in team
    drop_in_team: u8,
    /// drop in time
    drop_in_time: u16,
    /// This field corresponds to `RoboCupGameControlData::secsRemaining`.
    secs_remaining: i16,
    /// This field corresponds to `RoboCupGameControlData::secondaryTime`.
    secondary_time: i16,
    /// This field corresponds to `RoboCupGameControlData::teams`.
    teams: [ControlMessageTeam; 2],
}

impl From<HlControlMessage> for Bytes {
    fn from(message: HlControlMessage) -> Self {
        let mut bytes = BytesMut::with_capacity(HL_GAMECONTROLLER_STRUCT_SIZE);
        bytes.put(&GAMECONTROLLER_STRUCT_HEADER[..4]);
        bytes.put_u16_le(message.version);
        bytes.put_u8(message.packet_number);
        bytes.put_u8(message.players_per_team);
        bytes.put_u8(message.competition_type);
        bytes.put_u8(message.state);
        bytes.put_u8(if message.first_half { 1 } else { 0 });
        bytes.put_u8(message.kicking_team);
        bytes.put_u8(message.sec_game_state);
        bytes.put(&message.sec_game_state_info[..]);
        bytes.put_u8(message.drop_in_team);
        bytes.put_u16_le(message.drop_in_time);
        bytes.put_i16_le(message.secs_remaining);
        bytes.put_i16_le(message.secondary_time);
        for team in &message.teams {
            bytes.put_u8(team.number);
            bytes.put_u8(team.field_player_color);
            bytes.put_u8(team.score);
            bytes.put_u8(team.penalty_shot);
            bytes.put_u16_le(team.single_shots);
            bytes.put_u8(team.coach_sequence);
            bytes.put(&team.coach_message[..]);
            bytes.put_u8(team.coach.penalty);
            bytes.put_u8(team.coach.secs_till_unpenalized);
            bytes.put_u8(team.coach.number_of_warnings);
            bytes.put_u8(team.coach.yellow_card_count);
            bytes.put_u8(team.coach.red_card_count);
            bytes.put_u8(team.coach.goalkeeper);
            for idx in 0..(HL_MAX_NUM_PLAYERS as usize) {
                bytes.put_u8(team.players[idx].penalty);
                bytes.put_u8(team.players[idx].secs_till_unpenalized);
                bytes.put_u8(team.players[idx].number_of_warnings);
                bytes.put_u8(team.players[idx].yellow_card_count);
                bytes.put_u8(team.players[idx].red_card_count);
                bytes.put_u8(team.players[idx].goalkeeper);
            }
        }
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

/// TODO:
impl HlControlMessage {
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
            version: 12,
            packet_number,
            players_per_team: params.competition.players_per_team,
            competition_type: match params.competition.challenge_mode {
                Some(ChallengeMode::DropIn) => GAME_DROPIN,
                Some(ChallengeMode::AdultSize) => GAME_ADULT,
                Some(ChallengeMode::KidSize) | None => GAME_KID_SIZE,
            },
            state: match game.state {
                State::Initial | State::Timeout => STATE_INITIAL,
                State::Ready => STATE_READY,
                State::Set => STATE_SET,
                State::Playing => STATE_PLAYING,
                State::Finished => STATE_FINISHED,
            },
            first_half: game.phase == Phase::FirstHalf || game.phase == Phase::FirstExtraHalf,
            kicking_team: game
                .kicking_side
                .map_or(DROPBALL, |side| params.game.teams[side].number),
            sec_game_state: match game.set_play {
                SetPlay::DirectFreeKick => STATE2_DIRECT_FREEKICK,
                SetPlay::IndirectFreeKick => STATE2_INDIRECT_FREEKICK,
                SetPlay::PenaltyKick => STATE2_PENALTYKICK,
                SetPlay::CornerKick => STATE2_CORNER_KICK,
                SetPlay::GoalKick => STATE2_GOAL_KICK,
                SetPlay::ThrowIn => STATE2_THROW_IN,
                _ => {
                    if game.state == State::Timeout {
                        STATE2_TIMEOUT
                    } else {
                        match game.phase {
                            Phase::FirstHalf | Phase::SecondHalf => STATE2_NORMAL,
                            Phase::FirstExtraHalf | Phase::SecondExtraHalf => STATE2_OVERTIME,
                            Phase::PenaltyShootout => STATE2_PENALTYSHOOT,
                        }
                    }
                }
            },
            // TODO: ????
            sec_game_state_info: [
                if game.set_play != SetPlay::NoSetPlay {
                    game.kicking_side
                        .map_or(DROPBALL, |side| params.game.teams[side].number)
                } else {
                    0
                },
                game.sec_state_phase,
                0,
                0,
            ],
            // TODO: until here
            drop_in_team: 0,
            drop_in_time: u16::MAX,
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
                score: game.teams[side].score,
                penalty_shot: game.teams[side].penalty_shot,
                single_shots: game.teams[side].penalty_shot_mask,
                coach_sequence: 0,
                coach_message: [0; 253],
                coach: ControlMessagePlayer {
                    penalty: 0,
                    secs_till_unpenalized: 0,
                    number_of_warnings: 0,
                    yellow_card_count: 0,
                    red_card_count: 0,
                    goalkeeper: 0,
                },
                players: game.teams[side]
                    .players
                    .iter()
                    .enumerate()
                    .map(|(index, player)| ControlMessagePlayer {
                        penalty: match player.penalty {
                            Penalty::NoPenalty => PENALTY_NONE,
                            Penalty::Substitute => SUBSTITUTE,
                            Penalty::PickedUp => HL_PICKUP_OR_INCAPABLE,
                            Penalty::IllegalPositionInSet => UNKNOWN,
                            Penalty::IllegalPosition => UNKNOWN,
                            Penalty::MotionInSet => UNKNOWN,
                            Penalty::FallenInactive => UNKNOWN,
                            Penalty::LocalGameStuck => UNKNOWN,
                            Penalty::BallHolding | Penalty::PlayingWithArmsHands => {
                                HL_BALL_MANIPULATION
                            }
                            Penalty::PlayerStance => UNKNOWN,
                            Penalty::PlayerPushing => HL_PHYSICAL_CONTACT,
                            Penalty::LeavingTheField => UNKNOWN,
                        },
                        secs_till_unpenalized: get_duration(
                            player.penalty_timer.get_remaining(),
                            u8::MIN as i64,
                            u8::MAX as i64,
                        ) as u8,
                        // TODO
                        number_of_warnings: player.cards[HlCard::Warning],
                        yellow_card_count: player.cards[HlCard::Yellow],
                        red_card_count: player.cards[HlCard::Red],
                        goalkeeper: game.teams[side].goalkeeper.is_some_and(|goalkeeper| {
                            u8::from(goalkeeper) - PlayerNumber::MIN == index as u8
                        }) as u8,
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }),
        }
    }
}
