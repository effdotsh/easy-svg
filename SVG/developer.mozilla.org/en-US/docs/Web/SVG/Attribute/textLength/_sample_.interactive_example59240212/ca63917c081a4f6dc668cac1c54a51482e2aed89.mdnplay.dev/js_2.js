const widthSlider = document.getElementById("widthSlider");
const widthDisplay = document.getElementById("widthDisplay");
const textElement = document.getElementById("hello");
const baseLength = Math.floor(textElement.textLength.baseVal.value);

widthSlider.value = baseLength;

widthSlider.addEventListener(
  "input",
  (event) => {
    textElement.textLength.baseVal.newValueSpecifiedUnits(
      SVGLength.SVG_LENGTHTYPE_PX,
      widthSlider.valueAsNumber,
    );
    widthDisplay.innerText = widthSlider.value;
  },
  false,
);

widthSlider.dispatchEvent(new Event("input"));
