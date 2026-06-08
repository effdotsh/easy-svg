document.querySelector("text").addEventListener("copy", (evt) => {
  evt.clipboardData.setData(
    "text/plain",
    document.getSelection().toString().toUpperCase(),
  );
  evt.preventDefault();
});
