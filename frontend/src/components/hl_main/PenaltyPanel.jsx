import PenaltyButtonWA from "./PenaltyButtonWA";
import { isPenaltyCallLegal } from "../../hl_actions";

const PenaltyPanel = ({ game, selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    <div className="grow grid gap-2">
      {/* {PENALTIES.map((penalty, index) => (
        <PenaltyButton
          key={penalty[1]}
          label={penalty[0]}
          legal={isPenaltyCallLegal(legalPenaltyActions, index)}
          onClick={() => setSelectedPenaltyCall(selectedPenaltyCall === index ? null : index)}
          selected={selectedPenaltyCall === index}
        />
      ))} */}
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
          selectedPenaltyCall === "pickedUp" || selectedPenaltyCall === "ballHolding" ? false : true
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
        label={selectedPenaltyCall === "pickedUp" ? "Pick Up activated" : "Pick Up"}
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
          selectedPenaltyCall === "playerPushing" || selectedPenaltyCall === "pickedUp"
            ? false
            : true
        }
      />
      {/* <PenaltyButtonWA
      action={null}
      label={""}
      legal={false}
      disabled={true}
    /> */}
    </div>
  );
};

export default PenaltyPanel;
