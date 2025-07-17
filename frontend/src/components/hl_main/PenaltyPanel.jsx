import PenaltyButton from "./PenaltyButton";
import { isPenaltyCallLegal, PENALTIES } from "../../hl_actions";

const PenaltyPanel = ({ legalPenaltyActions, selectedPenaltyCall, setSelectedPenaltyCall }) => {
  return (
    <div className="grow grid gap-2">
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
