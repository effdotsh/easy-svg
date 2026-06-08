const svg = document.querySelector("svg");
// Create the `style` element in the SVG namespace
const style = document.createElementNS("http://www.w3.org/2000/svg", "style");
const node = document.createTextNode("circle { fill: red; }");
svg.appendChild(style);
style.appendChild(node);

const button = document.querySelector("button");

function setButtonText() {
  button.textContent = `Media: ${style.media} (Width: ${window.innerWidth})`;
}
setButtonText();

addEventListener("resize", () => {
  setButtonText();
});

button.addEventListener("click", () => {
  style.media = "all and (min-width: 700px)";
  setButtonText();
});
