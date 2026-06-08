const clipPath1 = document.getElementById("clip1");
const log = document.getElementById("log");

function displayLog() {
  const animValue = clipPath1.clipPathUnits.animVal;
  const baseValue = clipPath1.clipPathUnits.baseVal;
  log.textContent = `The 'clipPathUnits.animVal' is ${animValue}.\n`;
  log.textContent += `The 'clipPathUnits.baseVal' is ${baseValue}.`;
  requestAnimationFrame(displayLog);
}

displayLog();
