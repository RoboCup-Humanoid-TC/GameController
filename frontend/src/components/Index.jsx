import { useEffect, useState } from "react";
import Launcher from "./Launcher";
import Main from "./Main";
import HL_Main from "./HL_Main";
import HL_Launcher from "./HL_Launcher";
import { getLaunchData } from "../api";

const Index = () => {
  const [launched, setLaunched] = useState(false);
  const [league, setLeague] = useState(null);

  useEffect(() => {
    getLaunchData().then((data) => {
      setLeague(data.defaultSettings.league.league);
    });
  }, []);

  if (launched && league === "spl") {
    return <Main />;
  } else if (launched && league === "humanoid") {
    return <HL_Main />;
  } else if (league === "spl") {
    return <Launcher setLaunched={setLaunched} />;
  } else if (league === "humanoid") {
    return <HL_Launcher setLaunched={setLaunched} />;
  } else {
    return;
  }
};

export default Index;
