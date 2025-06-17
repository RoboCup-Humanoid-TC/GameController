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
      {/* <PenaltyButton
        key={"test"}
        label={"test"}
        legal={false}
        onClick={applyAction({
          type: "manipulateSecState",
          args: {i: 8},
        })}
        selected={false}
        /> */}
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "hlPushing") {
            setSelectedPenaltyCall("hlPushing");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={selectedPenaltyCall === "hlPushing" ? "Pushing activated" : "Pushing"}
        legal={
          selectedPenaltyCall === "hlPickUp" || selectedPenaltyCall === "hlBallManipulation"
            ? false
            : true
        }
      />
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "hlPickUp") {
            setSelectedPenaltyCall("hlPickUp");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={selectedPenaltyCall === "hlPickUp" ? "Pick Up activated" : "Pick Up"}
        legal={
          selectedPenaltyCall === "hlPushing" || selectedPenaltyCall === "hlBallManipulation"
            ? false
            : true
        }
      />
      <PenaltyButtonWA
        action={function () {
          if (selectedPenaltyCall !== "hlBallManipulation") {
            setSelectedPenaltyCall("hlBallManipulation");
          } else {
            setSelectedPenaltyCall(null);
          }
        }}
        label={
          selectedPenaltyCall === "hlBallManipulation"
            ? "Ball Manipulation activated"
            : "Ball Manipulation"
        }
        legal={
          selectedPenaltyCall === "hlPushing" || selectedPenaltyCall === "hlPickUp" ? false : true
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
