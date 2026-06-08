const translateType = [
  "unknown",
  "matrix()",
  "translate()",
  "scale()",
  "rotate()",
  "skewx()",
  "skewy()",
];

const clipPath1 = document.getElementById("clip1");

const log = document.getElementById("log");

let result = "The following transformation have been applied:\n";
for (const transform of clipPath1.transform.baseVal) {
  result += `- A transform of type '${translateType[transform.type]}' found.\n`;
}

log.textContent = result;
