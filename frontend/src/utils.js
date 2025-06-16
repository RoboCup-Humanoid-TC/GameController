export const formatMMSS = (timer) => {
  const getDuration = (duration) => {
    return duration[0] + (duration[1] > 0 ? 1 : 0);
  };
  const rawSeconds = timer.started ? getDuration(timer.started.remaining) : 0;
  const sign = rawSeconds < 0 ? "-" : "";
  var seconds = Math.abs(rawSeconds);
  var minutes = Math.floor(seconds / 60);
  seconds -= minutes * 60;
  if (minutes < 10) {
    minutes = "0" + minutes;
  }
  if (seconds < 10) {
    seconds = "0" + seconds;
  }
  return sign + minutes + ":" + seconds;
};
