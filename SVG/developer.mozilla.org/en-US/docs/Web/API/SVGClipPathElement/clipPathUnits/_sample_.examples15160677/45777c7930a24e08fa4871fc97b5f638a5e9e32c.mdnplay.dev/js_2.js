const clipPath1 = document.getElementById("clip1");
const clipPath2 = document.getElementById("clip2");

const log = document.getElementById("log");

log.textContent = `The clipPath used three times has a 'clipPathUnits' value of ${clipPath1.clipPathUnits.baseVal}
The clipPath used three times has a 'clipPathUnits' value of ${clipPath2.clipPathUnits.baseVal}`;
