type Theme = "light" | "dark";

export function getTheme(): Theme {
  const theme = localStorage.getItem("theme");
  if (theme) {
    if (theme === "dark") return "dark";
    return "light";
  }
  if (window.matchMedia("(prefers-color-scheme: dark)").matches) {
    return "dark";
  }
  return "light";
}

export function setTheme(theme: Theme) {
  if (theme === "light") {
    document.documentElement.classList.remove("dark");
  } else {
    document.documentElement.classList.add("dark");
  }
  window.localStorage.setItem("theme", theme);
}

export function toggleTheme() {
  const theme = getTheme();
  if (theme === "dark") setTheme("light");
  else setTheme("dark");
}
