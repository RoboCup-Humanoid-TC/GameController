import ActionButton from "./ActionButton";
import * as actions from "../../hl_actions.js";

const StatePanel = ({ game, legalGameActions }) => {
  const inHalfTimeBreak =
    (game.phase === "firstHalf" && game.state === "finished") ||
    (game.phase === "secondHalf" && game.state === "initial");
  let readyButton =
    game.secState.state != "penaltyshoot" &&
    (game.state === "initial" ||
      game.state === "timeout" ||
      (game.phase === "firstHalf" && game.state === "finished")) ? (
      <div className={inHalfTimeBreak ? "col-span-2" : "col-span-3"}>
        <ActionButton
          action={{ type: "hlStateShifter", args: { state: "ready" } }}
          label="Ready"
          legal={inHalfTimeBreak ? (game.phase === "secondHalf" ? true : false) : true}
        />
      </div>
    ) : (
      <></>
    );

  let setButton =
    game.secState.state === "penaltyshoot" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={{ type: "hlStateShifter", args: { state: "set" } }}
        label="Set"
        legal={game.state === "ready" || game.secState.state == "penaltyshoot"}
      />
    ) : (
      <></>
    );

  let playingButton =
    game.secState.state === "penaltyshoot" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={{ type: "hlStateShifter", args: { state: "playing" } }}
        label="Playing"
        legal={game.state === "set"}
      />
    ) : (
      <></>
    );

  let finishButton =
    game.secState.state === "penaltyshoot" ||
    game.state === "ready" ||
    game.state === "set" ||
    game.state === "playing" ? (
      <ActionButton
        action={{ type: "hlStateShifter", args: { state: "finished" } }}
        label="Finish"
        legal={true}
      />
    ) : (
      <></>
    );

  // This button is still displayed when we are already in the Initial state of the second half.
  // This is because the state can switch automatically to the second half and it would be bad if
  // the operator clicked the button exactly at that time, but the button switches its meaning to
  // Ready before the button is actually clicked. Therefore, both buttons (Ready and Second Half)
  // are displayed during the entire half-time break, even though only one of them can be legal.
  let secondHalfButton =
    inHalfTimeBreak && game.secState.state != "penaltyshoot" ? (
      <ActionButton
        action={{ type: "hlStateShifter", args: { state: "initial" } }}
        label="Second Half"
        legal={legalGameActions[actions.SWITCH_HALF]}
      />
    ) : (
      <></>
    );

  let penaltyshootButtons =
    game.phase === "secondHalf" && game.state === "finished" ? (
      <>
        <div className="col-span-1">
          <ActionButton
            action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsRightGoal" } }}
            label="Penalty Shots (Left Goal)"
            legal={true}
          />
        </div>
        <div className="col-span-1">
          <ActionButton
            action={{ type: "startPenaltyShootout", args: { sides: "homeDefendsLeftGoal" } }}
            label="Penalty Shots (Right Goal)"
            legal={false}
          />
        </div>
      </>
    ) : (
      <></>
    );

  let refereeTimeoutButton = (
    <div className={game.phase === "secondHalf" && game.state === "finished" ? "col-span-2" : "col-span-1"}>
      <ActionButton
        action={{ type: "timeout", args: { side: null } }}
        label="Referee Timeout"
        legal={true}
      />
    </div>
  );

  return (
    <div className="grid grid-cols-4 gap-2">
      {secondHalfButton}
      {penaltyshootButtons}
      {readyButton}
      {setButton}
      {playingButton}
      {finishButton}
      {refereeTimeoutButton}
    </div>
  );
};

export default StatePanel;
