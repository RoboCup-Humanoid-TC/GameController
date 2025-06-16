import PenaltyButton from "./PenaltyButton";
import { isPenaltyCallLegal, PENALTIES } from "../../actions.js";

const PenaltyPanel = ({ legalPenaltyActions, selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    <div className="grow grid grid-cols-2 gap-2">
      {PENALTIES.map((penalty, index) => (
        <PenaltyButton
          key={penalty[1]}
          label={penalty[0]}
          legal={isPenaltyCallLegal(legalPenaltyActions, index)}
          onClick={() => setSelectedPenaltyCall(selectedPenaltyCall === index ? null : index)}
          selected={selectedPenaltyCall === index}
        />
      ))}
    </div>
  );
};

export default PenaltyPanel;
