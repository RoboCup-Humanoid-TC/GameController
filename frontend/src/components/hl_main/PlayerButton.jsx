import { formatMMSS } from "../../utils.js";
import { applyAction } from "../../api.js";

const bgClasses = {
  red: "bg-red-100",
  blue: "bg-blue-100",
  yellow: "bg-yellow-100",
  black: "bg-white",
  white: "bg-white",
  green: "bg-green-100",
  orange: "bg-orange-100",
  purple: "bg-purple-100",
  brown: "bg-amber-100",
  gray: "bg-gray-200",
};

const penaltyDescriptions = {
  noPenalty: "No Penalty",
  substitute: "Substitute",
  pickedUp: "Picked Up",
  illegalPositionInSet: "Illegal Position",
  illegalPosition: "Illegal Position",
  motionInSet: "Motion in Set",
  fallenInactive: "Fallen / Inactive",
  localGameStuck: "Local Game Stuck",
  ballHolding: "Ball Holding",
  playerStance: "Player Stance",
  playerPushing: "Pushing",
  playingWithArmsHands: "Arms / Hands",
  leavingTheField: "Leaving the Field",
};

const PlayerButton = ({ color, legal, sign, onClick, player, side }) => {
  const addCard = (color) => {
    applyAction({
      type: "hlAddCard",
      args: {
        side: side,
        player: player.number,
        card: color,
      },
    });
  };

  const shouldFlash =
    player &&
    player.penalty != "noPenalty" &&
    player.penalty != "substitute" &&
    player.penalty != "motionInSet" &&
    (player.penaltyTimer.started
      ? player.penaltyTimer.started.remaining[0] < 10
      : player.penalty != "pickedUp" &&
        player.penalty != "playerPushing" &&
        player.penalty != "ballHolding");
  return (
    <div className="flex items-center">
      <button
        className={`grow rounded-md border border-gray-600 ${bgClasses[color]} ${
          shouldFlash ? "animate-flash-bg" : ""
        } ${legal ? "" : "text-gray-500"}`}
        disabled={!legal}
        onClick={onClick}
        style={{ width: "70%" }}
      >
        <div
          className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} items-center gap-4 px-4`}
        >
          <div className="grow flex flex-col">
            <p>{color.charAt(0).toUpperCase() + color.slice(1)}</p>
            {player ? (
              <p
                className={
                  player.penaltyTimer.started
                    ? "tabular-nums"
                    : player.penalty === "noPenalty"
                    ? "invisible"
                    : ""
                }
              >
                {player.penaltyTimer.started
                  ? formatMMSS(player.penaltyTimer)
                  : penaltyDescriptions[player.penalty]}
              </p>
            ) : (
              <></>
            )}
          </div>
          {player ? (
            <>
              <svg
                className={
                  player.connectionStatus >= 2
                    ? "text-green-600"
                    : player.connectionStatus >= 1
                    ? "text-yellow-400"
                    : "text-red-600"
                }
                fill="currentColor"
                height="14"
                width="14"
              >
                <circle cx="7" cy="7" r="7" />
              </svg>
              <p className="text-3xl tabular-nums">{player.number}</p>
            </>
          ) : (
            <></>
          )}
        </div>
      </button>
      <button
        className={"grow rounded-md border border-red-500 bg-red-500"}
        disabled={false}
        onClick={() => addCard("red")}
        style={{ width: "10%", height: "100%" }}
      >
        {player.cards["red"]}
      </button>
      <button
        className={"grow rounded-md border border-yellow-300 bg-yellow-300"}
        disabled={false}
        onClick={() => addCard("yellow")}
        style={{ width: "10%", height: "100%", margin: 0, alignContent: "center" }}
      >
        {player.cards["yellow"]}
      </button>
      <button
        className={"grow rounded-md border border-blue-400 bg-blue-400"}
        disabled={false}
        onClick={() => addCard("warning")}
        style={{ width: "10%", height: "100%" }}
      >
        {player.cards["warning"]}
      </button>
    </div>
  );
};

export default PlayerButton;
