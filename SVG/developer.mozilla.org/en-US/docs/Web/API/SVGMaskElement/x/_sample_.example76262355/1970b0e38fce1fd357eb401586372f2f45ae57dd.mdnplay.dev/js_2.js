const mask = document.getElementById("mask");

function displayLog() {
  const animValue = mask.x.animVal.value;
  const baseValue = mask.x.baseVal.value;
  log.textContent = `The 'x.animVal' is ${animValue}.\n`;
  log.textContent += `The 'x.baseVal' is ${baseValue}.`;
  requestAnimationFrame(displayLog);
}
displayLog();
