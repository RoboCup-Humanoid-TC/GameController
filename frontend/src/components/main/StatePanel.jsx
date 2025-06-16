import ActionButton from "./ActionButton";
import * as actions from "../../actions.js";

const StatePanel = ({ game, legalGameActions }) => {
  const inHalfTimeBreak =
    (game.phase === "firstHalf" && game.state === "finished") ||
    (game.phase === "secondHalf" && game.state === "initial");
  let readyButton =
    game.phase != "penaltyShootout" &&
    (game.state === "initial" ||
      game.state === "timeout" ||
      (game.phase === "firstHalf" && game.state === "finished")) ? (
      <div className={inHalfTimeBreak ? "col-span-3" : "col-span-4"}>
        <ActionButton
          action={{ type: "startSetPlay", args: { side: game.kickingSide, setPlay: "kickOff" } }}
          label="Ready"
          legal={
            legalGameActions[
              game.kickingSide === "home"
                ? actions.START_KICK_OFF_HOME
                : actions.START_KICK_OFF_AWAY
            ]
          }
        />
      </div>
    ) : (
      <></>
    );

  let setButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "waitForPenaltyShot", args: null }
            : { type: "waitForSetPlay", args: null }
        }
        label="Set"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout"
              ? actions.WAIT_FOR_PENALTY_SHOT
              : actions.WAIT_FOR_SET_PLAY
          ]
        }
      />
    ) : (
      <></>
    );

  let playingButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <div className={game.phase === "penaltyShootout" ? "col-span-2" : "col-span-1"}>
        <ActionButton
          action={
            game.phase === "penaltyShootout"
              ? { type: "freePenaltyShot", args: null }
              : { type: "freeSetPlay", args: null }
          }
          label={"Playing"}
          legal={
            legalGameActions[
              game.phase === "penaltyShootout" ? actions.FREE_PENALTY_SHOT : actions.FREE_SET_PLAY
            ]
          }
        />
      </div>
    ) : (
      <></>
    );

  let ballFreeButton =
    game.phase != "penaltyShootout" &&
    (game.state === "ready" || game.state === "set" || game.state === "playing") ? (
      <ActionButton
        action={{ type: "finishSetPlay", args: null }}
        label={"Ball Free"}
        legal={legalGameActions[actions.FINISH_SET_PLAY]}
      />
    ) : (
      <></>
    );

  let finishButton =
    game.phase === "penaltyShootout" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={
          game.phase === "penaltyShootout"
            ? { type: "finishPenaltyShot", args: null }
            : { type: "finishHalf", args: null }
        }
        label="Finish"
        legal={
          legalGameActions[
            game.phase === "penaltyShootout" ? actions.FINISH_PENALTY_SHOT : actions.FINISH_HALF
          ]
        }
      />
    ) : (
      <></>
    );

  // This button is still displayed when we are already in the Initial state of the second half.
  // This is because the state can switch automatically to the second half and it would be bad if
  // the operator clicked the button exactly at that time, but the button switches its meaning to
  // Ready before the button is actually clicked. Therefore, both buttons (Ready and Second Half)
  // are displayed during the entire half-time break, even though only one of them can be legal.
  let secondHalfButton = inHalfTimeBreak ? (
    <ActionButton
      action={{ type: "switchHalf", args: null }}
      label="Second Half"
      legal={legalGameActions[actions.SWITCH_HALF]}
    />
  ) : (
    <></>
  );

  let penaltyShootoutButtons =
    game.phase === "secondHalf" && game.state === "finished" ? (
      <>
        <div className="col-span-2">
          <ActionButton
            action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsRightGoal" } }}
            label="Penalty Shots (Left Goal)"
            legal={legalGameActions[actions.START_PENALTY_SHOOTOUT_RIGHT]}
          />
        </div>
        <div className="col-span-2">
          <ActionButton
            action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } }}
            label="Penalty Shots (Right Goal)"
            legal={legalGameActions[actions.START_PENALTY_SHOOTOUT_LEFT]}
          />
        </div>
      </>
    ) : (
      <></>
    );

  let refereeTimeoutButton = (
    <ActionButton
      action={{ type: "timeout", args: { side: null } }}
      label="Referee Timeout"
      legal={legalGameActions[actions.REFEREE_TIMEOUT]}
    />
  );

  return (
    <div className="grid grid-cols-5 gap-2">
      {secondHalfButton}
      {penaltyShootoutButtons}
      {readyButton}
      {setButton}
      {playingButton}
      {ballFreeButton}
      {finishButton}
      {refereeTimeoutButton}
    </div>
  );
};

export default StatePanel;
