//! This crate contains the core functionality of the GameController. This comprises:
//! - definition of states and timers
//! - applying actions to a state in response to external events or (user-controlled) passing of time
//! - logging
//!
//! It does not contain a user interface and does not handle network packets.

pub mod action;
pub mod actions;
pub mod log;
pub mod timer;
pub mod types;

use std::{cmp::min, iter::once, time::Duration};

use enum_map::enum_map;

use crate::action::{ActionContext, VAction};
use crate::actions::*;
use crate::log::{LogEntry, LoggedAction, Logger, TimestampedLogEntry};
use crate::timer::{BehaviorAtZero, EvaluatedRunConditions, RunCondition, Timer};
use crate::types::{
    ActionSource, Game, HlCard, Params, Penalty, Phase, Player, PlayerNumber, SecState,
    SecondaryState, SetPlay, Side, State, Team,
};

/// This struct encapsulates a delayed game state.
pub struct DelayHandler {
    game: Game,
    timer: Timer,
    ignore: Box<dyn Fn(&VAction) -> bool + Send>,
}

/// This struct handles the main logic of the GameController.
pub struct GameController {
    /// The constant parameters of the game.
    pub params: Params,
    game: Game,
    delay: Option<DelayHandler>,
    time: Duration,
    history: Vec<(Game, VAction)>,
    logger: Box<dyn Logger + Send>,
}

impl GameController {
    /// This function creates a new instance with given parameters and a logger.
    pub fn new(params: Params, logger: Box<dyn Logger + Send>) -> Self {
        let game = Game {
            sides: params.game.side_mapping,
            phase: Phase::FirstHalf,
            state: State::Initial,
            // HL
            sec_state: SecondaryState {
                state: SecState::Normal,
                side: Side::Away,
                phase: 0,
            },
            // !HL
            set_play: SetPlay::NoSetPlay,
            kicking_side: Some(params.game.kick_off_side),
            primary_timer: Timer::Started {
                remaining: params.competition.half_duration.try_into().unwrap(),
                run_condition: RunCondition::Playing,
                behavior_at_zero: BehaviorAtZero::Overflow,
            },
            secondary_timer: Timer::Stopped,
            timeout_rewind_timer: Timer::Stopped,
            switch_half_timer: Timer::Stopped,
            teams: enum_map! {
                _ => Team {
                    goalkeeper: Some(PlayerNumber::new(1)),
                    score: 0,
                    penalty_counter: 0,
                    timeout_budget: params.competition.timeouts_per_team,
                    message_budget: params.competition.messages_per_team,
                    illegal_communication: false,
                    penalty_shot: 0,
                    penalty_shot_mask: 0,
                    players: (PlayerNumber::MIN..=PlayerNumber::MAX)
                        .map(|player| Player {
                            // By default, the higher-numbered players are substitutes.
                            penalty: if player <= params.competition.players_per_team {
                                Penalty::NoPenalty
                            } else {
                                Penalty::Substitute
                            },
                            penalty_timer: Timer::Stopped,
                            cards: enum_map! {
                                _ => 0,
                            },
                        })
                        // We have to collect into a Vec first because this thing cannot be directly
                        // collected into a fixed size array.
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                },
            },
        };
        Self {
            params,
            game,
            delay: None,
            time: Duration::ZERO,
            history: vec![],
            logger,
        }
    }

    /// This function returns the dynamic state of the game. The caller can request if the game
    /// state should be the delayed game state.
    pub fn get_game(&self, delayed: bool) -> &Game {
        if delayed {
            self.delay.as_ref().map_or(&self.game, |delay| &delay.game)
        } else {
            &self.game
        }
    }

    /// This function returns an action context for the game. Although there is nothing that
    /// prevents it from being used in other ways, it should only be used to check if actions are
    /// legal. (I'm too lazy at the moment to create different types of contexts for is_legal and
    /// execute.)
    pub fn get_context(&mut self, delayed: bool) -> ActionContext {
        ActionContext::new(
            if delayed {
                self.delay
                    .as_mut()
                    .map_or(&mut self.game, |delay| &mut delay.game)
            } else {
                &mut self.game
            },
            &self.params,
            None,
            (!delayed).then_some(&mut self.history),
        )
    }

    /// This function returns the last n actions that can be undone.
    pub fn get_undo_actions(&self, n: u32) -> Vec<VAction> {
        self.history
            .iter()
            .rev()
            .take(n as usize)
            .map(|entry| entry.1.clone())
            .collect()
    }

    /// This function lets time progress. Timers are updated and expiration actions applied when
    /// necessary.
    pub fn seek(&mut self, mut dt: Duration) {
        // We must split the time when timers expire in the meantime, because they can have actions
        // which must be applied at the right point in time.
        while !dt.is_zero() {
            // Find out how far we can seek at most in this iteration.
            let this_dt = self.clip_next_timer_expiration(dt);

            // Update current time and remaining offset.
            self.time += this_dt;
            dt -= this_dt;

            let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);

            // Update the delayed game state.
            if let Some(delay) = self.delay.as_mut() {
                delay.timer.seek(this_dt, &run_conditions);
                if matches!(delay.timer, Timer::Stopped) {
                    // The delay is over, so it can be switched back to the true state.
                    self.delay = None;
                } else {
                    // Seek timers in the delayed game state and apply their actions.
                    let delayed_run_conditions =
                        EvaluatedRunConditions::new(&delay.game, &self.params);
                    let delayed_actions: Vec<VAction> = delay
                        .game
                        .timers_mut()
                        .filter_map(|timer| timer.seek(this_dt, &delayed_run_conditions))
                        .flatten()
                        .collect();
                    for action in delayed_actions {
                        self.apply_delayed(&action);
                    }
                }
            }

            // Seek timers and obtain actions of timers that expire at the end of this iteration.
            let actions: Vec<VAction> = self
                .game
                .timers_mut()
                .filter_map(|timer| timer.seek(this_dt, &run_conditions))
                .flatten()
                .collect();

            for action in actions {
                self.apply(action, ActionSource::Timer);
            }
        }
    }

    /// This function applies an action, given that it is legal. Some special cases will be
    /// filtered here. The action as well as the resulting game state is logged.
    pub fn apply(&mut self, action: VAction, source: ActionSource) {
        let mut context = ActionContext::new(
            &mut self.game,
            &self.params,
            Some(&mut self.delay),
            Some(&mut self.history),
        );
        if !action.is_legal(&context) {
            return;
        }

        if source == ActionSource::User || source == ActionSource::Referee {
            context.add_to_history(action.clone());
        }

        action.execute(&mut context);
        // I'm not sure if timer-triggered actions from the non-delayed state should still cancel
        // the delayed state if they are illegal.
        if source != ActionSource::Timer {
            self.apply_delayed(&action);
        }
        self.log_now(LogEntry::Action(LoggedAction { source, action }));
        self.log_now(LogEntry::GameState(Box::new(self.game.clone())));
    }

    /// This function adds an entry to the log file at the current point in time.
    pub fn log_now(&mut self, entry: LogEntry) {
        self.logger.append(TimestampedLogEntry {
            timestamp: self.time,
            entry,
        });
    }

    /// This function clips a given timestamp (given as duration from now) to the time when the
    /// next timer expires.
    pub fn clip_next_timer_expiration(&self, max: Duration) -> Duration {
        let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);
        self.game
            .timers()
            .chain(
                self.delay
                    .as_ref()
                    .map(|delay| once(&delay.timer))
                    .into_iter()
                    .flatten(),
            )
            .filter(|timer| timer.will_expire() && timer.is_running(&run_conditions))
            // At this point, we can be sure that the remaining time is not negative.
            .map(|timer| timer.get_remaining().try_into().unwrap())
            .chain(
                self.delay
                    .as_ref()
                    .map(|delay| {
                        let delayed_run_conditions =
                            EvaluatedRunConditions::new(&delay.game, &self.params);
                        delay
                            .game
                            .timers()
                            .filter(move |timer| {
                                timer.will_expire() && timer.is_running(&delayed_run_conditions)
                            })
                            // At this point, we can be sure that the remaining time is not
                            // negative.
                            .map(|timer| timer.get_remaining().try_into().unwrap())
                    })
                    .into_iter()
                    .flatten(),
            )
            .min()
            .map_or(max, |next| min(next, max))
    }

    /// This function clips a given timestamp (given as duration from now) to the time when the
    /// next timer, when rounded to seconds in a particular way, changes.
    pub fn clip_next_timer_wrap(&self, max: Duration) -> Duration {
        let next_wrap = |timer: &Timer| {
            Duration::from_nanos(
                ({
                    let nanos = timer.get_remaining().subsec_nanoseconds();
                    if nanos <= 0 {
                        nanos + 1_000_000_000
                    } else {
                        nanos
                    }
                })
                .try_into()
                .unwrap(),
            )
        };
        let run_conditions = EvaluatedRunConditions::new(&self.game, &self.params);
        self.game
            .timers()
            .filter(|timer| timer.is_running(&run_conditions))
            .map(next_wrap)
            .chain(
                self.delay
                    .as_ref()
                    .map(|delay| {
                        let delayed_run_conditions =
                            EvaluatedRunConditions::new(&delay.game, &self.params);
                        delay
                            .game
                            .timers()
                            .filter(move |timer| timer.is_running(&delayed_run_conditions))
                            .map(next_wrap)
                    })
                    .into_iter()
                    .flatten(),
            )
            .min()
            .map_or(max, |next| min(next, max))
    }

    fn apply_delayed(&mut self, action: &VAction) {
        if let Some(delay) = self.delay.as_mut() {
            let mut context = ActionContext::new(&mut delay.game, &self.params, None, None);
            if action.is_legal(&context) {
                action.execute(&mut context);
            } else if !(delay.ignore)(action) {
                self.delay = None;
            }
        }
    }

    pub fn automated_referee(
        &mut self,
        command_1: u8,
        command_2: u8,
        command_3: u8,
        command_4: u8,
        command_5: u8,
    ) {
        println!(
            "DEBUG CORE: {:?}, {:?}, {:?}, {:?}, {:?}",
            command_1, command_2, command_3, command_4, command_5
        );
        // Phase commands
        let side = match command_5 {
            0 => Side::Home,
            1 => Side::Away,
            _ => Side::Home,
        };
        match command_1 {
            1 => self.apply(
                VAction::HlStateShifter(HlStateShifter {
                    state: State::Ready,
                }),
                ActionSource::Referee,
            ),
            2 => self.apply(
                VAction::HlStateShifter(HlStateShifter { state: State::Set }),
                ActionSource::Referee,
            ),
            3 => self.apply(
                VAction::HlStateShifter(HlStateShifter {
                    state: State::Playing,
                }),
                ActionSource::Referee,
            ),
            4 => self.apply(
                VAction::HlStateShifter(HlStateShifter {
                    state: State::Finished,
                }),
                ActionSource::Referee,
            ),
            5 => self.apply(
                VAction::HlStateShifter(HlStateShifter {
                    state: State::Initial,
                }),
                ActionSource::Referee,
            ),
            6 => self.apply(
                VAction::HlStateShifter(HlStateShifter {
                    state: State::Timeout,
                }),
                ActionSource::Referee,
            ),
            7 => self.apply(
                VAction::AddExtraTime(AddExtraTime {}),
                ActionSource::Referee,
            ),
            _ => {}
        }
        match command_2 {
            // Home
            1 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::GoalKick,
                }),
                ActionSource::Referee,
            ),
            2 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::ThrowIn,
                }),
                ActionSource::Referee,
            ),
            3 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::CornerKick,
                }),
                ActionSource::Referee,
            ),
            4 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::PenaltyKick,
                }),
                ActionSource::Referee,
            ),
            5 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::DirectFreeKick,
                }),
                ActionSource::Referee,
            ),
            6 => self.apply(
                VAction::HlSetPlay(HlSetPlay {
                    side: side,
                    set_play: SecState::IndirectFreeKick,
                }),
                ActionSource::Referee,
            ),
            7 => self.apply(
                VAction::HlRetake(HlRetake { side: side }),
                ActionSource::Referee,
            ),
            8 => self.apply(
                VAction::HlAbort(HlAbort { side: side }),
                ActionSource::Referee,
            ),
            9 => self.apply(VAction::Goal(Goal { side: side }), ActionSource::Referee),
            _ => {}
        }
        match command_3 {
            1 => self.apply(
                VAction::HlSubstitute(HlSubstitute {
                    side: side,
                    player: PlayerNumber(command_4),
                }),
                ActionSource::Referee,
            ),
            2 => self.apply(
                VAction::HlPenalize(HlPenalize {
                    side: side,
                    player: PlayerNumber(command_4),
                    penalty: Penalty::PlayerPushing,
                }),
                ActionSource::Referee,
            ),
            3 => self.apply(
                VAction::HlPenalize(HlPenalize {
                    side: side,
                    player: PlayerNumber(command_4),
                    penalty: Penalty::PickedUp,
                }),
                ActionSource::Referee,
            ),
            4 => self.apply(
                VAction::HlPenalize(HlPenalize {
                    side: side,
                    player: PlayerNumber(command_4),
                    penalty: Penalty::BallHolding,
                }),
                ActionSource::Referee,
            ),
            5 => self.apply(
                VAction::HlUnpenalize(HlUnpenalize {
                    side: side,
                    player: PlayerNumber(command_4),
                }),
                ActionSource::Referee,
            ),
            6 => self.apply(
                VAction::HlAddCard(HlAddCard {
                    side: side,
                    player: PlayerNumber(command_4),
                    card: HlCard::Red,
                }),
                ActionSource::Referee,
            ),
            7 => self.apply(
                VAction::HlAddCard(HlAddCard {
                    side: side,
                    player: PlayerNumber(command_4),
                    card: HlCard::Yellow,
                }),
                ActionSource::Referee,
            ),
            8 => self.apply(
                VAction::HlAddCard(HlAddCard {
                    side: side,
                    player: PlayerNumber(command_4),
                    card: HlCard::Warning,
                }),
                ActionSource::Referee,
            ),
            _ => {}
        }
    }
}

impl Drop for GameController {
    fn drop(&mut self) {
        self.log_now(LogEntry::End);
    }
}
