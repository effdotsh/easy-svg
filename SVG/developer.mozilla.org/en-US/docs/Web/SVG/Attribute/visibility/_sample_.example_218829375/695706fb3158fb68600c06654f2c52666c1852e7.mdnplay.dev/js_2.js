const button = document.querySelector("button");
button.addEventListener("click", (evt) => {
  button
    .querySelector("svg > path:nth-of-type(1)")
    .classList.toggle("invisible");
  button
    .querySelector("svg > path:nth-of-type(2)")
    .classList.toggle("invisible");
});
