const svg = document.querySelector("svg");
const style = svg.getElementById("circle_style_id");

// Get logging text area
const log = document.getElementById("log");

function setLogText() {
  //Log current values of properties
  log.value = `style.media: ${style.media} (frame width: ${window.innerWidth})\n`; // 'all' by default
  log.value += `style.title: ${style.title}\n`; // no default value
  log.value += `style.disabled: ${style.disabled}\n`; // 'false' by default
  log.value += `style.type: ${style.type}\n`; // deprecated (do not use)
  log.value += `style.sheet.rules[0].cssText: ${style.sheet.rules[0].cssText}\n`;
}

// Log initial property values
setLogText();

// Log when the frame resizes
addEventListener("resize", () => {
  setLogText();
});

const button = document.querySelector("button");

button.addEventListener("click", () => {
  style.disabled = !style.disabled;
  button.textContent = style.disabled ? "Enable" : "Disable";

  // Log after button presses
  setLogText();
});
