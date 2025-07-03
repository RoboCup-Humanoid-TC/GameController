export const PENALTIES = [
  ["Pushing", "pushing"],
  ["Placeholder", "foul"],
  ["Placeholder", "fallenInactive"],
  ["Placeholder", "leavingTheField"],
  ["Placeholder", "motionInSet"],
  ["Placeholder", "illegalPosition"],
  ["Ball Holding", "ballHolding"],
  ["Penalty Kick", "penaltyKick"],
  ["Placeholder", "localGameStuck"],
  ["Pick-Up", "requestForPickUp"],
  ["Placeholder", "playerStance"],
  ["Placeholder", "playingWithArmsHands"],
];
const NUM_OF_PLAYERS = 20;
const NUM_OF_TEAMS = 2;

const TEAM_ACTION_BASE = 0;

export const TIMEOUT = 0;
export const GLOBAL_GAME_STUCK = 1;
export const GOAL = 2;
export const GOAL_KICK = 3;
export const THROW_IN = 4;
export const CORNER_KICK = 5;
export const INDIRECT_FREE_KICK = 6;
export const DIRECT_FREE_KICK = 7;
export const PENALTY_KICK = 8;
export const ABORT = 9;
export const RETAKE = 10;

const NUM_OF_TEAM_ACTIONS = 11;

const GAME_ACTION_BASE = TEAM_ACTION_BASE + NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS;

export const SWITCH_HALF = 0;
export const START_PENALTY_SHOOTOUT_LEFT = 1;
export const START_PENALTY_SHOOTOUT_RIGHT = 2;
export const WAIT_FOR_PENALTY_SHOT = 3;
export const WAIT_FOR_SET_PLAY = 4;
export const FREE_PENALTY_SHOT = 5;
export const FINISH_SET_PLAY = 6;
export const FREE_SET_PLAY = 7;
export const FINISH_PENALTY_SHOT = 8;
export const FINISH_HALF = 9;
// These are game actions because they are part of the center panel.
export const START_KICK_OFF_HOME = 10;
export const START_KICK_OFF_AWAY = 11;
export const ADD_EXTRA_TIME = 12;
export const REFEREE_TIMEOUT = 13;
export const INITIAL = 14;
export const READY = 15;
export const SET = 16;
export const PLAYING = 17;
export const FINISHED = 18;
export const HL_TIMEOUT = 19;

const NUM_OF_GAME_ACTIONS = 20;

const PENALTY_ACTION_BASE = GAME_ACTION_BASE + NUM_OF_GAME_ACTIONS;

const NUM_OF_PENALTY_ACTIONS = NUM_OF_TEAMS * NUM_OF_PLAYERS * (6 + 3); // The + 1 is the unpenalize action.

const UNDO_ACTION_BASE = PENALTY_ACTION_BASE + NUM_OF_PENALTY_ACTIONS;

const NUM_OF_UNDO_ACTIONS = 5;

export const NUM_OF_ACTIONS =
  NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS +
  NUM_OF_GAME_ACTIONS +
  NUM_OF_PENALTY_ACTIONS +
  NUM_OF_UNDO_ACTIONS;

export const getActions = () => {
  var actions = [];
  for (const side of ["home", "away"]) {
    actions.push(
      { type: "timeout", args: { side: side } },
      { type: "globalGameStuck", args: { side: side } },
      { type: "goal", args: { side: side } },
      { type: "hlSetPlay", args: { side: side, setPlay: "goalKick", seconds: 30 } },
      { type: "hlSetPlay", args: { side: side, setPlay: "throwIn", seconds: 30 } },
      { type: "hlSetPlay", args: { side: side, setPlay: "cornerKick", seconds: 30 } },
      { type: "hlSetPlay", args: { side: side, setPlay: "directFreekick", seconds: 30 } },
      { type: "hlSetPlay", args: { side: side, setPlay: "indirectFreekick", seconds: 30 } },
      { type: "hlSetPlay", args: { side: side, setPlay: "penaltykick", seconds: 30 } },
      { type: "hlAbort", args: { side: side } },
      { type: "hlRetake", args: { side: side } }
    );
  }
  actions.push({ type: "switchHalf", args: null });
  actions.push({ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } });
  actions.push({ type: "startPenaltyShootout", args: { sides: "homeDefendsRightGoal" } });
  actions.push({ type: "waitForPenaltyShot", args: null });
  actions.push({ type: "waitForSetPlay", args: null });
  actions.push({ type: "freePenaltyShot", args: null });
  actions.push({ type: "finishSetPlay", args: null });
  actions.push({ type: "freeSetPlay", args: null });
  actions.push({ type: "finishPenaltyShot", args: null });
  actions.push({ type: "finishHalf", args: null });
  actions.push({ type: "startSetPlay", args: { side: "home", setPlay: "kickOff" } });
  actions.push({ type: "startSetPlay", args: { side: "away", setPlay: "kickOff" } });
  actions.push({ type: "addExtraTime", args: null });
  actions.push({ type: "timeout", args: { side: null } });
  actions.push({ type: "hlStateShifter", args: { state: "initial" } });
  actions.push({ type: "hlStateShifter", args: { state: "ready" } });
  actions.push({ type: "hlStateShifter", args: { state: "set" } });
  actions.push({ type: "hlStateShifter", args: { state: "playing" } });
  actions.push({ type: "hlStateShifter", args: { state: "finished" } });
  actions.push({ type: "hlStateShifter", args: { state: "timeout" } });
  /*
  for (const penalty of PENALTIES) {
    for (const side of ["home", "away"]) {
      for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
        actions.push({ type: "penalize", args: { side: side, player: number, call: penalty[1] } });
      }
    }
  }
  */
  for (const card of ["warning", "yellow", "red"]) {
    for (const side of ["home", "away"]) {
      for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
        actions.push({ type: "hlAddCard", args: { side: side, player: number, card: card } });
      }
    }
  }
  for (const penalty of ["ballHolding", "playerPushing", "pickedUp"]) {
    for (const side of ["home", "away"]) {
      for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
        actions.push({
          type: "hlPenalize",
          args: { side: side, player: number, penalty: penalty },
        });
      }
    }
  }
  for (const side of ["home", "away"]) {
    for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
      actions.push({ type: "unpenalize", args: { side: side, player: number } });
    }
  }
  for (const timer of [true, false]) {
    for (const side of ["home", "away"]) {
      for (let number = 1; number <= NUM_OF_PLAYERS; ++number) {
        actions.push({ type: "hlUnpenalize", args: { side: side, player: number } });
      }
    }
  }
  for (let states = 1; states <= NUM_OF_UNDO_ACTIONS; ++states) {
    actions.push({ type: "undo", args: { states: states } });
  }
  return actions;
};

export const extractTeamActions = (legalActions, side) => {
  return side === "home"
    ? legalActions.slice(TEAM_ACTION_BASE, TEAM_ACTION_BASE + NUM_OF_TEAM_ACTIONS)
    : legalActions.slice(
        TEAM_ACTION_BASE + NUM_OF_TEAM_ACTIONS,
        TEAM_ACTION_BASE + NUM_OF_TEAMS * NUM_OF_TEAM_ACTIONS
      );
};

export const extractGameActions = (legalActions) => {
  return legalActions.slice(GAME_ACTION_BASE, GAME_ACTION_BASE + NUM_OF_GAME_ACTIONS);
};

export const extractPenaltyActions = (legalActions) => {
  return legalActions.slice(PENALTY_ACTION_BASE, PENALTY_ACTION_BASE + NUM_OF_PENALTY_ACTIONS);
};

export const extractUndoActions = (legalActions) => {
  return legalActions.slice(UNDO_ACTION_BASE, UNDO_ACTION_BASE + NUM_OF_UNDO_ACTIONS);
};

export const isPenaltyCallLegal = (legalPenaltyActions, callIndex) => {
  return legalPenaltyActions
    .slice(
      callIndex * NUM_OF_TEAMS * NUM_OF_PLAYERS,
      (callIndex + 1) * NUM_OF_TEAMS * NUM_OF_PLAYERS
    )
    .some((element) => element != 0);
};

export const isPenaltyCallLegalForPlayer = (legalPenaltyActions, side, player, callIndex) => {
  return (
    legalPenaltyActions[
      (callIndex === null ? PENALTIES.length : callIndex) * NUM_OF_TEAMS * NUM_OF_PLAYERS +
        (side === "home" ? 0 : NUM_OF_PLAYERS) +
        (player - 1)
    ] != 0
  );
};
