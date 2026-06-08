window.addEventListener("mouseup", (e) => {
  // Let's pick a random color between #000000 and #FFFFFF
  const color = Math.round(Math.random() * 0xffffff);

  // Let's format the color to fit CSS requirements
  const fill = `#${color.toString(16).padStart(6, "0")}`;

  // Let's apply our color in the
  // element we actually clicked on
  e.target.style.fill = fill;
});
