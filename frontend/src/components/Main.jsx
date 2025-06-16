import { useEffect, useState } from "react";
import CenterPanel from "./main/CenterPanel";
import TeamPanel from "./main/TeamPanel";
import UndoPanel from "./main/UndoPanel";
import {
  getActions,
  extractGameActions,
  extractPenaltyActions,
  extractTeamActions,
  extractUndoActions,
  isPenaltyCallLegal,
  NUM_OF_ACTIONS,
} from "../actions.js";
import { getLaunchData, declareActions, listenForState, syncWithBackend } from "../api.js";

const Main = () => {
  const [connectionStatus, setConnectionStatus] = useState(null);
  const [game, setGame] = useState(null);
  const [legalActions, setLegalActions] = useState(null);
  const [params, setParams] = useState(null);
  const [selectedPenaltyCall, setSelectedPenaltyCall] = useState(null);
  const [teamNames, setTeamNames] = useState(null);
  const [undoActions, setUndoActions] = useState(null);

  useEffect(() => {
    if (
      legalActions != null &&
      selectedPenaltyCall != null &&
      !isPenaltyCallLegal(extractPenaltyActions(legalActions), selectedPenaltyCall)
    ) {
      setSelectedPenaltyCall(null);
    }
  }, [legalActions]);

  useEffect(() => {
    const thePromise = (async () => {
      const unlisten = await listenForState((state) => {
        setConnectionStatus(state.connectionStatus);
        setGame(state.game);
        setLegalActions(state.legalActions);
        setUndoActions(state.undoActions);
      });
      // listen must have completed before starting the next call because the core may send a state
      // event once syncWithBackend is called that must not be missed.
      const params = await syncWithBackend();
      setParams(params);
      const teams = (await getLaunchData()).teams;
      setTeamNames(
        Object.fromEntries(
          Object.entries(params.game.teams).map(([side, teamParams]) => [
            side,
            teams.find((team) => team.number === teamParams.number).name,
          ])
        )
      );
      // syncWithBackend must have completed before the next call because declareActions fails if
      // certain things have not been initialized that are only guaranteed to be initialized after
      // syncWithBackend.
      declareActions(getActions());
      return unlisten;
    })();
    return () => {
      thePromise.then((unlisten) => unlisten());
    };
  }, []);

  if (
    connectionStatus != null &&
    game != null &&
    legalActions != null &&
    legalActions.length == NUM_OF_ACTIONS &&
    params != null &&
    teamNames != null &&
    undoActions != null
  ) {
    const mirror = game.sides === "homeDefendsRightGoal";
    return (
      <div className="flex flex-col w-screen h-screen p-2 gap-4">
        <div
          className={`grow h-[calc(100%-3.5rem)] flex ${
            mirror ? "flex-row-reverse" : "flex-row"
          } gap-4`}
        >
          <TeamPanel
            connectionStatus={connectionStatus}
            game={game}
            legalPenaltyActions={extractPenaltyActions(legalActions)}
            legalTeamActions={extractTeamActions(legalActions, "home")}
            params={params}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
            side="home"
            sign={mirror ? -1 : 1}
            teamNames={teamNames}
          />
          <CenterPanel
            game={game}
            legalGameActions={extractGameActions(legalActions)}
            legalPenaltyActions={extractPenaltyActions(legalActions)}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
          />
          <TeamPanel
            connectionStatus={connectionStatus}
            game={game}
            legalPenaltyActions={extractPenaltyActions(legalActions)}
            legalTeamActions={extractTeamActions(legalActions, "away")}
            params={params}
            selectedPenaltyCall={selectedPenaltyCall}
            setSelectedPenaltyCall={setSelectedPenaltyCall}
            side="away"
            sign={mirror ? 1 : -1}
            teamNames={teamNames}
          />
        </div>
        <UndoPanel undoActions={undoActions} legalUndoActions={extractUndoActions(legalActions)} />
      </div>
    );
  } else {
    return <></>;
  }
};

export default Main;
