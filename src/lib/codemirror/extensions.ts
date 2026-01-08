import { markdown } from "@codemirror/lang-markdown";
import { oneDark } from "@codemirror/theme-one-dark";
import { EditorView, keymap } from "@codemirror/view";
import { defaultKeymap, historyKeymap } from "@codemirror/commands";
import { searchKeymap } from "@codemirror/search";

export const defaultExtensions = [
  keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap]),
  EditorView.lineWrapping,
];

export const markdownExtensions = [
  markdown(),
  // Can add specific markdown highlighting here
];

export const themeExtensions = [
  oneDark, // Default theme for now, can be dynamic later
];
