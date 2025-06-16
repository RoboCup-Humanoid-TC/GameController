const PenaltyButton = ({ label, legal, onClick, selected }) => {
  return (
    <button
      className={`rounded-md border border-gray-600 ${
        selected ? "bg-gray-300" : legal ? "" : "text-gray-500 bg-gray-100"
      }`}
      disabled={!legal}
      onClick={onClick}
    >
      {label}
    </button>
  );
};

export default PenaltyButton;
