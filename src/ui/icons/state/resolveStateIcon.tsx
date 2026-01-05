import ChevronDown from "lucide-solid/icons/chevron-down";
import ChevronRight from "lucide-solid/icons/chevron-right";
import FileText from "lucide-solid/icons/file-text";
import Folder from "lucide-solid/icons/folder";
import FolderOpen from "lucide-solid/icons/folder-open";
import type { JSX } from "solid-js";
import type { StateIconContext, StateIconState, StateIconTheme } from "./types";

type ResolveStateIconArgs = {
  context: StateIconContext;
  state: StateIconState;
  theme?: StateIconTheme;
};

const isClassic = (theme?: StateIconTheme) => theme === "classic";

export const resolveStateIcon = (args: ResolveStateIconArgs): JSX.Element => {
  const { context, state, theme } = args;

  if (context === "collapsible") {
    return state === "open"
      ? <ChevronDown class="collapsible-chevron-icon" />
      : <ChevronRight class="collapsible-chevron-icon" />;
  }

  if (state === "file") {
    return isClassic(theme)
      ? <span class="file-tree-emoji">📄</span>
      : <FileText class="file-tree-icon-svg" />;
  }

  if (state === "folder-open") {
    return isClassic(theme)
      ? <span class="file-tree-emoji">📂</span>
      : <FolderOpen class="file-tree-icon-svg" />;
  }

  return isClassic(theme)
    ? <span class="file-tree-emoji">📁</span>
    : <Folder class="file-tree-icon-svg" />;
};
