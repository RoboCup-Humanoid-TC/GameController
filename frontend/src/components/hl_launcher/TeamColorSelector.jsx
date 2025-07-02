const TeamColorSelector = ({ colors, color, setColor }) => {
  return (
    <select value={color} onChange={(e) => setColor(e.target.value)}>
      {colors.map((color) => (
        <option key={color} value={color}>
          {color}
        </option>
      ))}
    </select>
  );
};

export default TeamColorSelector;
