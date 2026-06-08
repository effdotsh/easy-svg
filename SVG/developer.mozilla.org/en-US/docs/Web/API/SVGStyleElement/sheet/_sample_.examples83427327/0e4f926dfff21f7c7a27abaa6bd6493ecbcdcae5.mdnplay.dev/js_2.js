const svg = document.querySelector("svg");
// Create the `style` element in the SVG namespace
const style = document.createElementNS("http://www.w3.org/2000/svg", "style");
const node = document.createTextNode("circle { fill: red; }");
svg.appendChild(style);
style.appendChild(node);

// Log the sheet associated with this new element.
const log = document.getElementById("log");
log.value = `${style.sheet} with rules[0].cssText:\n ${style.sheet.rules[0].cssText}`;
