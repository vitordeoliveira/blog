// Function to apply dark mode
function applyDarkMode() {
  document.documentElement.classList.add("dark");
}

// Function to remove dark mode
function removeDarkMode() {
  document.documentElement.classList.remove("dark");
}

// Function to check and apply dark mode
function checkAndApplyDarkMode() {
  const currentTheme = localStorage.getItem("theme");
  if (
    currentTheme === "dark" ||
    (!currentTheme && window.matchMedia("(prefers-color-scheme: dark)").matches)
  ) {
    applyDarkMode();
  } else {
    removeDarkMode();
  }
}

// Event listener for changes in OS color scheme preference
window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", function (e) {
    checkAndApplyDarkMode();
  });

function toggleTheme() {
  const currentTheme = localStorage.getItem("theme");
  if (currentTheme === "dark") {
    localStorage.setItem("theme", "light");
  } else {
    localStorage.setItem("theme", "dark");
  }
  checkAndApplyDarkMode();
}

// On page load
document.addEventListener("DOMContentLoaded", function () {
  checkAndApplyDarkMode();
});

// Whenever the user explicitly chooses light mode
function setLightMode() {
  localStorage.setItem("theme", "light");
  removeDarkMode();
}

// Whenever the user explicitly chooses dark mode
function setDarkMode() {
  localStorage.setItem("theme", "dark");
  applyDarkMode();
}

// Whenever the user explicitly chooses to respect the OS preference
function respectOSTheme() {
  localStorage.removeItem("theme");
  checkAndApplyDarkMode();
}
