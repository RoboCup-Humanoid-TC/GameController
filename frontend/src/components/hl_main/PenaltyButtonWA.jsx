import { applyAction } from "../../hl_api.js";

const PenaltyButtonWA = ({ label, legal, action, active }) => {
  return (
    <button
      className={`rounded-md border border-gray-600 ${
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

export default PenaltyButtonWA;
