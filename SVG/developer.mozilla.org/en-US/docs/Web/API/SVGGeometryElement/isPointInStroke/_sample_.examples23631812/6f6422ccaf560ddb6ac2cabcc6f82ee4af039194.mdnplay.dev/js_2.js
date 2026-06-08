const svg = document.getElementsByTagName("svg")[0];
const circle = document.getElementById("circle");
const points = [
  [10, 10],
  [40, 30],
  [70, 40],
  [15, 75],
  [83, 83],
];

for (const point of points) {
  let isPointInStroke;

  try {
    const pointObj = { x: point[0], y: point[1] };
    isPointInStroke = circle.isPointInStroke(pointObj);
  } catch {
    // Fallback for browsers that don't support DOMPoint as an argument
    const pointObj = svg.createSVGPoint();
    pointObj.x = point[0];
    pointObj.y = point[1];
    isPointInStroke = circle.isPointInStroke(pointObj);
  }

  console.log(`Point at ${point[0]},${point[1]}: ${isPointInStroke}`);

  const pointEl = document.createElementNS(
    "http://www.w3.org/2000/svg",
    "circle",
  );
  pointEl.cx.baseVal.value = point[0];
  pointEl.cy.baseVal.value = point[1];
  pointEl.r.baseVal.value = 5;
  const pathEl = document.createElementNS("http://www.w3.org/2000/svg", "path");
  if (isPointInStroke) {
    pointEl.setAttribute("fill", "rgb(0 170 0 / 50%)");
    pointEl.setAttribute("stroke", "rgb(0 170 0)");
    pathEl.setAttribute("stroke", "rgb(0 170 0)");
    pathEl.setAttribute("d", `M ${point[0] - 5} ${point[1]} h 10 m -5 -5 v 10`);
  } else {
    pointEl.setAttribute("fill", "rgb(170 0 0 / 50%)");
    pointEl.setAttribute("stroke", "rgb(170 0 0)");
    pathEl.setAttribute("stroke", "rgb(170 0 0)");
    pathEl.setAttribute("d", `M ${point[0] - 5} ${point[1]} h 10`);
  }
  svg.append(pointEl, pathEl);
}
