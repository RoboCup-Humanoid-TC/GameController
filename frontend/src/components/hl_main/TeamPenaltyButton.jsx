import { applyAction } from "../../api.js";

const TeamPenaltyButton = ({ action, active, label, legal }) => {
  return (
    <button
      className={`w-1/3 h-full rounded-md border border-gray-600 ${
        active ? "bg-gray-300" : legal ? "" : "text-gray-500 bg-gray-100"
      }`}
      disabled={!legal}
      onClick={
        action ? (typeof action === "function" ? action : () => applyAction(action)) : () => {}
      }
    >
      {label}
    </button>
  );
};

export default TeamPenaltyButton;
