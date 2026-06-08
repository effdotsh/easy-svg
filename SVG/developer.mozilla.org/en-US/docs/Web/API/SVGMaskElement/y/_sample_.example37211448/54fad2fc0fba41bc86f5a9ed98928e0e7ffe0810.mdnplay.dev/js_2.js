const mask = document.getElementById("mask");

function displayLog() {
  const animValue = mask.y.animVal.value;
  const baseValue = mask.y.baseVal.value;
  log.textContent = `The 'y.animVal' is ${animValue}.\n`;
  log.textContent += `The 'y.baseVal' is ${baseValue}.`;
  requestAnimationFrame(displayLog);
}
displayLog();
