import ActionButton from "./ActionButton";
import * as actions from "../../actions.js";

const getActionName = (action) => {
  switch (action.type) {
    case "addExtraTime":
      return "Add Extra Time";
    case "finishHalf":
    case "finishPenaltyShot":
      return "Finish";
    case "finishSetPlay":
      return "Set Play Complete";
    case "freePenaltyShot":
    case "freeSetPlay":
      return "Playing";
    case "globalGameStuck":
      return "Drop Ball";
    case "goal":
      return "Goal";
    case "penalize": {
      const penalty = actions.PENALTIES.find((penalty) => penalty[1] === action.args.call);
      if (penalty) {
        return penalty[0];
      }
      return "Penalize";
    }
    case "selectPenaltyShotPlayer":
      return "Select";
    case "startPenaltyShootout":
      return "Penalty Shoot-out";
    case "startSetPlay":
      switch (action.args.setPlay) {
        case "kickOff":
          return "Ready";
        case "kickIn":
          return "Kick-in";
        case "goalKick":
          return "Goal Kick";
        case "cornerKick":
          return "Corner Kick";
      }
      break;
    case "substitute":
      return "Substitute";
    case "switchHalf":
      return "Second Half";
    case "timeout":
      return action.args.side ? "Timeout" : "Referee Timeout";
    case "unpenalize":
      return "Unpenalize";
    case "waitForPenaltyShot":
    case "waitForSetPlay":
      return "Set";
    case "hlUnpenalize":
      return "Unpenalize";
    case "hlStateShifter":
      switch (action.args.state) {
        case "initial":
          return "Second Half";
        case "ready":
          return "Ready"
        case "set":
          return "Set";
        case "playing":
          return "Playing";
        case "finished":
          return "Finish";
      }
    case "hlSetPlay":
      // TODO: Cant access secondaryState.state and SecondaryState.phase
      switch (action.args.setPlay) {
        case "directFreekick":
          return "Direct Free Kick";
        case "indirectFreekick":
          return "Indirect Free Kick";
        case "penaltykick":
          return "Penalty Kick";
        case "cornerKick":
          return "Corner Kick";
        case "goalKick":
          return "Goal Kick";
        case "throwIn":
          return "Throw-in";
      }
    case "hlAbort":
      return "Abort";
    case "hlRetake":
      return "Retake";
    case "hlSubstitute":
      return "Substitute";
    case "hlAddCard":
      switch (action.args.card) {
        case "warning":
          return "Warning";
        case "yellow":
          return "Yellow Card";
        case "red":
          return "Red Card";
      }
    case "hlPenalize":
      switch (action.args.penalty) {
        case "ballHolding":
          return "Ball Manipulation";
        case "playerPushing":
          return "Player Pushing";
        case "pickedUp":
          return "Picked Up";
      }
  }
  return action.type;
};

const UndoPanel = ({ undoActions, legalUndoActions }) => {
  return (
    <div className="flex flex-row-reverse gap-2 h-10">
      {legalUndoActions.map((legal, index) => (
        <ActionButton
          action={{ type: "undo", args: { states: index + 1 } }}
          label={index < undoActions.length ? getActionName(undoActions[index]) : "Undo"}
          legal={legal}
          key={index}
        />
      ))}
    </div>
  );
};

export default UndoPanel;
