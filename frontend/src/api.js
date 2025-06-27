import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { NUM_OF_ACTIONS } from "./actions.js";

export const getLaunchData = async () => {
  if (window.__TAURI_INTERNALS__) {
    return await invoke("get_launch_data");
  } else {
    return {
      competitions: [
        { id: "champions_cup", name: "Champions Cup", teams: [0, 1, 2] },
        { id: "challenge_shield", name: "Challenge Shield", teams: [0, 3, 4, 5] },
      ],
      teams: [
        {
          number: 0,
          name: "Invisibles",
          fieldPlayerColors: ["blue", "red"],
          goalkeeperColors: ["yellow", "black"],
        },
        {
          number: 1,
          name: "UT Austin Villa",
          fieldPlayerColors: ["white", "orange"],
          goalkeeperColors: ["white", "orange"],
        },
        {
          number: 2,
          name: "Austrian Kangaroos",
          fieldPlayerColors: ["blue", "red"],
          goalkeeperColors: ["blue", "red"],
        },
        {
          number: 3,
          name: "Bembelbots",
          fieldPlayerColors: ["gray", "blue"],
          goalkeeperColors: ["gray", "blue"],
        },
        {
          number: 4,
          name: "Berlin United",
          fieldPlayerColors: ["blue", "red"],
          goalkeeperColors: ["blue", "red"],
        },
        {
          number: 5,
          name: "B-Human",
          fieldPlayerColors: ["black", "red"],
          goalkeeperColors: ["black", "red"],
        },
      ],
      networkInterfaces: [
        { id: "en0", address: "10.0.0.1", broadcast: "10.0.255.255" },
        { id: "lo0", address: "127.0.0.1", broadcast: "127.0.0.1" },
      ],
      defaultSettings: {
        competition: { id: "champions_cup" },
        game: {
          teams: {
            home: { number: 0, fieldPlayerColor: "blue", goalkeeperColor: "yellow" },
            away: { number: 0, fieldPlayerColor: "red", goalkeeperColor: "black" },
          },
          long: false,
          kickOffSide: "home",
          sideMapping: "homeDefendsLeftGoal",
        },
        window: { fullscreen: false },
        network: { interface: "en0", broadcast: false, multicast: false },
        league: { league: "spl" },
      },
    };
  }
};

export const launch = async (settings) => {
  if (window.__TAURI_INTERNALS__) {
    await invoke("launch", { settings: settings });
  } else {
    console.log(settings);
  }
};

export const listenForState = async (handler) => {
  if (window.__TAURI_INTERNALS__) {
    return await getCurrentWebviewWindow().listen("state", (event) => {
      handler(event.payload);
    });
  } else {
    handler({
      game: {
        sides: "homeDefendsLeftGoal",
        phase: "firstHalf",
        state: "initial",
        setPlay: "noSetPlay",
        kickingSide: "home",
        primaryTimer: {
          started: { remaining: [600, 0], run_condition: "playing", behavior_at_zero: "overflow" },
        },
        secondaryTimer: { stopped: null },
        timeoutRewindTimer: { stopped: null },
        teams: {
          home: {
            goalkeeper: 1,
            score: 0,
            penaltyCounter: 0,
            timeoutBudget: 1,
            messageBudget: 1200,
            illegalCommunication: false,
            penaltyShot: 0,
            penaltyShotMask: 0,
            players: [
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
            ],
          },
          away: {
            goalkeeper: 1,
            score: 0,
            penaltyCounter: 0,
            timeoutBudget: 1,
            messageBudget: 1200,
            illegalCommunication: false,
            penaltyShot: 0,
            penaltyShotMask: 0,
            players: [
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "noPenalty", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
              { penalty: "substitute", penaltyTimer: { stopped: null } },
            ],
          },
        },
      },
      legalActions: new Array(NUM_OF_ACTIONS).fill(0),
      connectionStatus: {
        home: [1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0],
        away: [1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0, 1, 2, 1, 0],
      },
      undoActions: [],
    });
    return () => {};
  }
};

export const syncWithBackend = async () => {
  if (window.__TAURI_INTERNALS__) {
    return await invoke("sync_with_backend");
  } else {
    return {
      competition: {},
      game: {
        teams: {
          home: {
            number: 0,
            fieldPlayerColor: "blue",
            goalkeeperColor: "yellow",
          },
          away: {
            number: 0,
            fieldPlayerColor: "red",
            goalkeeperColor: "black",
          },
        },
        long: false,
      },
    };
  }
};

export const applyAction = (action) => {
  if (window.__TAURI_INTERNALS__) {
    invoke("apply_action", { action: action });
  } else {
    console.log(action);
  }
};

export const declareActions = (actions) => {
  if (window.__TAURI_INTERNALS__) {
    invoke("declare_actions", { actions: actions });
  } else {
    console.log(actions);
  }
};
