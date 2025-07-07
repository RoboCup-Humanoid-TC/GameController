import { useState } from "react";
import ActionButton from "./ActionButton";
import TeamPenaltyButton from "./TeamPenaltyButton.jsx";
import PlayerButton from "./PlayerButton";
import * as actions from "../../hl_actions.js";
import { applyAction } from "../../hl_api.js";

const textClasses = {
  red: "text-red-600",
  blue: "text-blue-600",
  yellow: "text-yellow-400",
  black: "text-black",
  white: "text-black",
  green: "text-green-600",
  orange: "text-orange-400",
  purple: "text-purple-600",
  brown: "text-amber-800",
  gray: "text-gray-600",
};

const TeamHeader = ({ color, isKicking, name }) => {
  return (
    <div className="flex items-center justify-center gap-2">
      <svg
        className={`${isKicking ? "" : "invisible"} text-black`}
        fill="currentColor"
        height="14"
        width="14"
      >
        <circle cx="7" cy="7" r="7" />
      </svg>
      <h1 className={`text-center text-2xl font-semibold ${textClasses[color]}`}>{name}</h1>
    </div>
  );
};

const TeamStats = ({ game, side, sign, team }) => {
  return (
    <dl className="flex-1">
      <dt className="sr-only">Score</dt>
      <dd
        className={`font-bold text-4xl ${sign > 0 ? "text-right" : "text-left"} tabular-nums ${
          team.illegalCommunication ? "text-fuchsia-400" : ""
        }`}
      >
        {team.score}
      </dd>

      {game.phase === "penaltyShootout" ? (
        <>
          <dt>Shot{game.kickingSide === side ? "" : "s"}:</dt>
          <dd className="tabular-nums text-right">{team.penaltyShot}</dd>
        </>
      ) : (
        <></>
      )}
    </dl>
  );
};

const FreeKickButtons = ({ game, legalTeamActions, side, sign }) => {
  return (
    <div className="h-1/4">
      <div
        className={`flex items-center w-full h-1/3 ${sign > 0 ? "flex-row" : "flex-row-reverse"}`}
      >
        <TeamPenaltyButton
          action={{ type: "hlSetPlay", args: { side: side, setPlay: "goalKick" } }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Goal Kick"
              : game.secState.state == "goalKick" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Goal Kick"
          }
          legal={legalTeamActions[actions.GOAL_KICK]}
        />
        <TeamPenaltyButton
          action={{ type: "hlSetPlay", args: { side: side, setPlay: "throwIn" } }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Throw in"
              : game.secState.state == "throwIn" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Throw in"
          }
          legal={legalTeamActions[actions.THROW_IN]}
        />
        <TeamPenaltyButton
          action={{ type: "hlSetPlay", args: { side: side, setPlay: "cornerKick" } }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Corner Kick"
              : game.secState.state == "cornerKick" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Corner Kick"
          }
          legal={legalTeamActions[actions.CORNER_KICK]}
        />
      </div>
      <div
        className={`flex items-center w-full h-1/3 ${sign > 0 ? "flex-row" : "flex-row-reverse"}`}
      >
        <TeamPenaltyButton
          action={{ type: "hlSetPlay", args: { side: side, setPlay: "penaltyKick" } }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Penalty Kick"
              : game.secState.state == "penaltyKick" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Penalty Kick"
          }
          legal={legalTeamActions[actions.PENALTY_KICK]}
        />
        <TeamPenaltyButton
          action={{
            type: "hlSetPlay",
            args: { side: side, setPlay: "directFreeKick" },
          }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Direct Free Kick"
              : game.secState.state == "directFreeKick" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Direct Free Kick"
          }
          legal={legalTeamActions[actions.DIRECT_FREE_KICK]}
        />
        <TeamPenaltyButton
          action={{
            type: "hlSetPlay",
            args: { side: side, setPlay: "indirectFreeKick" },
          }}
          active={false}
          label={
            game.secState.state == "normal"
              ? "Indirect Free Kick"
              : game.secState.state == "indirectFreeKick" && game.secState.side == side
              ? game.secState.phase == 0
                ? "Placement"
                : game.secState.phase == 1
                ? "!Placement"
                : game.secState.phase == 2
                ? "Execute"
                : ""
              : "Indirect Free Kick"
          }
          legal={legalTeamActions[actions.INDIRECT_FREE_KICK]}
        />
      </div>
      <div
        className={`flex items-center w-full h-1/3 ${sign > 0 ? "flex-row" : "flex-row-reverse"}`}
      >
        <ActionButton
          action={{ type: "hlRetake", args: { side: side } }}
          active={false}
          label="Retake"
          legal={legalTeamActions[actions.RETAKE]}
        />
        <ActionButton
          action={{ type: "hlAbort", args: { side: side } }}
          active={false}
          label="Abort"
          legal={legalTeamActions[actions.ABORT]}
        />
      </div>
    </div>
  );
};

const TeamPanel = ({
  connectionStatus,
  game,
  legalPenaltyActions,
  legalTeamActions,
  params,
  selectedPenaltyCall,
  setSelectedPenaltyCall,
  side,
  sign,
  teamNames,
}) => {
  // This indicates whether we are currently in the process of substitution or player selection.
  const [substitute, setSubstitute] = useState(false);
  // This doubles as carrying the number of the player which is substituted (out) for normal
  // substitutions, and a boolean indicating whether the penalty shoot-out player we are selecting
  // is a field player (false) or a goalkeeper (true). If the substitute state is false, this
  // should always be null.
  const [substitutedPlayer, setSubstitutedPlayer] = useState(null);

  // Thus, the allowed combinations of substituted/substitutedPlayer are:
  // substitute === false && substitutedPlayer === null
  //   -> no substitution / player selecting going on
  // substitute === true && !penaltyShootout && substitutedPlayer === null
  //   -> selecting the player going out
  // substitute === true && !penaltyShootout && substitutedPlayer === 1..20
  //   -> selecting the player coming in
  // substitute === true && penaltyShootout && substitutedPlayer === null
  //   -> selecting the player type (goalkeeper or field player)
  // substitute === true && penaltyShootout && substitutedPlayer === false
  //   -> selecting the player for this shot, wearing a field player jersey
  // substitute === true && penaltyShootout && substitutedPlayer === true
  //   -> selecting the player for this shot, wearing a goalkeeper jersey

  // This is terrible code, I know.
  const selectingPlayerIn = substitute && substitutedPlayer != null;
  const selectingPlayerTypePSO =
    substitute && game.phase === "penaltyShootout" && substitutedPlayer === null;
  const selectingPlayerInPSO =
    substitute && game.phase === "penaltyShootout" && substitutedPlayer != null;

  const team = game.teams[side];
  const teamConnectionStatus = connectionStatus[side];
  const teamParams = params.game.teams[side];
  const handlePlayerClick = (player) => {
    if (selectingPlayerInPSO) {
      applyAction({
        type: "selectPenaltyShotPlayer",
        args: { side: side, player: player.number, goalkeeper: substitutedPlayer === true },
      });
      setSubstitute(false);
      setSubstitutedPlayer(null);
    } else if (substitute) {
      applyAction({
        type: "hlSubstitute",
        args: { side: side, player: player.number },
      });
      setSubstitute(false);
    } else if (selectedPenaltyCall != null) {
      applyAction({
        type: "hlPenalize",
        args: {
          side: side,
          player: player.number,
          penalty: selectedPenaltyCall,
        },
      });
      setSelectedPenaltyCall(null);
    } else {
      applyAction({
          type: "hlUnpenalize",
          args: { side: side, player: player.number},
        });
    }
  };

  return (
    <div className="w-1/3 flex flex-col gap-2">
      <TeamHeader
        color={teamParams.fieldPlayerColor}
        isKicking={game.kickingSide === side}
        name={teamNames[side]}
      />
      <div className={`flex ${sign > 0 ? "flex-row" : "flex-row-reverse"} gap-2`}>
        <div className="flex-1 flex flex-col gap-2">
          <ActionButton
            action={() => {
              setSubstitute(!substitute);
              setSubstitutedPlayer(null);
            }}
            active={substitute}
            label={game.phase === "penaltyShootout" ? "Select" : "Substitute"}
            legal={true}
          />
          {game.phase === "penaltyShootout" || game.state != "playing" ? (
            <ActionButton
              action={{ type: "timeout", args: { side: side } }}
              label={game.secState.state == "timeout" ? "!Timeout" : "Timeout"}
              legal={legalTeamActions[actions.TIMEOUT]}
            />
          ) : (
            <ActionButton
              action={{ type: "timeout", args: { side: side } }}
              label={game.secState.state == "timeout" ? "!Timeout" : "Timeout"}
              legal={legalTeamActions[actions.TIMEOUT]}
            />
          )}
        </div>
        <div className="flex-1">
          <ActionButton
            action={{ type: "goal", args: { side: side } }}
            label="Goal"
            legal={legalTeamActions[actions.GOAL]}
          />
        </div>
        <TeamStats game={game} side={side} sign={sign} team={team} />
      </div>
      <div className="grow flex flex-col gap-2 overflow-auto">
        {team.players
          .slice(0, 6)
          .map((player, index) => {
            return {
              ...player,
              connectionStatus: teamConnectionStatus[index],
              number: index + 1,
            };
          })
          .map((player) => (
            <PlayerButton
              key={player.number}
              color={
                (
                  selectingPlayerInPSO
                    ? substitutedPlayer === true
                    : (selectingPlayerIn ? substitutedPlayer : player.number) === team.goalkeeper
                )
                  ? teamParams.fieldPlayerColor // later change this back to goalkeeperColor if hl teams have different colors for goalkeepers
                  : teamParams.fieldPlayerColor
              }
              legal={
                substitute ||
                player.penalty !== "noPenalty" ||
                actions.isPenaltyCallLegalForPlayer(
                  legalPenaltyActions,
                  side,
                  player.number,
                  selectedPenaltyCall
                )
              }
              sign={sign}
              onClick={() => handlePlayerClick(player)}
              player={player}
              side={side}
            />
          ))}
      </div>
      <FreeKickButtons game={game} legalTeamActions={legalTeamActions} side={side} sign={sign} />
    </div>
  );
};

export default TeamPanel;
