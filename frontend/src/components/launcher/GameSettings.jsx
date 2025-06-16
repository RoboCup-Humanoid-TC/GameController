import TeamSettings from "./TeamSettings";

const GameSettings = ({ teams, game, setGame }) => {
  return (
    <div className="flex flex-col items-center gap-2">
      <div className="flex flex-row items-center gap-2">
        <label htmlFor="play-off">Play-off</label>
        <input
          type="checkbox"
          checked={game.long}
          id="play-off"
          onChange={(e) => setGame({ ...game, long: e.target.checked })}
        />
      </div>
      <div
        className={`flex ${
          game.sideMapping === "homeDefendsRightGoal" ? "flex-row-reverse" : "flex-row"
        } gap-6`}
      >
        {["home", "away"].map((side) => {
          return (
            <div className="flex flex-col items-center gap-2" key={side}>
              <div className="flex flex-row items-center gap-2">
                <label htmlFor={`kick-off-${side}`}>Kick-off for {side} team</label>
                <input
                  type="radio"
                  checked={game.kickOffSide === side}
                  id={`kick-off-${side}`}
                  value={side}
                  onChange={(e) => {
                    setGame({ ...game, kickOffSide: e.target.value });
                  }}
                />
              </div>
              <TeamSettings
                teams={teams}
                team={game.teams[side]}
                setTeam={(team) =>
                  setGame({
                    ...game,
                    teams: { ...game.teams, [side]: team },
                  })
                }
              />
            </div>
          );
        })}
      </div>
      <div className="flex flex-row items-center gap-2">
        <label htmlFor="mirror">Mirror (home team starts on right side)</label>
        <input
          type="checkbox"
          checked={game.sideMapping === "homeDefendsRightGoal"}
          id="mirror"
          onChange={(e) =>
            setGame({
              ...game,
              sideMapping: e.target.checked ? "homeDefendsRightGoal" : "homeDefendsLeftGoal",
            })
          }
        />
      </div>
    </div>
  );
};

export default GameSettings;
