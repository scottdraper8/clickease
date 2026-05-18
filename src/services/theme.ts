export type Theme = "light" | "dark";

export class ThemeManager {
  private static STORAGE_KEY = "clickease-theme";

  static init() {
    const savedTheme = localStorage.getItem(this.STORAGE_KEY) as Theme | null;
    const systemTheme: Theme = window.matchMedia("(prefers-color-scheme: dark)")
      .matches
      ? "dark"
      : "light";
    const theme = savedTheme || systemTheme;
    this.applyTheme(theme);

    // Watch for system changes if no preference saved
    window
      .matchMedia("(prefers-color-scheme: dark)")
      .addEventListener("change", (e) => {
        if (!localStorage.getItem(this.STORAGE_KEY)) {
          this.applyTheme(e.matches ? "dark" : "light");
        }
      });
  }

  static toggle() {
    const current = document.documentElement.classList.contains("dark")
      ? "dark"
      : "light";
    const next = current === "light" ? "dark" : "light";
    this.applyTheme(next);
    localStorage.setItem(this.STORAGE_KEY, next);
  }

  static applyTheme(theme: Theme) {
    if (theme === "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }

  static getCurrentTheme(): Theme {
    return document.documentElement.classList.contains("dark")
      ? "dark"
      : "light";
  }
}
