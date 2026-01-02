export type FileTreeNode = {
  id: string;
  name: string;
  type: "file" | "folder";
  children?: FileTreeNode[];
  icon?: FileTreeIcon;
};

export type FileTreeIcon =
  | { kind: "emoji"; value: string }
  | { kind: "image"; src: string; alt?: string };
