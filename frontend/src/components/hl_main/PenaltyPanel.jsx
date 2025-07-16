import PenaltyButtonWA from "./PenaltyButtonWA";
import { isPenaltyCallLegal } from "../../hl_actions";

const PenaltyPanel = ({ game, selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    // TODO: REFACTOR: after RoboCup
    <div className="grow grid gap-2">
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "playerPushing") {
            setSelectedPenaltyCall("playerPushing");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={selectedPenaltyCall === "playerPushing" ? "Pushing activated" : "Pushing"}
        legal={
          game.state === "playing" || game.state === "set" || game.state === "ready"
            ? selectedPenaltyCall === "pickedUp" || selectedPenaltyCall === "ballHolding"
              ? false
              : true
            : false
        }
      />
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "pickedUp") {
            setSelectedPenaltyCall("pickedUp");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={selectedPenaltyCall === "pickedUp" ? "Pick Up activated" : "Pick Up / Incapable"}
        legal={
          selectedPenaltyCall === "playerPushing" || selectedPenaltyCall === "ballHolding"
            ? false
            : true
        }
      />
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "ballHolding") {
            setSelectedPenaltyCall("ballHolding");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={
          selectedPenaltyCall === "ballHolding"
            ? "Ball Manipulation activated"
            : "Ball Manipulation"
        }
        legal={
          game.state === "playing" || game.state === "ready"
            ? selectedPenaltyCall === "playerPushing" || selectedPenaltyCall === "pickedUp"
              ? false
              : true
            : false
        }
      />
    </div>
  );
};

export default PenaltyPanel;
