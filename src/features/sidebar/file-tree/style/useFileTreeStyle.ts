import { createSignal } from "solid-js";
import {
  DEFAULT_FILE_TREE_STYLE,
  type FileTreeStyle
} from "./fileTreeStyleTypes";

export const useFileTreeStyle = (initialStyle?: FileTreeStyle) => {
  const [style, setStyle] = createSignal<FileTreeStyle>(
    initialStyle ?? DEFAULT_FILE_TREE_STYLE
  );

  const styleAttrs = () => ({
    "data-style": style()
  });

  return {
    style,
    setStyle,
    styleAttrs
  };
};
