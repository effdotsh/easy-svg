document.getElementById("restart").addEventListener("click", (evt) => {
  document.querySelectorAll("animate").forEach((element) => {
    element.beginElement();
  });
});
