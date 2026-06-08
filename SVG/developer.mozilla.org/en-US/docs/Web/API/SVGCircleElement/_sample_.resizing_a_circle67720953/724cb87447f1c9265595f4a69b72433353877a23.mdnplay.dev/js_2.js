function clickCircle() {
  const circle = document.getElementById("circle");
  // Randomly determine if the circle radius will increase or decrease
  const change = Math.random() > 0.5 ? 10 : -10;
  // Clamp the circle radius to a minimum of 10 and a maximum of 250,
  // so it won't disappear or get bigger than the viewport
  const newValue = Math.min(Math.max(circle.r.baseVal.value + change, 10), 250);
  circle.setAttribute("r", newValue);
}
