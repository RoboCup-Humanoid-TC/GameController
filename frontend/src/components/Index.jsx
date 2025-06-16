import { useEffect, useState } from "react";
import Launcher from "./Launcher";
import Main from "./Main";
import HL_Main from "./HL_Main";
import HL_Launcher from "./HL_Launcher"
import { getLeague, getLaunchData } from "../api"

const Index = () => {
  const [launched, setLaunched] = useState(false);
  const [league, setLeague] = useState(null);
  
  useEffect(() => {
    getLeague().then((data) => {
      setLeague(data);
    });
  }, []);

  if (launched && league == false) {
    return <Main />;
  } else if (launched && league == true){
    return <HL_Main />;
  } else if (league == false) {
    return <Launcher setLaunched={setLaunched} />;
  } else if (league == true) {
    return <HL_Launcher setLaunched={setLaunched}/>;
  } else {
    return 
  }
};

export default Index;
