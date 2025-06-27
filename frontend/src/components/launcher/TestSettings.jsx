const TestSettings = ({ test, setTest }) => {
  return (
    <div className="flex flex-col items-center gap-2">
      <details>
        <summary>Testing</summary>
        <div className="flex flex-row items-center gap-2">
          <label htmlFor="no-delay">No Delay</label>
          <input
            type="checkbox"
            checked={test.noDelay}
            id="no-delay"
            onChange={(e) => setTest({ ...test, noDelay: e.target.checked })}
          />
        </div>
        <div className="flex flex-row items-center gap-2">
          <label htmlFor="penalty-shootout">Penalty Shoot-out</label>
          <input
            type="checkbox"
            checked={test.penaltyShootout}
            id="penalty-shootout"
            onChange={(e) => setTest({ ...test, penaltyShootout: e.target.checked })}
          />
        </div>
        <div className="flex flex-row items-center gap-2">
          <label htmlFor="unpenalize">Unpenalize</label>
          <input
            type="checkbox"
            checked={test.unpenalize}
            id="unpenalize"
            onChange={(e) => setTest({ ...test, unpenalize: e.target.checked })}
          />
        </div>
      </details>
    </div>
  );
};

export default TestSettings;
