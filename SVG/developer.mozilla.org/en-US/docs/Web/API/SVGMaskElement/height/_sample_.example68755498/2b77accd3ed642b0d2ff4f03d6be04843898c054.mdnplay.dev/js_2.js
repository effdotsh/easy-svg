const mask = document.getElementById("mask");

function displayLog() {
  const animValue = mask.height.animVal.value;
  const baseValue = mask.height.baseVal.value;
  log.textContent = `The 'height.animVal' is ${animValue}.\n`;
  log.textContent += `The 'height.baseVal' is ${baseValue}.`;
  requestAnimationFrame(displayLog);
}
displayLog();
