const unit = [
  "unknown",
  "",
  "%",
  "em",
  "ex",
  "px",
  "cm",
  "mm",
  "in",
  "pt",
  "pc",
];
const circle = document.getElementById("circle");
const log = document.getElementById("log");
const baseValue = circle.r.baseVal.value;
const baseUnit = unit[circle.r.baseVal.unitType];

log.textContent = `The 'circle.r.baseVal' is ${baseValue} (in ${baseUnit}).`;
