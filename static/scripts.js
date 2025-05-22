// Wait for DOM to load
document.addEventListener("DOMContentLoaded", () => {
  const yasuoDiv = document.getElementById("yasuo");
  const yasuoImage = document.querySelector("img");

  // Change text on click
  yasuoDiv.addEventListener("click", () => {
    yasuoDiv.textContent = "Hasagi!";
    setTimeout(() => {
      yasuoDiv.textContent = "YASUO";
    }, 1000);
  });

  // Image interaction
  yasuoImage.addEventListener("mouseenter", () => {
    document.body.style.backgroundColor = "#e6f7ff";
  });

  yasuoImage.addEventListener("mouseleave", () => {
    document.body.style.backgroundColor = "#f0f0f0";
  });

  // Console log when image loads
  yasuoImage.addEventListener("load", () => {
    console.log("Yasuo image loaded successfully!");
  });
});
