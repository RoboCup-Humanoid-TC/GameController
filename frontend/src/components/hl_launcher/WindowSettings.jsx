const WindowSettings = ({ window, setWindow }) => {
  return (
    <div className="flex flex-row items-center gap-2">
      <label htmlFor="fullscreen">Fullscreen</label>
      <input
        type="checkbox"
        checked={window.fullscreen}
        id="fullscreen"
        onChange={(e) => setWindow({ ...window, fullscreen: e.target.checked })}
      />
    </div>
  );
};

export default WindowSettings;
