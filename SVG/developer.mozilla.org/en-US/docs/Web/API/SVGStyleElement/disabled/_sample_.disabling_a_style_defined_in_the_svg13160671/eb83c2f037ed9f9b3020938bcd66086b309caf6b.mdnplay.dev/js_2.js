const svg = document.querySelector("svg");
const style = svg.getElementById("circle_style_id");
style.disabled = true;

const button = document.querySelector("button");

button.addEventListener("click", () => {
  style.disabled = !style.disabled;
  button.textContent = style.disabled ? "Enable" : "Disable";
});
