const unitType = ["unknown", "userSpaceOnUse", "objectBoundingBox"];

const mask = document.getElementById("mask2");
const log = document.getElementById("log");

function displayLog() {
  const baseValue = unitType[mask.maskContentUnits.baseVal];
  const animValue = unitType[mask.maskContentUnits.animVal];
  log.textContent = `The top-right 'maskContentUnits.baseVal' coord system : ${baseValue}\n`;
  log.textContent += `The top-right 'maskContentUnits.animVal' coord system : ${animValue}`;
  requestAnimationFrame(displayLog);
}
displayLog();
