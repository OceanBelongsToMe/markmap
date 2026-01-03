import type { JSX } from "solid-js";

export type FileTreeNode = {
  id: string;
  name: string;
  type: "file" | "folder";
  children?: FileTreeNode[];
  icon?: FileTreeIcon;
};

export type FileTreeIcon =
  | { kind: "emoji"; value: string }
  | { kind: "image"; src: string; alt?: string }
  | { kind: "lucide"; node: JSX.Element };
