import type { MarkdownTheme } from "@/stores/markdown";

export interface ThemeMeta {
  id: MarkdownTheme;
  label: string;
  group: "light" | "dark";
  colors: { bg: string; text: string; accent: string };
}

export const THEMES: ThemeMeta[] = [
  // Light
  { id: "github", label: "GitHub", group: "light", colors: { bg: "#ffffff", text: "#1f2328", accent: "#0969da" } },
  { id: "solarized", label: "Solarized", group: "light", colors: { bg: "#fdf6e3", text: "#657b83", accent: "#268bd2" } },
  { id: "arctic", label: "Arctic", group: "light", colors: { bg: "#f0f6fc", text: "#1c2d3f", accent: "#1a73c7" } },
  // Dark
  { id: "github-dark", label: "GitHub Dark", group: "dark", colors: { bg: "#0d1117", text: "#e6edf3", accent: "#58a6ff" } },
  { id: "dracula", label: "Dracula", group: "dark", colors: { bg: "#282a36", text: "#f8f8f2", accent: "#bd93f9" } },
  { id: "nord", label: "Nord", group: "dark", colors: { bg: "#2e3440", text: "#eceff4", accent: "#88c0d0" } },
  { id: "cobalt", label: "Cobalt", group: "dark", colors: { bg: "#002240", text: "#e1e1e1", accent: "#ffc600" } },
  { id: "monokai", label: "Monokai", group: "dark", colors: { bg: "#272822", text: "#f8f8f2", accent: "#f92672" } },
  { id: "terminal", label: "Terminal", group: "dark", colors: { bg: "#0a0a0a", text: "#33ff33", accent: "#00ff00" } },
  { id: "sunset", label: "Sunset", group: "dark", colors: { bg: "#1a1118", text: "#e8d5c4", accent: "#ff7b54" } },
];

export const LIGHT_THEMES = THEMES.filter((t) => t.group === "light");
export const DARK_THEMES = THEMES.filter((t) => t.group === "dark");
