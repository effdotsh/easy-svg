const unitType = ["unknown", "userSpaceOnUse", "objectBoundingBox"];

const mask = document.getElementById("mask2");
const log = document.getElementById("log");

function displayLog() {
  const baseValue = unitType[mask.maskUnits.baseVal];
  const animValue = unitType[mask.maskUnits.animVal];
  log.textContent = `The top-right 'maskUnits.baseVal' coord system : ${baseValue}\n`;
  log.textContent += `The top-right 'maskUnits.animVal' coord system : ${animValue}`;
  requestAnimationFrame(displayLog);
}
displayLog();
